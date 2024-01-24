use std::fmt;

use cranelift_entity::entity_impl;
use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink};
use miden_diagnostics::Spanned;

use super::{pass::AnalysisKey, *};

/// This error is raised when two function declarations conflict with the same symbol name
#[derive(Debug, thiserror::Error)]
#[error("item with this name has already been declared, or cannot be merged")]
pub struct SymbolConflictError(pub FunctionIdent);

/// A handle that refers to an [ExternalFunction]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncRef(u32);
entity_impl!(FuncRef, "fn");

/// Represents the calling convention of a function.
///
/// Calling conventions are part of a program's ABI (Application Binary Interface), and
/// they define things such how arguments are passed to a function, how results are returned,
/// etc. In essence, the contract between caller and callee is described by the calling convention
/// of a function.
///
/// Importantly, it is perfectly normal to mix calling conventions. For example, the public
/// API for a C library will use whatever calling convention is used by C on the target
/// platform (for Miden, that would be `SystemV`). However, internally that library may use
/// the `Fast` calling convention to allow the compiler to optimize more effectively calls
/// from the public API to private functions. In short, choose a calling convention that is
/// well-suited for a given function, to the extent that other constraints don't impose a choice
/// on you.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum CallConv {
    /// This calling convention is what I like to call "chef's choice" - the
    /// compiler chooses it's own convention that optimizes for call performance.
    ///
    /// As a result of this, it is not permitted to use this convention in externally
    /// linked functions, as the convention is unstable, and the compiler can't ensure
    /// that the caller in another translation unit will use the correct convention.
    Fast,
    /// The standard calling convention used for C on most platforms
    #[default]
    SystemV,
    /// A function with this calling convention must be called using
    /// the `syscall` instruction. Attempts to call it with any other
    /// call instruction will cause a validation error. The one exception
    /// to this rule is when calling another function with the `Kernel`
    /// convention that is defined in the same module, which can use the
    /// standard `call` instruction.
    ///
    /// Kernel functions may only be defined in a kernel [Module].
    ///
    /// In all other respects, this calling convention is the same as `SystemV`
    Kernel,
}
impl fmt::Display for CallConv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Fast => f.write_str("fast"),
            Self::SystemV => f.write_str("C"),
            Self::Kernel => f.write_str("kernel"),
        }
    }
}

/// Represents whether an argument or return value has a special purpose in
/// the calling convention of a function.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ArgumentPurpose {
    /// No special purpose, the argument is passed/returned by value
    #[default]
    Default,
    /// Used for platforms where the calling convention expects return values of
    /// a certain size to be written to a pointer passed in by the caller.
    StructReturn,
}
impl fmt::Display for ArgumentPurpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Default => f.write_str("default"),
            Self::StructReturn => f.write_str("sret"),
        }
    }
}

/// Represents how to extend a small integer value to native machine integer width.
///
/// For Miden, native integrals are unsigned 64-bit field elements, but it is typically
/// going to be the case that we are targeting the subset of Miden Assembly where integrals
/// are unsigned 32-bit integers with a standard twos-complement binary representation.
///
/// It is for the latter scenario that argument extension is really relevant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ArgumentExtension {
    /// Do not perform any extension, high bits have undefined contents
    #[default]
    None,
    /// Zero-extend the value
    Zext,
    /// Sign-extend the value
    Sext,
}
impl fmt::Display for ArgumentExtension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Zext => f.write_str("zext"),
            Self::Sext => f.write_str("sext"),
        }
    }
}

/// Describes a function parameter or result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiParam {
    /// The type associated with this value
    pub ty: Type,
    /// The special purpose, if any, of this parameter or result
    pub purpose: ArgumentPurpose,
    /// The desired approach to extending the size of this value to
    /// a larger bit width, if applicable.
    pub extension: ArgumentExtension,
}
impl AbiParam {
    pub fn new(ty: Type) -> Self {
        Self {
            ty,
            purpose: ArgumentPurpose::default(),
            extension: ArgumentExtension::default(),
        }
    }

    pub fn sret(ty: Type) -> Self {
        assert!(ty.is_pointer(), "sret parameters must be pointers");
        Self {
            ty,
            purpose: ArgumentPurpose::StructReturn,
            extension: ArgumentExtension::default(),
        }
    }
}

