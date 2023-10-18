use std::{collections::BTreeMap, fmt, path::Path, sync::Arc};

use intrusive_collections::{intrusive_adapter, LinkedList, RBTreeAtomicLink};
use miden_hir::{FunctionIdent, Ident};
use rustc_hash::FxHashMap;

use super::{FrozenFunctionListAdapter, Function, FunctionListAdapter, ModuleImportInfo, Op};

const I32_INTRINSICS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/i32.masm"));
const MEM_INTRINSICS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/mem.masm"));

/// This represents a single compiled Miden Assembly module in a form that is
/// designed to integrate well with the rest of our IR. You can think of this
/// as an intermediate representation corresponding to the Miden Assembly AST,
/// i.e. [miden_assembly::ast::ModuleAst].
pub struct Module {
    link: RBTreeAtomicLink,
    /// The name of this module, e.g. `std::math::u64`
    pub name: Ident,
    /// The module-scoped documentation for this module
    pub docs: Option<String>,
    /// If this module contains a program entrypoint, this is the
    /// function identifier which should be used for that purpose.
    pub entry: Option<FunctionIdent>,
    /// The modules to import, along with their local aliases
    pub imports: ModuleImportInfo,
    /// The functions defined in this module
    functions: FunctionList,
}
impl Module {
    /// Create a new, empty [Module] with the given name.
    pub fn new(name: Ident) -> Self {
        Self {
            link: Default::default(),
            name,
            docs: None,
            entry: None,
            imports: Default::default(),
            functions: Default::default(),
        }
    }

    /// Freezes this program, preventing further modifications
    pub fn freeze(mut self: Box<Self>) -> Arc<Module> {
        self.functions.freeze();
        Arc::from(self)
    }

