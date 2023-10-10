use miden_hir::{StructType, Type};

use crate::masm::{NativePtr, Op};

use super::OpEmitter;

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
}

/// Loads
impl<'a> OpEmitter<'a> {
    /// Load a value of corresponding to the pointee type of a pointer operand on the stack.
    ///
    /// The type of the pointer determines what address space the pointer value represents;
    /// either the Miden-native address space (word-addressable), or the IR's byte-addressable
    /// address space.
    pub fn load(&mut self, ty: Type) {
        let ptr = self.stack.pop().expect("operand stack is empty");
        match ptr.ty() {
            Type::Ptr(_) => {
                // Converet the pointer to a native pointer representation
                self.to_native_ptr();
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
    fn to_native_ptr(&mut self) {
        self.emit_all(&[
            // Copy the address
            //
            // [addr, addr]
            Op::Dup(0),
            // Obtain the absolute offset
            //
            // [abs_offset, addr]
            Op::U32CheckedModImm(16),
            // Obtain the byte offset
            //
            // [abs_offset, abs_offset, addr]
            Op::Dup(0),
            // [offset, abs_offset, addr]
            Op::U32CheckedModImm(4),
            // Obtain the element index
            //
            // [abs_offset, offset, addr]
            Op::Swap(1),
            // [index, byte_offset, addr]
            Op::U32CheckedDivImm(4),
            // Translate the address to Miden's address space
            //
            // [addr, index, offset]
            Op::Movup(2),
            // [waddr, index, offset]
            Op::U32CheckedDivImm(16),
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
        assert!(
            ptr.is_element_aligned(),
            "felt values must be naturally aligned"
        );
        match ptr.index {
            0 => self.emit(Op::MemLoadImm(ptr.waddr)),
            1 => {
                self.emit_all(&[
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
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Drop,
                    Op::Drop,
                    Op::Swap(1),
                    Op::Drop,
                ]);
            }
            3 => {
                self.emit_all(&[Op::MemLoadwImm(ptr.waddr), Op::Drop, Op::Drop, Op::Drop]);
            }
            _ => unreachable!(),
        }
    }

    /// Loads a single 32-bit machine word, i.e. a single field element, not the Miden notion of a word
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
                    Op::MemLoadwImm(ptr.waddr),
                    // Move the two elements across which the desired machine word spans
                    // to the bottom of the stack temporarily
                    Op::Movdn(4),
                    Op::Movdn(4),
                    // Drop the unused elements
                    Op::Drop,
                    Op::Drop,
                    // Shift the high bits left by the offset
                    Op::U32CheckedShlImm(ptr.offset as u32),
                    // Move the low bits to the top and shift them right
                    Op::Swap(1),
                    Op::U32CheckedShrImm(rshift),
                    // OR the high and low bits together
                    Op::U32Or,
                ]);
            }
            1 if is_aligned => self.emit_all(&[
                // Load a quad-word
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
                    Op::U32CheckedShlImm(ptr.offset as u32),
                    // Move the low bits to the top and shift them right
                    Op::Swap(1),
                    Op::U32CheckedShrImm(rshift),
                    // OR the high and low bits together
                    Op::U32Or,
                ]);
            }
            2 if is_aligned => self.emit_all(&[
                // Load a quad-word
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
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the first two unused elements
                    Op::Drop,
                    Op::Drop,
                    // Shift the high bits left by the offset
                    Op::U32CheckedShlImm(ptr.offset as u32),
                    // Move the low bits to the top and shift them right
                    Op::Swap(1),
                    Op::U32CheckedShrImm(rshift),
                    // OR the high and low bits together
                    Op::U32Or,
                ]);
            }
            3 if is_aligned => self.emit_all(&[
                // Load a quad-word
                Op::MemLoadwImm(ptr.waddr),
                // Drop the three unused elements
                Op::Drop,
                Op::Drop,
                Op::Drop,
            ]),
            3 => {
                self.emit_all(&[
                    // Load the quad-word containing the low bits
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Move the element we need to the bottom temporarily
                    Op::Movdn(4),
                    // Drop the unused elements
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                    // Shift the low bits right by the offset
                    Op::U32CheckedShrImm(rshift),
                    // Load the quad-word containing the high bits
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the unused elements
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                    // Shift the high bits left by the offset
                    Op::U32CheckedShlImm(ptr.offset as u32),
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
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the unused element
                    Op::Drop,
                ]);
                self.realign_double_word(ptr);
            }
            2 if aligned => {
                self.emit_all(&[
                    // Load quad-word
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
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Move the element we need to the bottom temporarily
                    Op::Movdn(4),
                    // Drop the three unused elements of this word
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                    // Load the first quad-word
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
                    Op::MemLoadwImm(ptr.waddr + 1),
                    Op::Movup(4),
                    Op::Drop,
                    Op::Movup(3),
                    Op::Drop,
                    // Load first word, drop unused elements
                    Op::MemLoadwImm(ptr.waddr),
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                ]);
            }
            3 => {
                self.emit_all(&[
                    // Load second word, drop unused element
                    Op::MemLoadwImm(ptr.waddr + 1),
                    Op::Movup(4),
                    Op::Drop,
                    // Load first word, drop unused elements
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
            0 if aligned => self.emit(Op::MemLoadwImm(ptr.waddr)),
            0 => {
                // An unaligned quad-word load spans five elements
                self.emit_all(&[
                    // Load second quad-word
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Drop all but the first element
                    Op::Movdn(4),
                    Op::Drop,
                    Op::Drop,
                    Op::Drop,
                    // Load first quad-word
                    Op::MemLoadwImm(ptr.waddr),
                ]);
                self.realign_quad_word(ptr);
            }
            1 if aligned => {
                self.emit_all(&[
                    // Load second quad-word
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Drop last element
                    Op::Movup(4),
                    Op::Drop,
                    // Load first quad-word
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop first element
                    Op::Drop,
                ]);
            }
            1 => {
                // An unaligned double-word load spans five elements
                self.emit_all(&[
                    // Load second quad-word
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Drop all but the first two elements
                    Op::Movdn(4),
                    Op::Movdn(4),
                    Op::Drop,
                    Op::Drop,
                    // Load first quad-word
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop the first word
                    Op::Drop,
                ]);
                self.realign_quad_word(ptr);
            }
            2 if aligned => {
                self.emit_all(&[
                    // Load second quad-word
                    Op::MemLoadwImm(ptr.waddr),
                    // Drop last two elements
                    Op::Movup(4),
                    Op::Movup(4),
                    Op::Drop,
                    Op::Drop,
                    // Load first quad-word
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
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Drop the last element
                    Op::Movup(4),
                    Op::Drop,
                    // Load the first quad-word
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
                    Op::MemLoadwImm(ptr.waddr + 1),
                    Op::Movup(4),
                    Op::Drop,
                    // Load first word
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
                    Op::MemLoadwImm(ptr.waddr + 1),
                    // Load first word
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
    /// ```ignore
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
    /// As illustrated above, what should be a double-word value is occupying three words. To "realign" the
    /// value, i.e. ensure that it is naturally aligned and fits in two words, we have to perform a sequence
    /// of shifts and masks to get the bits where they belong. This function performs those steps, with the
    /// assumption that the caller has three values on the operand stack representing any unaligned double-word
    /// value
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
            Op::U32CheckedShlImm(ptr.offset as u32),
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
            Op::U32CheckedShrImm(32 - ptr.offset as u32),
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
            Op::U32CheckedShlImm(ptr.offset as u32),
            // Next, swap the low bit chunk to the top temporarily
            Op::Swap(1),
            // Shift the value right, as done previously for the middle chunk
            Op::U32CheckedShrImm(32 - ptr.offset as u32),
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
            Op::U32CheckedShlImm(ptr.offset as u32),
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
            Op::U32CheckedShrImm(32 - ptr.offset as u32),
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
            Op::U32CheckedShlImm(ptr.offset as u32),
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
            Op::U32CheckedShrImm(32 - ptr.offset as u32),
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
            Op::U32CheckedShlImm(ptr.offset as u32),
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
            Op::U32CheckedShrImm(32 - ptr.offset as u32),
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
            Op::U32CheckedShlImm(ptr.offset as u32),
            // Move the chunk containing the low bits to the top
            //
            // [chunk_lo, x_hi2, x_hi1, x_lo2, x_lo1_hi]
            Op::Movdn(5),
            // Shift the value right to get the low bits of `x_lo1`
            Op::U32CheckedShrImm(32 - ptr.offset as u32),
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
    /// Store a value of type `value` to the address in the Miden address space
    /// which corresponds to a pointer in the IR's byte-addressable address space.
    ///
    /// The type of the pointer is given as `ptr`, and can be used for both validation and
    /// determining alignment.
    pub fn store(&mut self) {
        let ptr = self.stack.pop().expect("operand stack is empty");
        let value = self.stack.pop().expect("operand stack is empty");
        let ptr_ty = ptr.ty();
        assert!(
            ptr_ty.is_pointer(),
            "expected load operand to be a pointer, got {ptr_ty}"
        );
        let value_ty = value.ty();
        assert!(
            !value_ty.is_zst(),
            "cannot store a zero-sized type in memory"
        );
        match ptr_ty {
            Type::Ptr(_) => {
                // Converet the pointer to a native pointer representation
                self.to_native_ptr();
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
        assert!(
            !value_ty.is_zst(),
            "cannot store a zero-sized type in memory"
        );
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

    /// Copy `count * sizeof(*ty)` from a source address to a destination address.
    ///
    /// The order of operands on the stack is `src`, `dst`, then `count`.
    ///
    /// The addresses on the stack are interpreted based on the pointer type: native pointers are
    /// in the Miden address space; non-native pointers are assumed to be in the IR's byte addressable
    /// address space, and require translation.
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
        assert_eq!(
            ty,
            dst.ty(),
            "expected src and dst operands to have the same type"
        );
        match ty {
            Type::Ptr(ref _pointee) => {
                todo!()
            }
            ty if !ty.is_pointer() => {
                panic!("invalid operand to memcpy: expected pointer, got {ty}")
            }
            ty => unimplemented!("memcpy support for pointers of type {ty} is not implemented"),
        }
    }

    fn store_quad_word(&mut self, _ptr: Option<NativePtr>) {
        todo!()
    }

    fn store_double_word(&mut self, _ptr: Option<NativePtr>) {
        todo!()
    }

    fn store_word(&mut self, _ptr: Option<NativePtr>) {
        todo!()
    }

    fn store_felt(&mut self, _ptr: Option<NativePtr>) {
        todo!()
    }

    fn store_small(&mut self, _ty: &Type, _ptr: Option<NativePtr>) {
        todo!()
    }

    fn store_array(&mut self, _element_ty: &Type, _ptr: Option<NativePtr>) {
        todo!()
    }

    fn store_struct(&mut self, _ty: &StructType, _ptr: Option<NativePtr>) {
        todo!()
    }
}
