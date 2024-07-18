use midenc_hir::{self as hir, Felt, FieldElement, StructType, Type};

use super::OpEmitter;
use crate::masm::{NativePtr, Op};

const PAGE_SIZE: u32 = 64 * 1024;

/// Allocation
impl<'a> OpEmitter<'a> {
    /// Allocate a procedure-local memory slot of sufficient size to store a value
    /// indicated by the given pointer type, i.e the pointee type dictates the
    /// amount of memory allocated.
    ///
    /// The address of that slot is placed on the operand stack.
    pub fn alloca(&mut self, ptr: &Type) {
        match ptr {
            Type::Ptr(pointee) => {
                let local = self.function.alloc_local(pointee.as_ref().clone());
                self.emit(Op::LocAddr(local));
                self.stack.push(ptr.clone());
            }
            ty => panic!("expected a pointer type, got {ty}"),
        }
    }

    /// TODO(pauls): For now, we simply return -1 as if the heap cannot be grown any further
    pub fn mem_grow(&mut self) {
        let _size = self.stack.pop().expect("operand stack is empty");
        self.emit(Op::PushU32(-1i32 as u32));
        self.stack.push(Type::I32);
    }

    /// TODO(pauls): For now, we simply return u32::MAX as if the heap is already fully grown
    pub fn mem_size(&mut self) {
        const MAX_HEAP_PAGES: u32 = u32::MAX / PAGE_SIZE;
        self.emit(Op::PushU32(MAX_HEAP_PAGES));
        self.stack.push(Type::U32);
    }
}

/// Loads
impl<'a> OpEmitter<'a> {
    /// Load a value corresponding to the type of the given local, from the memory allocated for
    /// that local.
    ///
    /// Internally, this pushes the address of the local on the stack, then delegates to
    /// [OpEmitter::load]
    pub fn load_local(&mut self, local: hir::LocalId) {
        let ty = self.function.local(local).ty.clone();
        self.emit(Op::LocAddr(local));
        self.stack.push(Type::Ptr(Box::new(ty.clone())));
        self.load(ty)
    }

    /// Load a value corresponding to the pointee type of a pointer operand on the stack.
    ///
    /// The type of the pointer determines what address space the pointer value represents;
    /// either the Miden-native address space (word-addressable), or the IR's byte-addressable
    /// address space.
    pub fn load(&mut self, ty: Type) {
        let ptr = self.stack.pop().expect("operand stack is empty");
        match ptr.ty() {
            Type::Ptr(_) => {
                // Converet the pointer to a native pointer representation
                self.emit_native_ptr();
                match &ty {
                    Type::I128 => self.load_quad_word(None),
                    Type::I64 | Type::U64 => self.load_double_word(None),
                    Type::Felt => self.load_felt(None),
                    Type::I32 | Type::U32 => self.load_word(None),
                    ty @ (Type::I16 | Type::U16 | Type::U8 | Type::I8 | Type::I1) => {
                        self.load_word(None);
                        self.trunc_int32(ty.size_in_bits() as u32);
                    }
                    ty => todo!("support for loading {ty} is not yet implemented"),
                }
                self.stack.push(ty);
            }
            ty if !ty.is_pointer() => {
                panic!("invalid operand to load: expected pointer, got {ty}")
            }
            ty => unimplemented!("load support for pointers of type {ty} is not implemented"),
        }
    }

    /// Load a value of type `ty` from `addr`.
    ///
    /// NOTE: The address represented by `addr` is in the IR's byte-addressable address space.
    pub fn load_imm(&mut self, addr: u32, ty: Type) {
        let ptr = NativePtr::from_ptr(addr);
        match &ty {
            Type::I128 => self.load_quad_word(Some(ptr)),
            Type::I64 | Type::U64 => self.load_double_word(Some(ptr)),
            Type::Felt => self.load_felt(Some(ptr)),
            Type::I32 | Type::U32 => self.load_word(Some(ptr)),
            Type::I16 | Type::U16 | Type::U8 | Type::I8 | Type::I1 => {
                self.load_word(Some(ptr));
                self.trunc_int32(ty.size_in_bits() as u32);
            }
            ty => todo!("support for loading {ty} is not yet implemented"),
        }
        self.stack.push(ty);
    }

    /// Emit a sequence of instructions to translate a raw pointer value to
    /// a native pointer value, as a triple of `(waddr, index, offset)`, in
    /// that order on the stack.
    ///
    /// Instructions which must act on a pointer will expect the stack to have
    /// these values in that order so that they can perform any necessary
    /// re-alignment.
    fn emit_native_ptr(&mut self) {
        self.emit_all(&[
            // Copy the address
            //
            // [addr, addr]
            Op::Dup(0),
            // Obtain the absolute offset
            //
            // [abs_offset, addr]
            Op::U32ModImm(16),
            // Obtain the byte offset
            //
            // [abs_offset, abs_offset, addr]
            Op::Dup(0),
            // [offset, abs_offset, addr]
            Op::U32ModImm(4),
            // Obtain the element index
            //
            // [abs_offset, offset, addr]
            Op::Swap(1),
            // [index, byte_offset, addr]
            Op::U32DivImm(4),
            // Translate the address to Miden's address space
            //
            // [addr, index, offset]
            Op::Movup(2),
            // [waddr, index, offset]
            Op::U32DivImm(16),
        ]);
    }

    /// Load a field element from a naturally aligned address, either immediate or dynamic
    ///
    /// A native pointer triplet is expected on the stack if an immediate is not given.
    fn load_felt(&mut self, ptr: Option<NativePtr>) {
        if let Some(imm) = ptr {
            return self.load_felt_imm(imm);
        }

        self.emit(Op::Exec("intrinsics::mem::load_felt".parse().unwrap()));
    }

    fn load_felt_imm(&mut self, ptr: NativePtr) {
        assert!(ptr.is_element_aligned(), "felt values must be naturally aligned");
        match ptr.index {
            0 => self.emit(Op::MemLoadImm(ptr.waddr)),
            1 => {
                self.emit_all(&[
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Movup(4),
                    Op::Movup(4),
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                ]);
            }
            2 => {
                self.emit_all(&[
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Drop,
                    Op::Drop,
                    Op::Swap(1),
                    Op::Drop,
                ]);
            }
            3 => {
                self.emit_all(&[
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                ]);
            }
            _ => unreachable!(),
        }
    }

