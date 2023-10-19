//! WebAssembly module and function translation state.
//!
//! The `FuncTranslationState` struct defined in this module is used to keep track of the WebAssembly
//! value and control stacks during the translation of a single function.
//!
//! Based on Cranelift's Wasm -> CLIF translator v11.0.0

use crate::{
    error::{WasmError, WasmResult},
    function_builder_ext::FunctionBuilderExt,
    module_env::ModuleInfo,
    translation_utils::sig_from_funct_type,
    wasm_types::{BlockType, FuncIndex},
};
use miden_diagnostics::{DiagnosticsHandler, SourceSpan};
use miden_hir::{
    cranelift_entity::EntityRef, Block, CallConv, DataFlowGraph, FunctionIdent, Ident, Inst,
    InstBuilder, Linkage, Signature, Symbol, Value,
};
use miden_hir_type::Type;
use std::{
    collections::{hash_map::Entry::Occupied, hash_map::Entry::Vacant, HashMap},
    vec::Vec,
};

/// Information about the presence of an associated `else` for an `if`, or the
/// lack thereof.
#[derive(Debug)]
pub enum ElseData {
    /// The `if` does not already have an `else` block.
    ///
    /// This doesn't mean that it will never have an `else`, just that we
    /// haven't seen it yet.
    NoElse {
        /// If we discover that we need an `else` block, this is the jump
        /// instruction that needs to be fixed up to point to the new `else`
        /// block rather than the destination block after the `if...end`.
        branch_inst: Inst,

        /// The placeholder block we're replacing.
        placeholder: Block,
    },

    /// We have already allocated an `else` block.
    ///
    /// Usually we don't know whether we will hit an `if .. end` or an `if
    /// .. else .. end`, but sometimes we can tell based on the block's type
    /// signature that the signature is not valid if there isn't an `else`. In
    /// these cases, we pre-allocate the `else` block.
    WithElse {
        /// This is the `else` block.
        else_block: Block,
    },
}

/// A control stack frame can be an `if`, a `block` or a `loop`, each one having the following
/// fields:
///
/// - `destination`: reference to the `Block` that will hold the code after the control block;
/// - `num_return_values`: number of values returned by the control block;
/// - `original_stack_size`: size of the value stack at the beginning of the control block.
///
/// The `loop` frame has a `header` field that references the `Block` that contains the beginning
/// of the body of the loop.
#[derive(Debug)]
pub enum ControlStackFrame {
    If {
        destination: Block,
        else_data: ElseData,
        num_param_values: usize,
        num_return_values: usize,
        original_stack_size: usize,
        exit_is_branched_to: bool,
        blocktype: BlockType,
        /// Was the head of the `if` reachable?
        head_is_reachable: bool,
        /// What was the reachability at the end of the consequent?
        ///
        /// This is `None` until we're finished translating the consequent, and
        /// is set to `Some` either by hitting an `else` when we will begin
        /// translating the alternative, or by hitting an `end` in which case
        /// there is no alternative.
        consequent_ends_reachable: Option<bool>,
        // Note: no need for `alternative_ends_reachable` because that is just
        // `state.reachable` when we hit the `end` in the `if .. else .. end`.
    },
    Block {
        destination: Block,
        num_param_values: usize,
        num_return_values: usize,
        original_stack_size: usize,
        exit_is_branched_to: bool,
    },
    Loop {
        destination: Block,
        header: Block,
        num_param_values: usize,
        num_return_values: usize,
        original_stack_size: usize,
    },
}