    /// Get an iterator over the functions in this module
    pub fn functions(&self) -> impl Iterator<Item = &Function> + '_ {
        self.functions.iter()
    }

    /// Access the frozen functions list of this module, and panic if not frozen
    pub fn unwrap_frozen_functions(&self) -> &LinkedList<FrozenFunctionListAdapter> {
        match self.functions {
            FunctionList::Frozen(ref functions) => functions,
            FunctionList::Open(_) => panic!("expected module to be frozen"),
        }
    }

    /// Append a function to the end of this module
    ///
    /// NOTE: This function will panic if the module has been frozen
    pub fn push_back(&mut self, function: Box<Function>) {
        self.functions.push_back(function);
    }

    /// Convert this module into its [miden_assembly::Module] representation.
    pub fn to_module_ast(&self, codemap: &miden_diagnostics::CodeMap) -> miden_assembly::Module {
        use miden_assembly::{
            self as masm,
            ast::{ModuleAst, ModuleImports},
        };

        // Create module import table
        let mut imported = BTreeMap::<String, masm::LibraryPath>::default();
        let mut invoked = BTreeMap::<masm::ProcedureId, _>::default();
        let mut proc_ids = FxHashMap::<FunctionIdent, masm::ProcedureId>::default();
        for import in self.imports.iter() {
            let path = masm::LibraryPath::new(import.name.as_str()).expect("invalid module name");
            imported.insert(import.alias.to_string(), path.clone());
            if let Some(imported_fns) = self.imports.imported(&import.alias) {
                for import_fn in imported_fns.iter().copied() {
                    let fname = import_fn.to_string();
                    let name = masm::ProcedureName::try_from(fname.as_str())
                        .expect("invalid function name");
                    let id = masm::ProcedureId::from_name(fname.as_str(), &path);
                    invoked.insert(id, (name, path.clone()));
                    proc_ids.insert(import_fn, id);
                }
            }
        }
        let imports = ModuleImports::new(imported, invoked);

        // Translate functions
        let mut local_ids = FxHashMap::default();
        for (id, function) in self.functions.iter().enumerate() {
            local_ids.insert(function.name, id as u16);
        }
        let mut procs = Vec::with_capacity(self.num_imported_functions());
        for function in self.functions.iter() {
            procs.push(function.to_function_ast(codemap, &self.imports, &local_ids, &proc_ids));
        }

        // Construct module
        let path = masm::LibraryPath::new(self.name.as_str()).expect("invalid module name");
        let ast = ModuleAst::new(procs, vec![], self.docs.clone())
            .expect("invalid module body")
            .with_import_info(imports);
        masm::Module { path, ast }
    }

    fn num_imported_functions(&self) -> usize {
        self.imports
            .iter()
            .map(|i| {
                self.imports
                    .imported(&i.alias)
                    .map(|imported| imported.len())
                    .unwrap_or(0)
            })
            .sum()
    }

    /// Write this module to a new file under `dir`, assuming `dir` is the root directory for a program.
    ///
    /// For example, if this module is named `std::math::u64`, then it will be written to `<dir>/std/math/u64.masm`
    pub fn write_to_directory<P: AsRef<Path>>(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        dir: P,
    ) -> std::io::Result<()> {
        use std::fs::File;

        let mut path = dir.as_ref().to_path_buf();
        assert!(path.is_dir());
        for component in self.name.as_str().split("::") {
            path.push(component);
        }
        assert!(path.set_extension("masm"));

        let mut out = File::create(&path)?;
        self.emit(codemap, &mut out)
    }

    /// Write this module as Miden Assembly text to `out`
    pub fn emit(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        let ast = self.to_module_ast(codemap);
        out.write_fmt(format_args!("{}", &ast.ast))
    }
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for import in self.imports.iter() {
            if import.is_aliased() {
                writeln!(f, "use.{}->{}", import.name, import.alias)?;
            } else {
                writeln!(f, "use.{}", import.name)?;
            }
        }

        if !self.imports.is_empty() {
            writeln!(f)?;
        }

        for (i, function) in self.functions.iter().enumerate() {
            if i > 0 {
                writeln!(f, "\n{}", function.display(&self.imports))?;
            } else {
                writeln!(f, "{}", function.display(&self.imports))?;
            }
        }

        if let Some(entry) = self.entry {
            f.write_str("begin\n")?;
            writeln!(f, "    exec.{}", entry.function)?;
            f.write_str("end\n")?;
        }

        Ok(())
    }
}
impl midenc_session::Emit for Module {
    fn name(&self) -> Option<miden_hir::Symbol> {
        Some(self.name.as_symbol())
    }
    fn output_type(&self) -> midenc_session::OutputType {
        midenc_session::OutputType::Masm
    }
    fn write_to<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_fmt(format_args!("{}", self))
    }
}

intrusive_adapter!(pub ModuleTreeAdapter = Box<Module>: Module { link: RBTreeAtomicLink });
intrusive_adapter!(pub FrozenModuleTreeAdapter = Arc<Module>: Module { link: RBTreeAtomicLink });
impl<'a> intrusive_collections::KeyAdapter<'a> for ModuleTreeAdapter {
    type Key = Ident;

    #[inline]
    fn get_key(&self, module: &'a Module) -> Ident {
        module.name
    }
}
impl<'a> intrusive_collections::KeyAdapter<'a> for FrozenModuleTreeAdapter {
    type Key = Ident;

    #[inline]
    fn get_key(&self, module: &'a Module) -> Ident {
        module.name
    }
}

enum FunctionList {
    Open(LinkedList<FunctionListAdapter>),
    Frozen(LinkedList<FrozenFunctionListAdapter>),
}
impl Default for FunctionList {
    fn default() -> Self {
        Self::Open(Default::default())
    }
}
impl FunctionList {
    pub fn iter(&self) -> FunctionListIter<'_> {
        match self {
            Self::Open(ref list) => FunctionListIter::Open(list.iter()),
            Self::Frozen(ref list) => FunctionListIter::Frozen(list.iter()),
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

    fn freeze(&mut self) {
        if let Self::Open(ref mut functions) = self {
            let mut frozen = LinkedList::<FrozenFunctionListAdapter>::default();

            while let Some(function) = functions.pop_front() {
                frozen.push_back(Arc::from(function));
            }

            *self = Self::Frozen(frozen);
        }
    }
}

enum FunctionListIter<'a> {
    Open(intrusive_collections::linked_list::Iter<'a, FunctionListAdapter>),
    Frozen(intrusive_collections::linked_list::Iter<'a, FrozenFunctionListAdapter>),
}
impl<'a> Iterator for FunctionListIter<'a> {
    type Item = &'a Function;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Open(ref mut iter) => iter.next(),
            Self::Frozen(ref mut iter) => iter.next(),
        }
    }
}