    /// Loads a single 32-bit machine word, i.e. a single field element, not the Miden notion of a
    /// word
    ///
    /// Expects a native pointer triplet on the stack if an immediate address is not given.
    fn load_word(&mut self, ptr: Option<NativePtr>) {
        if let Some(imm) = ptr {
            return self.load_word_imm(imm);
        }

        self.emit(Op::Exec("intrinsics::mem::load_sw".parse().unwrap()));
    }

    /// Loads a single 32-bit machine word from the given immediate address.
    fn load_word_imm(&mut self, ptr: NativePtr) {
        let is_aligned = ptr.is_element_aligned();
        let rshift = 32 - ptr.offset as u32;
        match ptr.index {
            0 if is_aligned => self.emit(Op::MemLoadImm(ptr.waddr)),
            0 => {
                self.emit_all(&[
                    // Load a quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Move the two elements across which the desired machine word spans
                    // to the bottom of the stack temporarily
                    Op::Movdn(4),
                    Op::Movdn(4),
                    // Drop the unused elements
                    Op::Drop,
                    Op::Drop,
                    // Shift the high bits left by the offset
                    Op::U32ShlImm(ptr.offset as u32),
                    // Move the low bits to the top and shift them right
                    Op::Swap(1),
                    Op::U32ShrImm(rshift),
                    // OR the high and low bits together
                    Op::U32Or,
                ]);
            }
            1 if is_aligned => self.emit_all(&[
                // Load a quad-word
                Op::Padw,
                Op::MemLoadwImm(ptr.waddr),
                // Drop the first unused element
                Op::Drop,
                // Move the desired element past the last two unused
                Op::Movdn(3),
                // Drop the remaining unused elements
                Op::Drop,
                Op::Drop,
            ]),
            1 => {
                self.emit_all(&[
                    // Load a quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the first unused element
                    Op::Drop,
                    // Move the two elements across which the desired machine word spans
                    // to the bottom of the stack temporarily
                    Op::Movdn(3),
                    Op::Movdn(3),
                    // Drop the remaining unused element
                    Op::Drop,
                    // Shift the high bits left by the offset
                    Op::U32ShlImm(ptr.offset as u32),
                    // Move the low bits to the top and shift them right
                    Op::Swap(1),
                    Op::U32ShrImm(rshift),
                    // OR the high and low bits together
                    Op::U32Or,
                ]);
            }
            2 if is_aligned => self.emit_all(&[
                // Load a quad-word
                Op::Padw,
                Op::MemLoadwImm(ptr.waddr),
                // Drop the first two unused elements
                Op::Drop,
                Op::Drop,
                // Swap the last remaining unused element to the top and drop it
                Op::Swap(1),
                Op::Drop,
            ]),
            2 => {
                self.emit_all(&[
                    // Load a quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the first two unused elements
                    Op::Drop,
                    Op::Drop,
                    // Shift the high bits left by the offset
                    Op::U32ShlImm(ptr.offset as u32),
                    // Move the low bits to the top and shift them right
                    Op::Swap(1),
                    Op::U32ShrImm(rshift),
                    // OR the high and low bits together
                    Op::U32Or,
                ]);
            }
            3 if is_aligned => self.emit_all(&[
                // Load a quad-word
                Op::Padw,
                Op::MemLoadwImm(ptr.waddr),
                // Drop the three unused elements
                Op::Drop,
                Op::Drop,
                Op::Drop,
            ]),
            3 => {
                self.emit_all(&[
                    // Load the quad-word containing the low bits
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Move the element we need to the bottom temporarily
                    Op::Movdn(4),
                    // Drop the unused elements
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                    // Shift the low bits right by the offset
                    Op::U32ShrImm(rshift),
                    // Load the quad-word containing the high bits
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the unused elements
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                    // Shift the high bits left by the offset
                    Op::U32ShlImm(ptr.offset as u32),
                    // OR the high and low bits together
                    Op::U32Or,
                ]);
            }
            _ => unreachable!(),
        }
    }

    /// Load a pair of machine words (32-bit elements) to the operand stack
    fn load_double_word(&mut self, ptr: Option<NativePtr>) {
        if let Some(imm) = ptr {
            return self.load_double_word_imm(imm);
        }

        self.emit(Op::Exec("intrinsics::mem::load_dw".parse().unwrap()));
    }

    fn load_double_word_imm(&mut self, ptr: NativePtr) {
        let aligned = ptr.is_element_aligned();
        match ptr.index {
            0 if aligned => {
                self.emit_all(&[
                    // Load quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Move the two elements we need to the bottom temporarily
                    Op::Movdn(4),
                    Op::Movdn(4),
                    // Drop the unused elements
                    Op::Drop,
                    Op::Drop,
                ]);
            }
            0 => {
                // An unaligned double-word load spans three elements
                self.emit_all(&[
                    // Load quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Move the unused element to the top and drop it
                    Op::Movup(4),
                    Op::Drop,
                ]);
                self.realign_double_word(ptr);
            }
            1 if aligned => {
                self.emit_all(&[
                    // Load quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the first word, its unused
                    Op::Drop,
                    // Move the last word up and drop it, also unused
                    Op::Movup(3),
                    Op::Drop,
                ]);
            }
            1 => {
                // An unaligned double-word load spans three elements
                self.emit_all(&[
                    // Load a quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the unused element
                    Op::Drop,
                ]);
                self.realign_double_word(ptr);
            }
            2 if aligned => {
                self.emit_all(&[
                    // Load quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop unused words
                    Op::Drop,
                    Op::Drop,
                ]);
            }
            2 => {
                // An unaligned double-word load spans three elements,
                // and in this case, two quad-words, because the last
                // element is across a quad-word boundary
                self.emit_all(&[
                    // Load the second quad-word first
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Move the element we need to the bottom temporarily
                    Op::Movdn(4),
                    // Drop the three unused elements of this word
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                    // Load the first quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the two unused elements
                    Op::Drop,
                    Op::Drop,
                ]);
                self.realign_double_word(ptr);
            }
            3 if aligned => {
                self.emit_all(&[
                    // Load second word, drop unused elements
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    Op::Movup(4),
                    Op::Drop,
                    Op::Movup(3),
                    Op::Drop,
                    // Load first word, drop unused elements
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                ]);
            }
            3 => {
                self.emit_all(&[
                    // Load second word, drop unused element
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    Op::Movup(4),
                    Op::Drop,
                    // Load first word, drop unused elements
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                ]);
                self.realign_double_word(ptr);
            }
            _ => unimplemented!("unaligned loads are not yet implemented: {ptr:#?}"),
        }
    }