/// Helper methods for the control stack objects.
impl ControlStackFrame {
    pub fn num_return_values(&self) -> usize {
        match *self {
            Self::If {
                num_return_values, ..
            }
            | Self::Block {
                num_return_values, ..
            }
            | Self::Loop {
                num_return_values, ..
            } => num_return_values,
        }
    }
    pub fn num_param_values(&self) -> usize {
        match *self {
            Self::If {
                num_param_values, ..
            }
            | Self::Block {
                num_param_values, ..
            }
            | Self::Loop {
                num_param_values, ..
            } => num_param_values,
        }
    }
    pub fn following_code(&self) -> Block {
        match *self {
            Self::If { destination, .. }
            | Self::Block { destination, .. }
            | Self::Loop { destination, .. } => destination,
        }
    }
    pub fn br_destination(&self) -> Block {
        match *self {
            Self::If { destination, .. } | Self::Block { destination, .. } => destination,
            Self::Loop { header, .. } => header,
        }
    }
    /// Private helper. Use `truncate_value_stack_to_else_params()` or
    /// `truncate_value_stack_to_original_size()` to restore value-stack state.
    fn original_stack_size(&self) -> usize {
        match *self {
            Self::If {
                original_stack_size,
                ..
            }
            | Self::Block {
                original_stack_size,
                ..
            }
            | Self::Loop {
                original_stack_size,
                ..
            } => original_stack_size,
        }
    }
    pub fn is_loop(&self) -> bool {
        match *self {
            Self::If { .. } | Self::Block { .. } => false,
            Self::Loop { .. } => true,
        }
    }

    pub fn exit_is_branched_to(&self) -> bool {
        match *self {
            Self::If {
                exit_is_branched_to,
                ..
            }
            | Self::Block {
                exit_is_branched_to,
                ..
            } => exit_is_branched_to,
            Self::Loop { .. } => false,
        }
    }

    pub fn set_branched_to_exit(&mut self) {
        match *self {
            Self::If {
                ref mut exit_is_branched_to,
                ..
            }
            | Self::Block {
                ref mut exit_is_branched_to,
                ..
            } => *exit_is_branched_to = true,
            Self::Loop { .. } => {}
        }
    }

    /// Pop values from the value stack so that it is left at the
    /// input-parameters to an else-block.
    pub fn truncate_value_stack_to_else_params(&self, stack: &mut Vec<Value>) {
        debug_assert!(matches!(self, &ControlStackFrame::If { .. }));
        stack.truncate(self.original_stack_size());
    }

    /// Pop values from the value stack so that it is left at the state it was
    /// before this control-flow frame.
    pub fn truncate_value_stack_to_original_size(&self, stack: &mut Vec<Value>) {
        // The "If" frame pushes its parameters twice, so they're available to the else block
        // (see also `FuncTranslationState::push_if`).
        // Yet, the original_stack_size member accounts for them only once, so that the else
        // block can see the same number of parameters as the consequent block. As a matter of
        // fact, we need to substract an extra number of parameter values for if blocks.
        let num_duplicated_params = match self {
            &ControlStackFrame::If {
                num_param_values, ..
            } => {
                debug_assert!(num_param_values <= self.original_stack_size());
                num_param_values
            }
            _ => 0,
        };
        stack.truncate(self.original_stack_size() - num_duplicated_params);
    }
}

/// Contains information passed along during a function's translation and that records:
///
/// - The current value and control stacks.
/// - The depth of the two unreachable control blocks stacks, that are manipulated when translating
///   unreachable code;
pub struct FuncTranslationState {
    /// A stack of values corresponding to the active values in the input wasm function at this
    /// point.
    pub(crate) stack: Vec<Value>,
    /// A stack of active control flow operations at this point in the input wasm function.
    pub(crate) control_stack: Vec<ControlStackFrame>,
    /// Is the current translation state still reachable? This is false when translating operators
    /// like End, Return, or Unreachable.
    pub(crate) reachable: bool,

    // Imported and local functions that have been created by
    // `FuncEnvironment::make_direct_func()`.
    // Stores both the function reference and the number of WebAssembly arguments
    functions: HashMap<FuncIndex, (FunctionIdent, usize)>,
}