/// A [Signature] represents the type, ABI, and linkage of a function.
///
/// A function signature provides us with all of the necessary detail to correctly
/// validate and emit code for a function, whether from the perspective of a caller,
/// or the callee.
#[derive(Debug, Clone)]
pub struct Signature {
    /// The arguments expected by this function
    pub params: Vec<AbiParam>,
    /// The results returned by this function
    pub results: Vec<AbiParam>,
    /// The calling convention that applies to this function
    pub cc: CallConv,
    /// The linkage that should be used for this function
    pub linkage: Linkage,
}
impl Signature {
    /// Create a new signature with the given parameter and result types,
    /// for a public function using the `SystemV` calling convention
    pub fn new<P: IntoIterator<Item = AbiParam>, R: IntoIterator<Item = AbiParam>>(
        params: P,
        results: R,
    ) -> Self {
        Self {
            params: params.into_iter().collect(),
            results: results.into_iter().collect(),
            cc: CallConv::SystemV,
            linkage: Linkage::External,
        }
    }

    /// Returns true if this function is externally visible
    pub fn is_public(&self) -> bool {
        matches!(self.linkage, Linkage::External)
    }

    /// Returns true if this function is only visible within it's containing module
    pub fn is_private(&self) -> bool {
        matches!(self.linkage, Linkage::Internal)
    }

    /// Returns true if this function is a kernel function
    pub fn is_kernel(&self) -> bool {
        matches!(self.cc, CallConv::Kernel)
    }

    /// Returns the number of arguments expected by this function
    pub fn arity(&self) -> usize {
        self.params().len()
    }

    /// Returns a slice containing the parameters for this function
    pub fn params(&self) -> &[AbiParam] {
        self.params.as_slice()
    }

    /// Returns the parameter at `index`, if present
    #[inline]
    pub fn param(&self, index: usize) -> Option<&AbiParam> {
        self.params.get(index)
    }

    /// Returns a slice containing the results of this function
    pub fn results(&self) -> &[AbiParam] {
        match self.results.as_slice() {
            [AbiParam { ty: Type::Unit, .. }] => &[],
            [AbiParam {
                ty: Type::Never, ..
            }] => &[],
            results => results,
        }
    }
}
impl Eq for Signature {}
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.linkage == other.linkage
            && self.cc == other.cc
            && self.params.len() == other.params.len()
            && self.results.len() == other.results.len()
    }
}

/// Represents the lifted(component) type of a component imported/exported function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftedFunctionType {
    /// The arguments expected by this function
    pub params: Vec<Type>,
    /// The results returned by this function
    pub results: Vec<Type>,
}

/// An [ExternalFunction] represents a function whose name and signature are known,
/// but which may or may not be compiled as part of the current translation unit.
///
/// When building a [Function], we use [ExternalFunction] to represent references to
/// other functions in the program which are called from its body. One "imports" a
/// function to make it callable.
///
/// At link time, we make sure all external function references are either defined in
/// the current program, or are well-known functions that are provided as part of a kernel
/// or standard library in the Miden VM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalFunction {
    pub id: FunctionIdent,
    pub signature: Signature,
}
impl Ord for ExternalFunction {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for ExternalFunction {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

intrusive_adapter!(pub FunctionListAdapter = Box<Function>: Function { link: LinkedListLink });

/// A type alias for `LinkedList<FunctionListAdapter>`
pub type FunctionList = LinkedList<FunctionListAdapter>;

/// [Function] corresponds to a function definition, in single-static assignment (SSA) form.
///
/// * Functions may have zero or more parameters, and produce zero or more results.
/// * Functions are namespaced in [Module]s. You may define a function separately from a module,
/// to aid in parallelizing compilation, but functions must be attached to a module prior to code
/// generation. Furthermore, in order to reference other functions, you must do so using their
/// fully-qualified names.
/// * Functions consist of one or more basic blocks, where the entry block is predefined based
/// on the function signature.
/// * Basic blocks consist of a sequence of [Instruction] without any control flow (excluding calls),
/// terminating with a control flow instruction. Our SSA representation uses block arguments rather
/// than phi nodes to represent join points in the control flow graph.
/// * Instructions consume zero or more arguments, and produce zero or more results. Results produced
/// by an instruction constitute definitions of those values. A value may only ever have a single
/// definition, e.g. you can't reassign a value after it is introduced by an instruction.
///
/// References to functions and global variables from a [Function] are not fully validated until
/// link-time/code generation.
#[derive(Spanned, AnalysisKey)]
pub struct Function {
    link: LinkedListLink,
    #[span]
    #[analysis_key]
    pub id: FunctionIdent,
    pub signature: Signature,
    pub dfg: DataFlowGraph,
}
impl Function {
    /// Create a new [Function] with the given name, signature, and source location.
    ///
    /// The resulting function will be given default internal linkage, i.e. it will only
    /// be visible within it's containing [Module].
    pub fn new(id: FunctionIdent, signature: Signature) -> Self {
        let mut dfg = DataFlowGraph::default();
        let entry = dfg.entry_block();
        for param in signature.params() {
            dfg.append_block_param(entry, param.ty.clone(), id.span());
        }
        dfg.imports.insert(
            id,
            ExternalFunction {
                id,
                signature: signature.clone(),
            },
        );
        Self {
            link: Default::default(),
            id,
            signature,
            dfg,
        }
    }

