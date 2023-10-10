use core::fmt;

use intrusive_collections::LinkedList;
use miden_hir::{FunctionIdent, Ident};

use super::{Function, FunctionListAdapter, ModuleImportInfo, Op};

pub struct Module {
    /// The name of this module, e.g. `std::math::u64`
    pub name: Ident,
    /// If this module contains a program entrypoint, this is the
    /// function identifier which should be used for that purpose.
    pub entry: Option<FunctionIdent>,
    /// The modules to import, along with their local aliases
    pub imports: ModuleImportInfo,
    /// The functions defined in this module
    pub functions: LinkedList<FunctionListAdapter>,
}

impl Module {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            entry: None,
            imports: Default::default(),
            functions: Default::default(),
        }
    }

    /// Write this module as Miden Assembly text to `out`
    pub fn emit(&self, _out: &mut dyn std::io::Write) -> std::io::Result<()> {
        todo!()
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

        for function in self.functions.iter() {
            writeln!(f, "{}\n", function.display(&self.imports))?;
        }

        if let Some(entry) = self.entry {
            f.write_str("begin\n")?;
            writeln!(f, "    exec.{}", entry.function)?;
            f.write_str("end\n")?;
        }

        Ok(())
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
                Op::PushU8(3),
                Op::Lte,
                Op::Assert,
                // compute a set of three booleans which used in conjunction with cdrop will
                // extract the desired element of the given word
                // # [element_index, w0, ..w3, element_index < 3]
                Op::Dup(0),
                Op::PushU8(3),
                Op::Lt,
                Op::Movdn(6),
                // # [element_index, w0, ..w3, element_index < 2, ..]
                Op::Dup(0),
                Op::PushU8(2),
                Op::Lt,
                Op::Movdn(6),
                // # [element_index < 1, w0, ..w3, ..]
                Op::PushU8(1),
                Op::Lt,
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
}
