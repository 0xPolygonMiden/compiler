use std::{collections::BTreeSet, fmt, sync::Arc};

use cranelift_entity::EntityRef;
use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListAtomicLink};
use miden_assembly::ast;
use miden_diagnostics::{SourceSpan, Spanned};
use miden_hir::{formatter::PrettyPrint, AttributeSet, FunctionIdent, Ident, Signature, Type};
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
    /// The set of procedures invoked from the body of this function
    invoked: BTreeSet<ast::Invoke>,
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
            invoked: Default::default(),
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

    /// Get a reference to the entry block for this function
    pub fn body(&self) -> &Block {
        self.body.block(self.body.body)
    }

    /// Get a mutable reference to the entry block for this function
    pub fn body_mut(&mut self) -> &mut Block {
        self.body.block_mut(self.body.body)
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

    pub fn invoked(&self) -> impl Iterator<Item = &ast::Invoke> + '_ {
        self.invoked.iter()
    }

    pub fn register_invoked(&mut self, kind: ast::InvokeKind, target: ast::InvocationTarget) {
        self.invoked.insert(ast::Invoke { kind, target });
    }

    #[inline(never)]
    pub fn register_absolute_invocation_target(
        &mut self,
        kind: ast::InvokeKind,
        target: FunctionIdent,
    ) {
        let path = miden_assembly::LibraryPath::new(target.module.as_str())
            .expect("invalid procedure path");
        let name_span = miden_assembly::SourceSpan::new(
            target.function.span.start_index().0..target.function.span.end_index().0,
        );
        let id = ast::Ident::new_unchecked(miden_assembly::Span::new(
            name_span,
            Arc::from(target.function.as_str().to_string().into_boxed_str()),
        ));
        let name = ast::ProcedureName::new_unchecked(id);
        self.register_invoked(kind, ast::InvocationTarget::AbsoluteProcedurePath { name, path });
    }

    /// Return an implementation of [std::fmt::Display] for this function
    pub fn display<'a, 'b: 'a>(&'b self, imports: &'b ModuleImportInfo) -> DisplayMasmFunction<'a> {
        DisplayMasmFunction {
            function: self,
            imports,
        }
    }

    pub fn from_ast(module: Ident, proc: &ast::Procedure) -> Box<Self> {
        use miden_hir::{Linkage, Symbol};

        let id = FunctionIdent {
            module,
            function: Ident::with_empty_span(Symbol::intern(AsRef::<str>::as_ref(proc.name()))),
        };

        let mut signature = Signature::new(vec![], vec![]);
        let visibility = proc.visibility();
        if !visibility.is_exported() {
            signature.linkage = Linkage::Internal;
        } else if visibility.is_syscall() {
            signature.cc = miden_hir::CallConv::Kernel;
        }

        let mut function = Box::new(Self::new(id, signature));
        if proc.is_entrypoint() {
            function.attrs.set(miden_hir::attributes::ENTRYPOINT);
        }

        for _ in 0..proc.num_locals() {
            function.alloc_local(Type::Felt);
        }

        function.invoked.extend(proc.invoked().cloned());
        function.body = Region::from_block(module, &proc.body());

        function
    }

    pub fn to_ast(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        imports: &miden_hir::ModuleImportInfo,
        locals: &BTreeSet<FunctionIdent>,
    ) -> ast::Procedure {
        let visibility = if self.signature.is_kernel() {
            ast::Visibility::Syscall
        } else if self.signature.is_public() {
            ast::Visibility::Public
        } else {
            ast::Visibility::Private
        };
        let source_id = self.span.source_id();
        let span =
            miden_assembly::SourceSpan::new(self.span.start_index().0..self.span.end_index().0);
        let source_file = codemap.get(source_id).ok().map(|sf| {
            let nf = miden_assembly::diagnostics::SourceFile::new(
                sf.name().as_str().unwrap(),
                sf.source().to_string(),
            );
            Arc::new(nf)
        });

        let name_span = miden_assembly::SourceSpan::new(
            self.name.function.span.start_index().0..self.name.function.span.end_index().0,
        );
        let id = ast::Ident::new_unchecked(miden_assembly::Span::new(
            name_span,
            Arc::from(self.name.function.as_str().to_string().into_boxed_str()),
        ));
        let name = ast::ProcedureName::new_unchecked(id);

        let body = self.body.to_block(codemap, imports, locals);

        let num_locals = u16::try_from(self.locals.len()).expect("too many locals");
        let mut proc = ast::Procedure::new(span, visibility, name, num_locals, body)
            .with_source_file(source_file);
        proc.extend_invoked(self.invoked().cloned());
        proc
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
impl<'a> miden_hir::formatter::PrettyPrint for DisplayMasmFunction<'a> {
    fn render(&self) -> miden_hir::formatter::Document {
        use miden_hir::formatter::*;

        let visibility = if self.function.signature.is_kernel() {
            ast::Visibility::Syscall
        } else if self.function.signature.is_public() {
            ast::Visibility::Public
        } else {
            ast::Visibility::Private
        };
        let mut doc = display(visibility) + const_text(".") + display(self.function.name);
        if self.function.locals.len() > 0 {
            doc += const_text(".") + display(self.function.locals.len());
        }

        let body = self.function.body.display(Some(self.function.name), self.imports);
        doc + indent(4, nl() + body.render()) + nl() + const_text("end") + nl() + nl()
    }
}
impl<'a> fmt::Display for DisplayMasmFunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_print(f)
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