    /// This function is like [Function::new], except it does not initialize the
    /// function entry block using the provided [Signature]. Instead, it is expected
    /// that the caller does this manually.
    ///
    /// This is primarily intended for use by the IR parser.
    pub(crate) fn new_uninit(id: FunctionIdent, signature: Signature) -> Self {
        let mut dfg = DataFlowGraph::new_uninit();
        dfg.imports.insert(
            id,
            ExternalFunction {
                id,
                signature: signature.clone(),
            },
        );
        Self {
            link: Default::default(),
            id,
            signature,
            dfg,
        }
    }

    /// Returns true if this function has yet to be attached to a [Module]
    pub fn is_detached(&self) -> bool {
        !self.link.is_linked()
    }

    /// Returns true if this function is a kernel function
    pub fn is_kernel(&self) -> bool {
        self.signature.is_kernel()
    }

    /// Returns true if this function has external linkage
    pub fn is_public(&self) -> bool {
        self.signature.is_public()
    }

    /// Return the [Signature] for this function
    #[inline]
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Return the [Signature] for this function
    #[inline]
    pub fn signature_mut(&mut self) -> &mut Signature {
        &mut self.signature
    }

    /// Return the number of parameters this function expects
    pub fn arity(&self) -> usize {
        self.signature.arity()
    }

    /// Return the [Linkage] type for this function
    pub fn linkage(&self) -> Linkage {
        self.signature.linkage
    }

    /// Set the linkage type for this function
    pub fn set_linkage(&mut self, linkage: Linkage) {
        self.signature.linkage = linkage;
    }

    /// Return the [CallConv] type for this function
    pub fn calling_convention(&self) -> CallConv {
        self.signature.cc
    }

    /// Set the linkage type for this function
    pub fn set_calling_convention(&mut self, cc: CallConv) {
        self.signature.cc = cc;
    }

    /// Return true if this function has attribute `name`
    pub fn has_attribute<Q>(&self, name: &Q) -> bool
    where
        Q: Ord + ?Sized,
        Symbol: std::borrow::Borrow<Q>,
    {
        self.dfg.has_attribute(name)
    }

    /// Iterate over all of the external functions imported by this function
    pub fn imports<'a, 'b: 'a>(&'b self) -> impl Iterator<Item = &'a ExternalFunction> + 'a {
        self.dfg.imports().filter(|ext| ext.id != self.id)
    }

    pub fn builder(&mut self) -> FunctionBuilder {
        FunctionBuilder::new(self)
    }
}
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Function")
            .field("id", &self.id)
            .field("signature", &self.signature)
            .finish()
    }
}
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        crate::write_function(f, self)
    }
}
impl Eq for Function {}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        let is_eq = self.id == other.id && self.signature == other.signature;
        if !is_eq {
            return false;
        }

        // We expect the entry block to be the same
        if self.dfg.entry != other.dfg.entry {
            return false;
        }

        // We expect the blocks to be laid out in the same order, and to have the same parameter lists
        for (block_id, block) in self.dfg.blocks() {
            if let Some(other_block) = other.dfg.blocks.get(block_id) {
                if block.params.as_slice(&self.dfg.value_lists)
                    != other_block.params.as_slice(&other.dfg.value_lists)
                {
                    return false;
                }
                // We expect the instructions in each block to be the same
                if !block
                    .insts
                    .iter()
                    .map(|i| InstructionWithValueListPool {
                        inst: i,
                        value_lists: &self.dfg.value_lists,
                    })
                    .eq(other_block
                        .insts
                        .iter()
                        .map(|i| InstructionWithValueListPool {
                            inst: i,
                            value_lists: &other.dfg.value_lists,
                        }))
                {
                    return false;
                }
            } else {
                return false;
            }
        }

        // We expect both functions to have the same imports
        self.dfg.imports == other.dfg.imports
    }
}
