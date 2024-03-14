use std::{fmt, sync::Arc};

use cranelift_entity::EntityRef;
use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListAtomicLink};
use miden_diagnostics::{SourceSpan, Spanned};
use miden_hir::{AttributeSet, FunctionIdent, Ident, Signature, Type};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use super::*;

intrusive_adapter!(pub FunctionListAdapter = Box<Function>: Function { link: LinkedListAtomicLink });
intrusive_adapter!(pub FrozenFunctionListAdapter = Arc<Function>: Function { link: LinkedListAtomicLink });

/// This represents a function in Miden Assembly
#[derive(Spanned)]
pub struct Function {
    link: LinkedListAtomicLink,
    #[span]
    pub span: SourceSpan,
    /// The attributes associated with this function
    pub attrs: AttributeSet,
    /// The name of this function
    pub name: FunctionIdent,
    /// The type signature of this function
    pub signature: Signature,
    /// The [Region] which forms the body of this function
    pub body: Region,
    /// Locals allocated for this function
    locals: SmallVec<[Local; 1]>,
    /// The next available local index
    next_local_id: usize,
}
impl Function {
    pub fn new(name: FunctionIdent, signature: Signature) -> Self {
        Self {
            link: Default::default(),
            span: SourceSpan::UNKNOWN,
            attrs: Default::default(),
            name,
            signature,
            body: Default::default(),
            locals: Default::default(),
            next_local_id: 0,
        }
    }

    /// Returns true if this function is decorated with the `entrypoint` attribute.
    pub fn is_entrypoint(&self) -> bool {
        use miden_hir::symbols;

        self.attrs.has(&symbols::Entrypoint)
    }

    /// Return the number of arguments expected on the operand stack
    #[inline]
    pub fn arity(&self) -> usize {
        self.signature.arity()
    }

    /// Return the number of results produced by this function
    #[inline]
    pub fn num_results(&self) -> usize {
        self.signature.results.len()
    }

    /// Allocate a new local in this function, using the provided data
    ///
    /// The index of the local is returned as it's identifier
    pub fn alloc_local(&mut self, ty: Type) -> LocalId {
        let num_words = ty.size_in_words();
        let next_id = self.next_local_id;
        assert!(
            (next_id + num_words) < (u8::MAX as usize),
            "unable to allocate a local of type {}: unable to allocate enough local memory",
            &ty
        );
        let id = LocalId::new(next_id);
        self.next_local_id += num_words;
        let local = Local { id, ty };
        self.locals.push(local);
        id
    }

    /// Get the local with the given identifier
    pub fn local(&self, id: LocalId) -> &Local {
        self.locals.iter().find(|l| l.id == id).expect("invalid local id")
    }

    /// Return the locals allocated in this function as a slice
    #[inline]
    pub fn locals(&self) -> &[Local] {
        self.locals.as_slice()
    }

    /// Allocate a new code block in this function
    #[inline(always)]
    pub fn create_block(&mut self) -> BlockId {
        self.body.create_block()
    }

    /// Get a reference to a [Block] by [BlockId]
    #[inline(always)]
    pub fn block(&self, id: BlockId) -> &Block {
        self.body.block(id)
    }

    /// Get a mutable reference to a [Block] by [BlockId]
    #[inline(always)]
    pub fn block_mut(&mut self, id: BlockId) -> &mut Block {
        self.body.block_mut(id)
    }

    /// Return an implementation of [std::fmt::Display] for this function
    pub fn display<'a, 'b: 'a>(&'b self, imports: &'b ModuleImportInfo) -> DisplayMasmFunction<'a> {
        DisplayMasmFunction {
            function: self,
            imports,
        }
    }

    pub fn from_procedure_ast(
        module: Ident,
        proc: &miden_assembly::ast::ProcedureAst,
        locals: &[FunctionIdent],
        imported: &miden_assembly::ast::ModuleImports,
    ) -> Box<Self> {
        use miden_hir::{Linkage, Symbol};
        let id = FunctionIdent {
            module,
            function: Ident::with_empty_span(Symbol::intern(proc.name.as_ref())),
        };
        let mut signature = Signature::new(vec![], vec![]);
        if !proc.is_export {
            signature.linkage = Linkage::Internal;
        }
        let mut function = Box::new(Self::new(id, signature));
        if proc.name.is_main() {
            function.attrs.set(miden_hir::attributes::ENTRYPOINT);
        }
        for _ in 0..proc.num_locals {
            function.alloc_local(Type::Felt);
        }

        function.body = Region::from_code_body(&proc.body, locals, imported);

        function
    }