impl FuncTranslationState {
    /// Construct a new, empty, `FuncTranslationState`
    pub(crate) fn new() -> Self {
        Self {
            stack: Vec::new(),
            control_stack: Vec::new(),
            reachable: true,
            functions: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        debug_assert!(self.stack.is_empty());
        debug_assert!(self.control_stack.is_empty());
        self.reachable = true;
        self.functions.clear();
    }

    /// Initialize the state for compiling a function with the given signature.
    ///
    /// This resets the state to containing only a single block representing the whole function.
    /// The exit block is the last block in the function which will contain the return instruction.
    pub(crate) fn initialize(&mut self, sig: &Signature, exit_block: Block) {
        self.clear();
        self.push_block(exit_block, 0, sig.results().len());
    }

    /// Push a value.
    pub(crate) fn push1(&mut self, val: Value) {
        self.stack.push(val);
    }

    /// Push multiple values.
    pub(crate) fn pushn(&mut self, vals: &[Value]) {
        self.stack.extend_from_slice(vals);
    }

    /// Pop one value.
    pub(crate) fn pop1(&mut self) -> Value {
        self.stack
            .pop()
            .expect("attempted to pop a value from an empty stack")
    }

    /// Pop one value and cast it to the specified type.
    pub(crate) fn pop1_casted(
        &mut self,
        ty: Type,
        builder: &mut FunctionBuilderExt,
        span: SourceSpan,
    ) -> Value {
        let val = self
            .stack
            .pop()
            .expect("attempted to pop a value from an empty stack");
        builder.ins().cast(val, ty.clone(), span)
    }

    /// Peek at the top of the stack without popping it.
    pub(crate) fn peek1(&self) -> Value {
        *self
            .stack
            .last()
            .expect("attempted to peek at a value on an empty stack")
    }

    /// Pop two values. Return them in the order they were pushed.
    pub(crate) fn pop2(&mut self) -> (Value, Value) {
        let v2 = self.stack.pop().unwrap();
        let v1 = self.stack.pop().unwrap();
        (v1, v2)
    }

    /// Pop two values. Cast them to the specified type. Return them in the order they were pushed.
    pub(crate) fn pop2_casted(
        &mut self,
        ty: Type,
        builder: &mut FunctionBuilderExt,
        span: SourceSpan,
    ) -> (Value, Value) {
        let v2 = self.stack.pop().unwrap();
        let v1 = self.stack.pop().unwrap();
        let v1_casted = builder.ins().cast(v1, ty.clone(), span);
        let v2_casted = builder.ins().cast(v2, ty, span);
        (v1_casted, v2_casted)
    }

    /// Pop three values. Return them in the order they were pushed.
    pub(crate) fn pop3(&mut self) -> (Value, Value, Value) {
        let v3 = self.stack.pop().unwrap();
        let v2 = self.stack.pop().unwrap();
        let v1 = self.stack.pop().unwrap();
        (v1, v2, v3)
    }

    /// Helper to ensure the the stack size is at least as big as `n`; note that due to
    /// `debug_assert` this will not execute in non-optimized builds.
    #[inline]
    fn ensure_length_is_at_least(&self, n: usize) {
        debug_assert!(
            n <= self.stack.len(),
            "attempted to access {} values but stack only has {} values",
            n,
            self.stack.len()
        )
    }

    /// Pop the top `n` values on the stack.
    ///
    /// The popped values are not returned. Use `peekn` to look at them before popping.
    pub(crate) fn popn(&mut self, n: usize) {
        self.ensure_length_is_at_least(n);
        let new_len = self.stack.len() - n;
        self.stack.truncate(new_len);
    }

    /// Peek at the top `n` values on the stack in the order they were pushed.
    pub(crate) fn peekn(&self, n: usize) -> &[Value] {
        self.ensure_length_is_at_least(n);
        &self.stack[self.stack.len() - n..]
    }

    /// Peek at the top `n` values on the stack in the order they were pushed.
    pub(crate) fn peekn_mut(&mut self, n: usize) -> &mut [Value] {
        self.ensure_length_is_at_least(n);
        let len = self.stack.len();
        &mut self.stack[len - n..]
    }

    /// Push a block on the control stack.
    pub(crate) fn push_block(
        &mut self,
        following_code: Block,
        num_param_types: usize,
        num_result_types: usize,
    ) {
        debug_assert!(num_param_types <= self.stack.len());
        self.control_stack.push(ControlStackFrame::Block {
            destination: following_code,
            original_stack_size: self.stack.len() - num_param_types,
            num_param_values: num_param_types,
            num_return_values: num_result_types,
            exit_is_branched_to: false,
        });
    }

    /// Push a loop on the control stack.
    pub(crate) fn push_loop(
        &mut self,
        header: Block,
        following_code: Block,
        num_param_types: usize,
        num_result_types: usize,
    ) {
        debug_assert!(num_param_types <= self.stack.len());
        self.control_stack.push(ControlStackFrame::Loop {
            header,
            destination: following_code,
            original_stack_size: self.stack.len() - num_param_types,
            num_param_values: num_param_types,
            num_return_values: num_result_types,
        });
    }

    /// Push an if on the control stack.
    pub(crate) fn push_if(
        &mut self,
        destination: Block,
        else_data: ElseData,
        num_param_types: usize,
        num_result_types: usize,
        blocktype: BlockType,
    ) {
        debug_assert!(num_param_types <= self.stack.len());

        // Push a second copy of our `if`'s parameters on the stack. This lets
        // us avoid saving them on the side in the `ControlStackFrame` for our
        // `else` block (if it exists), which would require a second heap
        // allocation. See also the comment in `translate_operator` for
        // `Operator::Else`.
        self.stack.reserve(num_param_types);
        for i in (self.stack.len() - num_param_types)..self.stack.len() {
            let val = self.stack[i];
            self.stack.push(val);
        }

        self.control_stack.push(ControlStackFrame::If {
            destination,
            else_data,
            original_stack_size: self.stack.len() - num_param_types,
            num_param_values: num_param_types,
            num_return_values: num_result_types,
            exit_is_branched_to: false,
            head_is_reachable: self.reachable,
            consequent_ends_reachable: None,
            blocktype,
        });
    }
}

/// Methods for handling entity references.
impl FuncTranslationState {
    /// Get the `FunctionIdent` that should be used to make a direct call to function
    /// `index`. Also return the number of WebAssembly arguments in the signature.
    ///
    /// Import the callee into `func`'s DFG if it is not already present.
    pub(crate) fn get_direct_func(
        &mut self,
        dfg: &mut DataFlowGraph,
        index: u32,
        mod_info: &ModuleInfo,
        diagnostics: &DiagnosticsHandler,
    ) -> WasmResult<(FunctionIdent, usize)> {
        let index = FuncIndex::from_u32(index);
        Ok(match self.functions.entry(index) {
            Occupied(entry) => *entry.get(),
            Vacant(entry) => {
                let sigidx = mod_info.functions[index];
                let func_type = mod_info.func_types[sigidx].clone();
                let func_name = mod_info
                    .function_names
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| format!("func{}", index.index()));
                let func_name_id = Ident::with_empty_span(Symbol::intern(&func_name));
                let sig = sig_from_funct_type(&func_type, CallConv::SystemV, Linkage::External);
                let Ok(func_id) = dfg.import_function(mod_info.name, func_name_id, sig.clone())
                else {
                    let message = format!("Function with name {} in module {} with signature {sig:?} is already imported (function call) with a different signature", func_name_id, mod_info.name);
                    diagnostics
                        .diagnostic(miden_diagnostics::Severity::Error)
                        .with_message(message.clone())
                        .emit();
                    return Err(WasmError::Unexpected(message));
                };
                *entry.insert((func_id, sig.params().len()))
            }
        })
    }
}