impl Module {
    /// This is a helper that constructs and returns the predefined `intrinsics::mem` module
    pub fn mem_intrinsics() -> Self {
        use miden_hir::{AbiParam, Felt, FieldElement, Linkage, Signature, Type};

        let mut module = Module::new("intrinsics::mem".parse().unwrap());

        // # Given an element index, and a word, in that order, drop the elements of the
        // # word other than the at the specified index.
        // #
        // # The element index must be in the range 0..=3.
        // export.extract_element # [element_index, w0, w1, w2, w3]

        let extract_element = "intrinsics::mem::extract_element".parse().unwrap();
        let mut f = Box::new(Function::new(
            extract_element,
            Signature::new(
                [
                    AbiParam::new(Type::U8),
                    AbiParam::new(Type::Felt),
                    AbiParam::new(Type::Felt),
                    AbiParam::new(Type::Felt),
                    AbiParam::new(Type::Felt),
                ],
                [AbiParam::new(Type::Felt)],
            ),
        ));

        {
            let body = f.block_mut(f.body);
            body.extend_from_slice(&[
                // assert the index given is valid
                Op::Dup(0),
                Op::LteImm(Felt::new(3)),
                Op::Assert,
                // compute a set of three booleans which used in conjunction with cdrop will
                // extract the desired element of the given word
                // # [element_index, w0, ..w3, element_index < 3]
                Op::Dup(0),
                Op::LtImm(Felt::new(3)),
                Op::Movdn(5),
                // # [element_index, w0, ..w3, element_index < 2, ..]
                Op::Dup(0),
                Op::LtImm(Felt::new(2)),
                Op::Movdn(5),
                // # [element_index < 1, w0, ..w3, ..]
                Op::LtImm(Felt::ONE),
                // # drop w1 if the element index is zero; or drop w0 if the element index is non-zero
                Op::Cdrop,
                // # drop w2 if the element index is one; or drop w0 and w1 if the element index is > 1
                Op::Movup(3),
                Op::Cdrop,
                // # drop w3 if the element index is two; or drop w0, w1, and w2 if the element index is 3
                // #
                // # after this point, the only value on the operand stack remaining will be
                // # the element of the word indicated by the index that was on the top of the
                // # stack on entry. We've consumed the word itself, as well as the element
                // # index
                Op::Movup(2),
                Op::Cdrop,
            ]);
        }

        module.functions.push_back(f);

        // # See `load_felt` for safe usage
        // proc.load_felt_unchecked # [waddr, index]
        let load_felt_unchecked = "intrinsics::mem::load_felt_unchecked".parse().unwrap();
        let mut signature = Signature::new(
            [AbiParam::new(Type::U32), AbiParam::new(Type::U8)],
            [AbiParam::new(Type::Felt)],
        );
        signature.linkage = Linkage::Internal;
        let mut f = Box::new(Function::new(load_felt_unchecked, signature));

        {
            let body = f.block_mut(f.body);
            body.extend_from_slice(&[
                // Prepare the stack to receive the loaded word
                // # [waddr, 0, 0, 0, 0, index]
                Op::Padw,
                Op::Movup(4),
                // # load the word which contains the desired element
                // # [w0, w1, w2, w3, index]
                Op::MemLoadw,
                // # select the desired element
                Op::Movup(4),
                Op::Exec(extract_element),
            ]);
        }

        module.functions.push_back(f);

        // # Load a field element from the given native pointer triplet.
        // #
        // # A native pointer triplet consists of a word address which contains the
        // # start of the data; an element index, which indicates which element of
        // # the word the data starts in; and a byte offset, which indicates which
        // # byte is the start of the data.
        // #
        // # A field element must be naturally aligned, i.e. it's byte offset must be zero.
        // export.load_felt # [waddr, index, offset]
        let load_felt = "intrinsics::mem::load_felt".parse().unwrap();
        let mut f = Box::new(Function::new(
            load_felt,
            Signature::new(
                [
                    AbiParam::new(Type::U32),
                    AbiParam::new(Type::U8),
                    AbiParam::new(Type::U8),
                ],
                [AbiParam::new(Type::Felt)],
            ),
        ));
        {
            let body = f.block_mut(f.body);
            body.extend_from_slice(&[
                // # assert the pointer is felt-aligned, then load
                Op::Movup(2),
                Op::Assertz,
                Op::Exec(load_felt_unchecked),
            ]);
        }

        module.functions.push_back(f);

        // # Load a single 32-bit machine word from the given native pointer triplet.
        // #
        // # A native pointer triplet consists of a word address which contains the
        // # start of the data; an element index, which indicates which element of
        // # the word the data starts in; and a byte offset, which indicates which
        // # byte is the start of the data.
        // export.load_sw # [waddr, index, offset]
        let load_sw = "intrinsics::mem::load_sw".parse().unwrap();
        let mut f = Box::new(Function::new(
            load_sw,
            Signature::new(
                [
                    AbiParam::new(Type::U32),
                    AbiParam::new(Type::U8),
                    AbiParam::new(Type::U8),
                ],
                [AbiParam::new(Type::Felt)],
            ),
        ));
        {
            let is_aligned = f.create_block();
            let is_unaligned = f.create_block();
            let is_first_element = f.create_block();
            let is_not_first_element = f.create_block();
            let is_second_element = f.create_block();
            let is_not_second_element = f.create_block();
            let is_third_element = f.create_block();
            let is_fourth_element = f.create_block();
            let body = f.block_mut(f.body);
            body.extend_from_slice(&[
                // # check for alignment and offset validity
                Op::Dup(2),
                Op::EqImm(Felt::ZERO),
                Op::Dup(3),
                Op::PushU8(8),
                Op::U32CheckedLt,
                // # offset must be < 8
                Op::Assert,
                // # if the pointer is naturally aligned..
                Op::If(is_aligned, is_unaligned),
            ]);

            let is_aligned = f.block_mut(is_aligned);
            is_aligned.extend_from_slice(&[
                // # drop the byte offset
                Op::Movup(2),
                Op::Drop,
                // # load the element containing the data we want
                Op::Exec(load_felt_unchecked),
            ]);

            let is_unaligned = f.block_mut(is_unaligned);
            is_unaligned.extend_from_slice(&[
                // # check if the load starts in the first element
                Op::Dup(1),
                Op::EqImm(Felt::ZERO),
                Op::If(is_first_element, is_not_first_element),
            ]);

            let is_first_element = f.block_mut(is_first_element);
            is_first_element.extend_from_slice(&[
                // # the load is across both the first and second elements
                // # drop the element index
                Op::Swap(1),
                Op::Drop,
                // # load a word
                // # [w0, w1, w2, w3, offset]
                Op::Padw,
                Op::Movup(4),
                Op::MemLoadw,
                // # drop the unused elements
                Op::Movup(3),
                Op::Movup(3),
                Op::Drop,
                Op::Drop,
                // # shift high bits left by the offset
                // # [hi, w1, offset]
                Op::Dup(2),
                Op::U32CheckedShl,
                // # move the low bits to the top and shift them as well
                // # [offset, 32, w1, hi]
                Op::Swap(1),
                Op::PushU8(32),
                Op::Movup(3),
                // # [32 - offset, w1, hi]
                Op::U32CheckedSub,
                // # [lo, hi]
                Op::U32CheckedShr,
                // # combine the two halves
                // # [result]
                Op::U32Or,
            ]);

            let is_not_first_element = f.block_mut(is_not_first_element);
            is_not_first_element.extend_from_slice(&[
                // # check if the load starts in the second element
                Op::Dup(1),
                Op::EqImm(Felt::ONE),
                Op::If(is_second_element, is_not_second_element),
            ]);

            let is_second_element = f.block_mut(is_second_element);
            is_second_element.extend_from_slice(&[
                // # the load is across both the second and third elements
                // # drop the element idnex
                Op::Swap(1),
                Op::Drop,
                // # load
                // # [w0, w1, w2, w3, offset]
                Op::Padw,
                Op::Movup(4),
                Op::MemLoadw,
                // # drop the unused elements
                // # [w1, w2, offset]
                Op::Drop,
                Op::Movdn(2),
                Op::Movdn(2),
                Op::Drop,
                // # shift the high bits
                // # [hi, w2, offset]
                Op::Dup(2),
                Op::U32CheckedShl,
                // # shift the low bits
                // # [offset, 32, w2, hi]
                Op::Swap(1),
                Op::PushU8(32),
                Op::Movup(3),
                // # [32 - offset, w2, hi]
                Op::U32CheckedSub,
                // # [lo, hi]
                Op::U32CheckedShr,
                // # combine the two halves
                // # [result]
                Op::U32Or,
            ]);

            let is_not_second_element = f.block_mut(is_not_second_element);
            is_not_second_element.extend_from_slice(&[
                // # check if the load starts in the third element
                Op::Swap(1),
                Op::EqImm(Felt::new(2)),
                Op::If(is_third_element, is_fourth_element),
            ]);

            let is_third_element = f.block_mut(is_third_element);
            is_third_element.extend_from_slice(&[
                // # the load is across both the third and fourth elements
                // # [w0, w1, w2, w3, offset]
                Op::Padw,
                Op::Movup(4),
                Op::MemLoadw,
                // # drop first two unused
                // # [w2, w3, offset]
                Op::Drop,
                Op::Drop,
                // # shift the high bits
                // # [hi, w3, offset]
                Op::Dup(2),
                Op::U32CheckedShl,
                // # shift the low bits
                // # [offset, 32, w3, hi]
                Op::Swap(1),
                Op::PushU8(32),
                Op::Movup(3),
                // # [32 - offset, w3, hi]
                Op::U32CheckedSub,
                // # [lo, hi]
                Op::U32CheckedShr,
                // # combine the two halves
                // # [result]
                Op::U32Or,
            ]);

            let is_fourth_element = f.block_mut(is_fourth_element);
            is_fourth_element.extend_from_slice(&[
                // # the load crosses a word boundary
                // # start with the word containing the low bits
                // # [waddr, waddr, offset]
                Op::Dup(0),
                // # [waddr + 1, waddr, offset]
                Op::U32CheckedAddImm(1),
                // # load the word and drop the unused elements
                // # [w0, waddr, offset]
                Op::Padw,
                Op::Movup(4),
                Op::MemLoadw,
                Op::Movdn(4),
                Op::Drop,
                Op::Drop,
                Op::Drop,
                // # shift the low bits
                // # [offset, 32, w0, waddr, offset]
                Op::PushU8(32),
                Op::Dup(3),
                // # [32 - offset, w0, waddr, offset]
                Op::U32CheckedSub,
                // # [lo, waddr, offset]
                Op::U32CheckedShr,
                // # load the word with the high bits, drop unused elements
                // # [w3, lo, offset]
                Op::Swap(1),
                Op::Padw,
                Op::Movup(4),
                Op::MemLoadw,
                Op::Drop,
                Op::Drop,
                Op::Drop,
                // # shift high bits
                // # [hi, lo]
                Op::Movup(2),
                Op::U32CheckedShl,
                // # combine the two halves
                // # [result]
                Op::U32Or,
            ]);
        }

        module.functions.push_back(f);

        module
    }

    /// This is a helper that parses and returns the predefined `intrinsics::i32` module
    pub fn i32_intrinsics() -> Self {
        Self::parse_str(I32_INTRINSICS, "intrinsics::i32").expect("invalid module")
    }
}