    pub fn to_function_ast(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        imports: &miden_hir::ModuleImportInfo,
        local_ids: &FxHashMap<FunctionIdent, u16>,
        proc_ids: &FxHashMap<FunctionIdent, miden_assembly::ProcedureId>,
    ) -> miden_assembly::ast::ProcedureAst {
        use miden_assembly::{
            self as masm,
            ast::{ProcedureAst, SourceLocation},
        };

        let name = masm::ProcedureName::try_from(self.name.function.as_str())
            .expect("invalid function name");
        let num_locals = u16::try_from(self.locals.len()).expect("too many locals");
        let start = codemap
            .location(self)
            .ok()
            .map(|loc| {
                SourceLocation::new(loc.line.to_usize() as u32, loc.column.to_usize() as u32)
            })
            .unwrap_or_default();
        let body = self.body.to_code_body(codemap, imports, local_ids, proc_ids);

        ProcedureAst {
            name,
            docs: None,
            num_locals,
            body,
            start,
            is_export: self.signature.is_public(),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("signature", &self.signature)
            .field("attrs", &self.attrs)
            .field("locals", &self.locals)
            .field("body", &self.body)
            .finish()
    }
}

#[doc(hidden)]
pub struct DisplayMasmFunction<'a> {
    function: &'a Function,
    imports: &'a ModuleImportInfo,
}
impl<'a> fmt::Display for DisplayMasmFunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let visibility = if self.function.signature.is_public() {
            "export"
        } else {
            "proc"
        };
        let name = self.function.name;
        match self.function.locals.len() {
            0 => {
                writeln!(f, "{visibility}.{}", &name.function)?;
            }
            n => {
                writeln!(f, "{visibility}.{}.{}", &name.function, n)?;
            }
        }

        writeln!(f, "{}", self.function.body.display(Some(self.function.name), self.imports, 1))?;

        f.write_str("end")
    }
}

pub type FunctionList = LinkedList<FunctionListAdapter>;
pub type FunctionListIter<'a> = intrusive_collections::linked_list::Iter<'a, FunctionListAdapter>;

pub type FrozenFunctionList = LinkedList<FrozenFunctionListAdapter>;
pub type FrozenFunctionListIter<'a> =
    intrusive_collections::linked_list::Iter<'a, FrozenFunctionListAdapter>;

pub(super) enum Functions {
    Open(FunctionList),
    Frozen(FrozenFunctionList),
}
impl Default for Functions {
    fn default() -> Self {
        Self::Open(Default::default())
    }
}
impl Functions {
    pub fn iter(&self) -> impl Iterator<Item = &Function> + '_ {
        match self {
            Self::Open(ref list) => FunctionsIter::Open(list.iter()),
            Self::Frozen(ref list) => FunctionsIter::Frozen(list.iter()),
        }
    }

    pub fn push_back(&mut self, function: Box<Function>) {
        match self {
            Self::Open(ref mut list) => {
                list.push_back(function);
            }
            Self::Frozen(_) => panic!("cannot insert function into frozen module"),
        }
    }

    pub fn freeze(&mut self) {
        if let Self::Open(ref mut functions) = self {
            let mut frozen = FrozenFunctionList::default();

            while let Some(function) = functions.pop_front() {
                frozen.push_back(Arc::from(function));
            }

            *self = Self::Frozen(frozen);
        }
    }
}

enum FunctionsIter<'a> {
    Open(FunctionListIter<'a>),
    Frozen(FrozenFunctionListIter<'a>),
}
impl<'a> Iterator for FunctionsIter<'a> {
    type Item = &'a Function;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Open(ref mut iter) => iter.next(),
            Self::Frozen(ref mut iter) => iter.next(),
        }
    }
}