    /// Load a quartet of machine words (32-bit elements) to the operand stack
    fn load_quad_word(&mut self, ptr: Option<NativePtr>) {
        if let Some(imm) = ptr {
            return self.load_quad_word_imm(imm);
        }
        self.emit(Op::Exec("intrinsics::mem::load_qw".parse().unwrap()));
    }

    fn load_quad_word_imm(&mut self, ptr: NativePtr) {
        // For all other cases, more complicated loads are required
        let aligned = ptr.is_element_aligned();
        match ptr.index {
            // Naturally-aligned
            0 if aligned => self.emit_all(&[Op::Padw, Op::MemLoadwImm(ptr.waddr)]),
            0 => {
                // An unaligned quad-word load spans five elements
                self.emit_all(&[
                    // Load second quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Drop all but the first element
                    Op::Movdn(4),
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                    // Load first quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                ]);
                self.realign_quad_word(ptr);
            }
            1 if aligned => {
                self.emit_all(&[
                    // Load second quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Drop last element
                    Op::Movup(4),
                    Op::Drop,
                    // Load first quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop first element
                    Op::Drop,
                ]);
            }
            1 => {
                // An unaligned double-word load spans five elements
                self.emit_all(&[
                    // Load second quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Drop all but the first two elements
                    Op::Movdn(4),
                    Op::Movdn(4),
                    Op::Drop,
                    Op::Drop,
                    // Load first quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the first word
                    Op::Drop,
                ]);
                self.realign_quad_word(ptr);
            }
            2 if aligned => {
                self.emit_all(&[
                    // Load second quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop last two elements
                    Op::Movup(4),
                    Op::Movup(4),
                    Op::Drop,
                    Op::Drop,
                    // Load first quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop first two elements
                    Op::Drop,
                    Op::Drop,
                ]);
            }
            2 => {
                // An unaligned double-word load spans five elements
                self.emit_all(&[
                    // Load the second quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Drop the last element
                    Op::Movup(4),
                    Op::Drop,
                    // Load the first quad-word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the two unused elements
                    Op::Drop,
                    Op::Drop,
                ]);
                self.realign_quad_word(ptr);
            }
            3 if aligned => {
                self.emit_all(&[
                    // Load second word, drop last element
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    Op::Movup(4),
                    Op::Drop,
                    // Load first word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop first three elements
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                ]);
            }
            3 => {
                // An unaligned quad-word load spans five elements,
                self.emit_all(&[
                    // Load second word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Load first word
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop unused elements
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                ]);
                self.realign_quad_word(ptr);
            }
            _ => unimplemented!("unaligned loads are not yet implemented: {ptr:#?}"),
        }
    }

    /// This handles emitting code that handles aligning an unaligned double machine-word value
    /// which is split across three machine words (field elements).
    ///
    /// To recap:
    ///
    /// * A machine word is a 32-bit chunk stored in a single field element
    /// * A double word is a pair of 32-bit chunks
    /// * A quad word is a quartet of 32-bit chunks (i.e. a Miden "word")
    /// * An unaligned double-word requires three 32-bit chunks to represent,
    /// since the first chunk does not contain a full 32-bits, so an extra is
    /// needed to hold those bits.
    ///
    /// As an example, assume the pointer we are dereferencing is a u64 value,
    /// which has 8-byte alignment, and the value is stored 40 bytes from the
    /// nearest quad-word-aligned boundary. To load the value, we must fetch
    /// the full quad-word from the aligned address, drop the first word, as
    /// it is unused, and then recombine the 64 bits we need spread across
    /// the remaining three words to obtain the double-word value we actually want.
    ///
    /// The data, on the stack, is shown below:
    ///
    /// ```text,ignore
    /// # If we visualize which bytes are contained in each 32-bit chunk on the stack, we get:
    /// [0..=4, 5..=8, 9..=12]
    ///
    /// # These byte indices are relative to the nearest word-aligned address, in the same order
    /// # as they would occur in a byte-addressable address space. The significance of each byte
    /// # depends on the value being dereferenced, but Miden is a little-endian machine, so typically
    /// # the most significant bytes come first (i.e. also commonly referred to as "high" vs "low" bits).
    /// #
    /// # If we visualize the layout of the bits of our u64 value spread across the three chunks, we get:
    /// [00000000111111111111111111111111, 111111111111111111111111111111, 11111111111111111111111100000000]
    /// ```
    ///
    /// As illustrated above, what should be a double-word value is occupying three words. To
    /// "realign" the value, i.e. ensure that it is naturally aligned and fits in two words, we
    /// have to perform a sequence of shifts and masks to get the bits where they belong. This
    /// function performs those steps, with the assumption that the caller has three values on
    /// the operand stack representing any unaligned double-word value
    fn realign_double_word(&mut self, ptr: NativePtr) {
        // The stack starts as: [chunk_hi, chunk_mid, chunk_lo]
        //
        // We will refer to the parts of our desired double-word value
        // as two parts, `x_hi` and `x_lo`.
        self.emit_all(&[
            // Re-align the high bits by shifting out the offset
            //
            // This gives us the first half of the first word.
            //
            // [x_hi_hi, chunk_mid, chunk__lo]
            Op::U32ShlImm(ptr.offset as u32),
            // Move the value below the other chunks temporarily
            //
            // [chunk_mid, chunk_lo, x_hi_hi]
            Op::Movdn(3),
            // We must split the middle chunk into two parts,
            // one containing the bits to be combined with the
            // first machine word; the other to be combined with
            // the second machine word.
            //
            // First, we duplicate the chunk, since we need two
            // copies of it:
            //
            // [chunk_mid, chunk_mid, chunk_lo, x_hi_hi]
            Op::Dup(0),
            // Then, we shift the chunk right by 32 - offset bits,
            // re-aligning the low bits of the first word, and
            // isolating them.
            //
            // [x_hi_lo, chunk_mid, chunk_lo, x_hi_hi]
            Op::U32ShrImm(32 - ptr.offset as u32),
            // Move the high bits back to the top
            //
            // [x_hi_hi, x_hi_lo, chunk_mid, chunk_lo]
            Op::Movup(3),
            // OR the two parts of the `x_hi` chunk together
            //
            // [x_hi, chunk_mid, chunk_lo]
            Op::U32Or,
            // Move `x_hi` to the bottom for later
            Op::Movdn(2),
            // Now, we need to re-align the high bits of the second word
            // by shifting the remaining copy of the middle chunk, similar
            // to what we did at the very beginning.
            //
            // This gives us the first half of the second word.
            //
            // [x_lo_hi, chunk_lo, x_hi]
            Op::U32ShlImm(ptr.offset as u32),
            // Next, swap the low bit chunk to the top temporarily
            Op::Swap(1),
            // Shift the value right, as done previously for the middle chunk
            Op::U32ShrImm(32 - ptr.offset as u32),
            // OR the two halves together, giving us our second word, `x_lo`
            //
            // [x_lo, x_hi]
            Op::U32Or,
            // Swap the words so they are in the correct order
            //
            // [x_hi, x_lo]
            Op::Swap(1),
        ]);
    }

    /// This handles emitting code that handles aligning an unaligned quad machine-word value
    /// which is split across five machine words (field elements).
    ///
    /// To recap:
    ///
    /// * A machine word is a 32-bit chunk stored in a single field element
    /// * A double word is a pair of 32-bit chunks
    /// * A quad word is a quartet of 32-bit chunks (i.e. a Miden "word")
    /// * An unaligned quad-word requires five 32-bit chunks to represent,
    /// since the first chunk does not contain a full 32-bits, so an extra is
    /// needed to hold those bits.
    ///
    /// See the example in [OpEmitter::realign_quad_word] for more details on how bits are
    /// laid out in each word, and what is required to realign unaligned words.
    fn realign_quad_word(&mut self, ptr: NativePtr) {
        // The stack starts as: [chunk_hi, chunk_mid_hi, chunk_mid_mid, chunk_mid_lo, chunk_lo]
        //
        // We will refer to the parts of our desired quad-word value
        // as four parts, `x_hi2`, `x_hi1`, `x_lo2`, and `x_lo1`, where
        // the integer suffix should appear in decreasing order on the
        // stack when we're done.
        self.emit_all(&[
            // Re-align the high bits by shifting out the offset
            //
            // This gives us the first half of `x_hi2`.
            //
            // [x_hi2_hi, chunk_mid_hi, chunk_mid_mid, chunk_mid_lo, chunk__lo]
            Op::U32ShlImm(ptr.offset as u32),
            // Move the value below the other chunks temporarily
            //
            // [chunk_mid_hi, chunk_mid_mid, chunk_mid_lo, chunk__lo, x_hi2_hi]
            Op::Movdn(5),
            // We must split the `chunk_mid_hi` chunk into two parts,
            // one containing the bits to be combined with `x_hi2_hi`;
            // the other to be combined with `x_hi1_hi`.
            //
            // First, we duplicate the chunk, since we need two
            // copies of it:
            //
            // [chunk_mid_hi, chunk_mid_hi, chunk_mid_mid, chunk_mid_lo, chunk_lo, x_hi2_hi]
            Op::Dup(0),
            // Then, we shift the chunk right by 32 - offset bits,
            // re-aligning the low bits of `x_hi2`, and isolating them.
            //
            // [x_hi2_lo, chunk_mid_hi, chunk_mid_mid, chunk_mid_lo, chunk_lo, x_hi2_hi]
            Op::U32ShrImm(32 - ptr.offset as u32),
            // Move the high bits of `x_hi2` back to the top
            //
            // [x_hi2_hi, x_hi2_lo, chunk_mid_hi, chunk_mid_mid, chunk_mid_lo, chunk_lo]
            Op::Movup(3),
            // OR the two parts of the `x_hi2` chunk together
            //
            // [x_hi2, chunk_mid_hi, chunk_mid_mid, chunk_mid_lo, chunk_lo]
            Op::U32Or,
            // Move `x_hi2` to the bottom for later
            //
            // [chunk_mid_hi, chunk_mid_mid, chunk_mid_lo, chunk_lo, x_hi2]
            Op::Movdn(5),
            // Now, we need to re-align the high bits of `x_hi1` by shifting
            // the remaining copy of `chunk_mid_hi`, similar to what we did for `x_hi2`
            //
            // This gives us the first half of `x_hi1`
            //
            // [x_hi1_hi, chunk_mid_mid, chunk_mid_lo, chunk_lo, x_hi2]
            Op::U32ShlImm(ptr.offset as u32),
            // Next, move the chunk containing the low bits of `x_hi1` to the top temporarily
            //
            // [chunk_mid_mid, chunk_mid_lo, chunk_lo, x_hi2, x_hi1_hi]
            Op::Movdn(5),
            // Duplicate it, as we need two copies
            //
            // [chunk_mid_mid, chunk_mid_mid, chunk_mid_lo, chunk_lo, x_hi2, x_hi1_hi]
            Op::Dup(0),
            // Shift the value right, as done previously for the low bits of `x_hi2`
            //
            // [x_hi1_lo, chunk_mid_mid, chunk_mid_lo, chunk_lo, x_hi2, x_hi1_hi]
            Op::U32ShrImm(32 - ptr.offset as u32),
            // Move the high bits of `x_hi1` to the top
            Op::Movup(5),
            // OR the two halves together, giving us our second word, `x_hi1`
            //
            // [x_hi1, chunk_mid_mid, chunk_mid_lo, chunk_lo, x_hi2]
            Op::U32Or,
            // Move the word to the bottom of the stack
            //
            // [chunk_mid_mid, chunk_mid_lo, chunk_lo, x_hi2, x_hi1]
            Op::Movdn(5),
            // Now, we need to re-align the high bits of `x_lo2` by shifting
            // the remaining copy of `chunk_mid_mid`, as done previously.
            //
            // [x_lo2_hi, chunk_mid_lo, chunk_lo, x_hi2, x_hi1]
            Op::U32ShlImm(ptr.offset as u32),
            // Next, move the chunk containing the low bits of `x_lo2` to the top temporarily
            //
            // [chunk_mid_lo, chunk_lo, x_hi2, x_hi1, x_lo2_hi]
            Op::Movdn(5),
            // Duplicate it, as done previously
            //
            // [chunk_mid_lo, chunk_mid_lo, chunk_lo, x_hi2, x_hi1, x_lo2_hi]
            Op::Dup(0),
            // Shift the value right to get the low bits of `x_lo2`
            //
            // [x_lo2_lo, chunk_mid_lo, chunk_lo, x_hi2, x_hi1, x_lo2_hi]
            Op::U32ShrImm(32 - ptr.offset as u32),
            // Move the high bits of `x_lo2` to the top
            //
            // [x_lo2_hi, x_lo2_lo, chunk_mid_lo, chunk_lo, x_hi2, x_hi1]
            Op::Movup(6),
            // OR the two halves together, giving us our third word, `x_lo2`
            //
            // [x_lo2, chunk_mid_lo, chunk_lo, x_hi2, x_hi1]
            Op::U32Or,
            // Move to the bottom of the stack
            //
            // [chunk_mid_lo, chunk_lo, x_hi2, x_hi1, x_lo2]
            Op::Movdn(5),
            // Re-align the high bits of `x_lo1`
            //
            // [x_lo1_hi, chunk_lo, x_hi2, x_hi1, x_lo2]
            Op::U32ShlImm(ptr.offset as u32),
            // Move the chunk containing the low bits to the top
            //
            // [chunk_lo, x_hi2, x_hi1, x_lo2, x_lo1_hi]
            Op::Movdn(5),
            // Shift the value right to get the low bits of `x_lo1`
            Op::U32ShrImm(32 - ptr.offset as u32),
            // Move the high bits of `x_lo1` to the top
            //
            // [x_lo1_hi, x_lo1_lo, x_hi2, x_hi1, x_lo2]
            Op::Movup(5),
            // OR the two halves together, giving us our fourth word, `x_lo1`
            //
            // [x_lo1, x_hi2, x_hi1, x_lo2]
            Op::U32Or,
            // Move to the bottom
            //
            // [x_hi2, x_hi1, x_lo2, x_lo1]
            Op::Movdn(5),
        ]);
    }
}

/// Stores
impl<'a> OpEmitter<'a> {
    /// Store a value of the type given by the specified [hir::LocalId], using the memory allocated
    /// for that local.
    ///
    /// Internally, this pushes the address of the given local on the stack, and delegates to
    /// [OpEmitter::store] to perform the actual store.
    pub fn store_local(&mut self, local: hir::LocalId) {
        let ty = self.function.local(local).ty.clone();
        self.emit(Op::LocAddr(local));
        self.stack.push(Type::Ptr(Box::new(ty)));
        self.store()
    }

    /// Store a value of type `value` to the address in the Miden address space
    /// which corresponds to a pointer in the IR's byte-addressable address space.
    ///
    /// The type of the pointer is given as `ptr`, and can be used for both validation and
    /// determining alignment.
    pub fn store(&mut self) {
        let ptr = self.stack.pop().expect("operand stack is empty");
        let value = self.stack.pop().expect("operand stack is empty");
        let ptr_ty = ptr.ty();
        assert!(ptr_ty.is_pointer(), "expected store operand to be a pointer, got {ptr_ty}");
        let value_ty = value.ty();
        assert!(!value_ty.is_zst(), "cannot store a zero-sized type in memory");
        match ptr_ty {
            Type::Ptr(_) => {
                // Convert the pointer to a native pointer representation
                self.emit_native_ptr();
                match value_ty {
                    Type::I128 => self.store_quad_word(None),
                    Type::I64 | Type::U64 => self.store_double_word(None),
                    Type::Felt => self.store_felt(None),
                    Type::I32 | Type::U32 => self.store_word(None),
                    ref ty if ty.size_in_bytes() <= 4 => self.store_small(ty, None),
                    Type::Array(ref elem_ty, _) => self.store_array(elem_ty, None),
                    Type::Struct(ref struct_ty) => self.store_struct(struct_ty, None),
                    ty => unimplemented!(
                        "invalid store: support for storing {ty} has not been implemented"
                    ),
                }
            }
            ty if !ty.is_pointer() => {
                panic!("invalid operand to store: expected pointer, got {ty}")
            }
            ty => unimplemented!("store support for pointers of type {ty} is not implemented"),
        }
    }

    /// Store a value of type `ty` to `addr`.
    ///
    /// NOTE: The address represented by `addr` is in the IR's byte-addressable address space.
    pub fn store_imm(&mut self, addr: u32) {
        let value = self.stack.pop().expect("operand stack is empty");
        let value_ty = value.ty();
        assert!(!value_ty.is_zst(), "cannot store a zero-sized type in memory");
        let ptr = NativePtr::from_ptr(addr);
        match value_ty {
            Type::I128 => self.store_quad_word(Some(ptr)),
            Type::I64 | Type::U64 => self.store_double_word(Some(ptr)),
            Type::Felt => self.store_felt(Some(ptr)),
            Type::I32 | Type::U32 => self.store_word(Some(ptr)),
            ref ty if ty.size_in_bytes() <= 4 => self.store_small(ty, Some(ptr)),
            Type::Array(ref elem_ty, _) => self.store_array(elem_ty, Some(ptr)),
            Type::Struct(ref struct_ty) => self.store_struct(struct_ty, Some(ptr)),
            ty => {
                unimplemented!("invalid store: support for storing {ty} has not been implemented")
            }
        }
    }

    pub fn memset(&mut self) {
        let dst = self.stack.pop().expect("operand stack is empty");
        let count = self.stack.pop().expect("operand stack is empty");
        let value = self.stack.pop().expect("operand stack is empty");
        assert_eq!(count.ty(), Type::U32, "expected count operand to be a u32");
        let ty = value.ty();
        assert!(dst.ty().is_pointer());
        assert_eq!(&ty, dst.ty().pointee().unwrap(), "expected value and pointee type to match");

        // Prepare to loop until `count` iterations have been performed
        let current_block = self.current_block;
        let body = self.function.create_block();
        self.emit_all(&[
            // [dst, count, value..]
            Op::PushU32(0),         // [i, dst, count, value..]
            Op::Dup(2),             // [count, i, dst, count, value..]
            Op::GteImm(Felt::ZERO), // [count > 0, i, dst, count, value..]
            Op::While(body),
        ]);

        // Loop body - compute address for next value to be written
        let value_size = value.ty().size_in_bytes();
        self.switch_to_block(body);
        self.emit_all(&[
            // [i, dst, count, value..]
            // Offset the pointer by the current iteration count * aligned size of value, and trap
            // if it overflows
            Op::Dup(1), // [dst, i, dst, count, value]
            Op::Dup(1), // [i, dst, i, dst, count, value]
            Op::PushU32(value_size.try_into().expect("invalid value size")), /* [value_size, i,
                         * dst, ..] */
            Op::U32OverflowingMadd, // [value_size * i + dst, i, dst, count, value]
            Op::Assertz,            // [aligned_dst, i, dst, count, value..]
        ]);

        // Loop body - move value to top of stack, swap with pointer
        self.stack.push(value);
        self.stack.push(count);
        self.stack.push(dst.clone());
        self.stack.push(dst.ty());
        self.stack.push(dst.ty());
        self.dup(4); // [value, aligned_dst, i, dst, count, value]
        self.swap(1); // [aligned_dst, value, i, dst, count, value]

        // Loop body - write value to destination
        self.store(); // [i, dst, count, value]

        // Loop body - increment iteration count, determine whether to continue loop
        self.emit_all(&[
            Op::U32WrappingAddImm(1),
            Op::Dup(0), // [i++, i++, dst, count, value]
            Op::Dup(3), // [count, i++, i++, dst, count, value]
            Op::U32Gte, // [i++ >= count, i++, dst, count, value]
        ]);

        // Cleanup - at end of 'while' loop, drop the 4 operands remaining on the stack
        self.switch_to_block(current_block);
        self.dropn(4);
    }

    /// Copy `count * sizeof(*ty)` from a source address to a destination address.
    ///
    /// The order of operands on the stack is `src`, `dst`, then `count`.
    ///
    /// The addresses on the stack are interpreted based on the pointer type: native pointers are
    /// in the Miden address space; non-native pointers are assumed to be in the IR's byte
    /// addressable address space, and require translation.
    ///
    /// The semantics of this instruction are as follows:
    ///
    /// * The ``
    pub fn memcpy(&mut self) {
        let src = self.stack.pop().expect("operand stack is empty");
        let dst = self.stack.pop().expect("operand stack is empty");
        let count = self.stack.pop().expect("operand stack is empty");
        assert_eq!(count.ty(), Type::U32, "expected count operand to be a u32");
        let ty = src.ty();
        assert!(ty.is_pointer());
        assert_eq!(ty, dst.ty(), "expected src and dst operands to have the same type");
        let value_ty = ty.pointee().unwrap();
        let value_size = u32::try_from(value_ty.size_in_bytes()).expect("invalid value size");

        // Use optimized intrinsics when available
        match value_size {
            // Word-sized values have an optimized intrinsic we can lean on
            16 => {
                self.emit_all(&[
                    // [src, dst, count]
                    Op::Movup(2), // [count, src, dst]
                    Op::Exec("std::mem::memcopy".parse().unwrap()),
                ]);
                return;
            }
            // Values which can be broken up into word-sized chunks can piggy-back on the
            // intrinsic for word-sized values, but we have to compute a new `count` by
            // multiplying `count` by the number of words in each value
            size if size % 16 == 0 => {
                let factor = size / 16;
                self.emit_all(&[
                    // [src, dst, count]
                    Op::Movup(2), // [count, src, dst]
                    Op::U32OverflowingMulImm(factor),
                    Op::Assertz, // [count * (size / 16), src, dst]
                    Op::Exec("std::mem::memcopy".parse().unwrap()),
                ]);
                return;
            }
            // For now, all other values fallback to the default implementation
            _ => (),
        }

        // Prepare to loop until `count` iterations have been performed
        let current_block = self.current_block;
        let body = self.function.create_block();
        self.emit_all(&[
            // [src, dst, count]
            Op::PushU32(0),         // [i, src, dst, count]
            Op::Dup(3),             // [count, i, src, dst, count]
            Op::GteImm(Felt::ZERO), // [count > 0, i, src, dst, count]
            Op::While(body),
        ]);

        // Loop body - compute address for next value to be written
        self.switch_to_block(body);

        // Compute the source and destination addresses
        self.emit_all(&[
            // [i, src, dst, count]
            Op::Dup(2),              // [dst, i, src, dst, count]
            Op::Dup(1),              // [i, dst, i, src, dst, count]
            Op::PushU32(value_size), // [offset, i, dst, i, src, dst, count]
            Op::U32OverflowingMadd,
            Op::Assertz,             // [new_dst := i * offset + dst, i, src, dst, count]
            Op::Dup(2),              // [src, new_dst, i, src, dst, count]
            Op::Dup(2),              // [i, src, new_dst, i, src, dst, count]
            Op::PushU32(value_size), // [offset, i, src, new_dst, i, src, dst, count]
            Op::U32OverflowingMadd,
            Op::Assertz, // [new_src := i * offset + src, new_dst, i, src, dst, count]
        ]);

        // Load the source value
        self.stack.push(count.clone());
        self.stack.push(dst.clone());
        self.stack.push(src.clone());
        self.stack.push(Type::U32);
        self.stack.push(dst.clone());
        self.stack.push(src.clone());
        self.load(value_ty.clone()); // [value, new_dst, i, src, dst, count]

        // Write to the destination
        self.swap(1); // [new_dst, value, i, src, dst, count]
        self.store(); // [i, src, dst, count]

        // Increment iteration count, determine whether to continue loop
        self.emit_all(&[
            Op::U32WrappingAddImm(1),
            Op::Dup(0), // [i++, i++, src, dst, count]
            Op::Dup(4), // [count, i++, i++, src, dst, count]
            Op::U32Gte, // [i++ >= count, i++, src, dst, count]
        ]);

        // Cleanup - at end of 'while' loop, drop the 4 operands remaining on the stack
        self.switch_to_block(current_block);
        self.dropn(4);
    }

    /// Store a quartet of machine words (32-bit elements) to the operand stack
    fn store_quad_word(&mut self, ptr: Option<NativePtr>) {
        if let Some(imm) = ptr {
            return self.store_quad_word_imm(imm);
        }
        self.emit(Op::Exec("intrinsics::mem::store_qw".parse().unwrap()));
    }

    fn store_quad_word_imm(&mut self, ptr: NativePtr) {
        // For all other cases, more complicated loads are required
        let aligned = ptr.is_element_aligned();
        match ptr.index {
            // Naturally-aligned
            0 if aligned => self.emit_all(&[Op::Padw, Op::MemLoadwImm(ptr.waddr)]),
            _ => {
                todo!()
            }
        }
    }

    /// Store a pair of machine words (32-bit elements) to the operand stack
    fn store_double_word(&mut self, ptr: Option<NativePtr>) {
        if let Some(imm) = ptr {
            return self.store_double_word_imm(imm);
        }

        self.emit(Op::Exec("intrinsics::mem::store_dw".parse().unwrap()));
    }

    fn store_double_word_imm(&mut self, ptr: NativePtr) {
        // For all other cases, more complicated stores are required
        let aligned = ptr.is_element_aligned();
        match ptr.index {
            // Naturally-aligned
            0 if aligned => self.emit_all(&[Op::Padw, Op::MemLoadwImm(ptr.waddr)]),
            _ => {
                todo!()
            }
        }
    }

    /// Stores a single 32-bit machine word, i.e. a single field element, not the Miden notion of a
    /// word
    ///
    /// Expects a native pointer triplet on the stack if an immediate address is not given.
    fn store_word(&mut self, ptr: Option<NativePtr>) {
        if let Some(imm) = ptr {
            return self.store_word_imm(imm);
        }

        self.emit(Op::Exec("intrinsics::mem::store_sw".parse().unwrap()));
    }

    /// Stores a single 32-bit machine word to the given immediate address.
    fn store_word_imm(&mut self, ptr: NativePtr) {
        let is_aligned = ptr.is_element_aligned();
        let rshift = 32 - ptr.offset as u32;
        match ptr.index {
            0 if is_aligned => self.emit(Op::MemStoreImm(ptr.waddr)),
            0 => {
                let mask_hi = u32::MAX << rshift;
                let mask_lo = u32::MAX >> (ptr.offset as u32);
                self.emit_all(&[
                    // Load the full quad-word on to the operand stack
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Manipulate the bits of the first two elements, such that the 32-bit
                    // word we're storing is placed at the correct offset from the start
                    // of the memory cell when viewing the cell as a set of 4 32-bit chunks
                    //
                    // First, mask out the bits we plan to overwrite with the store op from the
                    // first two elements
                    Op::Swap(1),
                    Op::PushU32(mask_lo),
                    Op::U32And,
                    Op::Swap(1),
                    Op::PushU32(mask_hi),
                    Op::U32And,
                    // Now, we need to shift/mask/split the 32-bit value into two elements, then
                    // combine them with the preserved bits of the original
                    // contents of the cell
                    //
                    // We start with the bits belonging to the first element in the cell
                    Op::Dup(4),
                    Op::U32ShrImm(ptr.offset as u32),
                    Op::U32Or,
                    // Then the bits belonging to the second element in the cell
                    Op::Movup(4),
                    Op::U32ShlImm(rshift),
                    Op::Movup(2),
                    Op::U32Or,
                    // Make sure the elements of the cell are in order
                    Op::Swap(1),
                    // Write the word back to the cell
                    Op::MemStorewImm(ptr.waddr),
                    // Clean up the operand stack
                    Op::Dropw,
                ]);
            }
            1 if is_aligned => self.emit_all(&[
                // Load a quad-word
                Op::Padw,
                Op::MemLoadwImm(ptr.waddr),
                // Replace the stored element
                Op::Movup(4),
                Op::Swap(2),
                Op::Drop,
                // Write the word back to the cell
                Op::MemStorewImm(ptr.waddr),
                // Clean up the operand stack
                Op::Dropw,
            ]),
            1 => {
                let mask_hi = u32::MAX << rshift;
                let mask_lo = u32::MAX >> (ptr.offset as u32);
                self.emit_all(&[
                    // Load the full quad-word on to the operand stack
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Manipulate the bits of the middle two elements, such that the 32-bit
                    // word we're storing is placed at the correct offset from the start
                    // of the memory cell when viewing the cell as a set of 4 32-bit chunks
                    //
                    // First, mask out the bits we plan to overwrite with the store op from the
                    // first two elements
                    Op::Swap(2), // [elem3, elem2, elem1, elem4, value]
                    Op::PushU32(mask_lo),
                    Op::U32And,
                    Op::Swap(1), // [elem2, elem3, elem1, elem4, value]
                    Op::PushU32(mask_hi),
                    Op::U32And,
                    // Now, we need to shift/mask/split the 32-bit value into two elements, then
                    // combine them with the preserved bits of the original
                    // contents of the cell
                    //
                    // We start with the bits belonging to the second element in the cell
                    Op::Dup(4), // [value, elem2, elem3, elem1, elem4, value]
                    Op::U32ShrImm(ptr.offset as u32),
                    Op::U32Or,
                    // Then the bits belonging to the third element in the cell
                    Op::Movup(4),
                    Op::U32ShlImm(rshift),
                    Op::Movup(2),
                    Op::U32Or, // [elem3, elem2, elem1, elem4]
                    // Make sure the elements of the cell are in order
                    Op::Swap(1),
                    Op::Movup(2), // [elem1, elem2, elem3, elem4]
                    // Write the word back to the cell
                    Op::MemStorewImm(ptr.waddr),
                    // Clean up the operand stack
                    Op::Dropw,
                ]);
            }
            2 if is_aligned => self.emit_all(&[
                // Load a quad-word
                Op::Padw,
                Op::MemLoadwImm(ptr.waddr),
                // Replace the stored element
                Op::Movup(5),
                Op::Swap(3),
                Op::Drop,
                // Write the word back to the cell
                Op::MemStorewImm(ptr.waddr),
                // Clean up the operand stack
                Op::Dropw,
            ]),
            2 => {
                let mask_hi = u32::MAX << rshift;
                let mask_lo = u32::MAX >> (ptr.offset as u32);
                self.emit_all(&[
                    // Load the full quad-word on to the operand stack
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Manipulate the bits of the last two elements, such that the 32-bit
                    // word we're storing is placed at the correct offset from the start
                    // of the memory cell when viewing the cell as a set of 4 32-bit chunks
                    //
                    // First, mask out the bits we plan to overwrite with the store op from the
                    // first two elements
                    Op::Swap(3), // [elem4, elem2, elem3, elem1, value]
                    Op::PushU32(mask_lo),
                    Op::U32And,
                    Op::Movup(2), // [elem3, elem4, elem2, elem1, value]
                    Op::PushU32(mask_hi),
                    Op::U32And,
                    // Now, we need to shift/mask/split the 32-bit value into two elements, then
                    // combine them with the preserved bits of the original
                    // contents of the cell
                    //
                    // We start with the bits belonging to the third element in the cell
                    Op::Dup(4), // [value, elem3, elem4, elem2, elem1, value]
                    Op::U32ShrImm(ptr.offset as u32),
                    Op::U32Or,
                    // Then the bits belonging to the fourth element in the cell
                    Op::Movup(4),
                    Op::U32ShlImm(rshift),
                    Op::Movup(2),
                    Op::U32Or, // [elem4, elem3, elem2, elem1]
                    // Make sure the elements of the cell are in order
                    Op::Swap(2),  // [elem2, elem3, elem4, elem1]
                    Op::Movup(3), // [elem1, elem2, elem3, elem4]
                    // Write the word back to the cell
                    Op::MemStorewImm(ptr.waddr),
                    // Clean up the operand stack
                    Op::Dropw,
                ]);
            }
            3 if is_aligned => self.emit_all(&[
                // Load a quad-word
                Op::Padw,
                Op::MemLoadwImm(ptr.waddr),
                // Replace the stored element
                Op::Movup(4),
                Op::Drop,
                // Write the word back to the cell
                Op::MemStorewImm(ptr.waddr),
                // Clean up the operand stack
                Op::Dropw,
            ]),
            3 => {
                // This is a rather annoying edge case, as it requires us to store bits
                // across two different words. We start with the "hi" bits that go at
                // the end of the first word, and then handle the "lo" bits in a simpler
                // fashion
                let mask_hi = u32::MAX << rshift;
                let mask_lo = u32::MAX >> (ptr.offset as u32);
                self.emit_all(&[
                    // Load the full quad-word on to the operand stack
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    // Manipulate the bits of the last element, such that the "high" bits
                    // of the 32-bit word we're storing is placed at the correct offset from the
                    // start of the memory cell when viewing the cell as a set
                    // of 4 32-bit chunks
                    //
                    // First, mask out the bits we plan to overwrite with the store op from the
                    // last element
                    Op::Swap(3), // [elem4, elem2, elem3, elem1, value]
                    Op::PushU32(mask_lo),
                    Op::U32And,
                    // Now, we need to shift/mask/split the 32-bit value into the bits that will be
                    // merged with this word
                    Op::Dup(4), // [value, elem4, elem2, elem3, elem1, value]
                    Op::U32ShrImm(ptr.offset as u32),
                    Op::U32Or,
                    // Move the fourth element back into place
                    Op::Swap(3), // [elem1, elem2, elem3, elem4, value]
                    // Write the first word and clear the operand stack
                    Op::MemStorewImm(ptr.waddr),
                    Op::Dropw,
                    // Compute the bits of the value that we'll merge into the second word
                    Op::U32ShlImm(rshift),
                    // Load the first element of the second word
                    Op::MemLoadImm(ptr.waddr + 1),
                    // Mask out the bits we plan to overwrite
                    Op::PushU32(mask_hi),
                    Op::U32And,
                    // Merge the bits and write back the second word
                    Op::U32Or,
                    Op::MemStoreImm(ptr.waddr + 1),
                ]);
            }
            _ => unreachable!(),
        }
    }

    /// Store a field element to a naturally aligned address, either immediate or dynamic
    ///
    /// A native pointer triplet is expected on the stack if an immediate is not given.
    fn store_felt(&mut self, ptr: Option<NativePtr>) {
        if let Some(imm) = ptr {
            return self.store_felt_imm(imm);
        }

        self.emit(Op::Exec("intrinsics::mem::store_felt".parse().unwrap()));
    }

    fn store_felt_imm(&mut self, ptr: NativePtr) {
        assert!(ptr.is_element_aligned(), "felt values must be naturally aligned");
        match ptr.index {
            0 => self.emit(Op::MemStoreImm(ptr.waddr)),
            1 => {
                self.emit_all(&[
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Movup(4),
                    Op::Swap(2),
                    Op::Drop,
                    Op::MemStorewImm(ptr.waddr),
                    Op::Dropw,
                ]);
            }
            2 => {
                self.emit_all(&[
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Movup(4),
                    Op::Swap(3),
                    Op::Drop,
                    Op::MemStorewImm(ptr.waddr),
                    Op::Dropw,
                ]);
            }
            3 => {
                self.emit_all(&[
                    Op::Padw,
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Movup(3),
                    Op::Drop,
                    Op::MemStorewImm(ptr.waddr),
                    Op::Dropw,
                ]);
            }
            _ => unreachable!(),
        }
    }

    fn store_small(&mut self, ty: &Type, ptr: Option<NativePtr>) {
        if let Some(imm) = ptr {
            return self.store_small_imm(ty, imm);
        }

        let type_size = ty.size_in_bits();
        if type_size == 32 {
            self.store_word(ptr);
            return;
        }

        // Duplicate the address
        self.emit_all(&[Op::Dup(2), Op::Dup(2), Op::Dup(2)]);

        // Load the current 32-bit value at `ptr`
        self.load_word(ptr);

        // Mask out the bits we're going to be writing from the loaded value
        let mask = u32::MAX << type_size;
        self.const_mask_u32(mask);

        // Mix in the bits we want to write: [masked, addr1, addr2, addr3, value]
        self.emit(Op::Movup(5));
        self.bor_u32();

        // Store the combined bits: [value, addr1, addr2, addr3]
        self.emit(Op::Movdn(4));
        self.store_word(ptr);
    }

    fn store_small_imm(&mut self, ty: &Type, ptr: NativePtr) {
        assert!(ptr.alignment() as usize >= ty.min_alignment());

        let type_size = ty.size_in_bits();
        if type_size == 32 {
            self.store_word_imm(ptr);
            return;
        }

        // Load the current 32-bit value at `ptr`
        self.load_word_imm(ptr);

        // Mask out the bits we're going to be writing from the loaded value
        let mask = u32::MAX << type_size;
        self.const_mask_u32(mask);

        // Mix in the bits we want to write
        self.emit(Op::Movup(4));
        self.bor_u32();

        // Store the combined bits
        self.store_word_imm(ptr);
    }

    fn store_array(&mut self, _element_ty: &Type, _ptr: Option<NativePtr>) {
        todo!()
    }

    fn store_struct(&mut self, _ty: &StructType, _ptr: Option<NativePtr>) {
        todo!()
    }
}
