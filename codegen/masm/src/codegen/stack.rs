use core::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

use midenc_hir::{Felt, FieldElement, Immediate, Type, Value};
use smallvec::{smallvec, SmallVec};

/// This represents a constraint an operand's usage at
/// a given program point, namely when used as an instruction
/// or block argument.
#[derive(Debug, Copy, Clone)]
pub enum Constraint {
    /// The operand should be moved, consuming it
    /// from the stack and making it unavailable for
    /// further use.
    Move,
    /// The operand should be copied, preserving the
    /// original value for later use.
    Copy,
}

/// A [TypedValue] is a pair of an SSA value with its known type
#[derive(Debug, Clone)]
pub struct TypedValue {
    pub value: Value,
    pub ty: Type,
}
impl Eq for TypedValue {}
impl PartialEq for TypedValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Ord for TypedValue {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialOrd for TypedValue {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}
impl Hash for TypedValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
impl AsRef<Value> for TypedValue {
    #[inline(always)]
    fn as_ref(&self) -> &Value {
        &self.value
    }
}
impl fmt::Display for TypedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", &self.value, &self.ty)
    }
}

/// A [ConstantValue] is either an immediate value, or a large immediate that has been
/// split into raw bytes.
#[derive(Clone)]
pub enum ConstantValue {
    Imm(Immediate),
    Bytes(SmallVec<[u8; 16]>),
}
impl ConstantValue {
    #[inline]
    pub fn ty(&self) -> Type {
        match self {
            Self::Imm(imm) => imm.ty(),
            Self::Bytes(ref bytes) => Type::Array(Box::new(Type::U8), bytes.len()),
        }
    }
}
impl Eq for ConstantValue {}
impl PartialEq for ConstantValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Imm(ref a), Self::Imm(ref b)) => a.cmp(b).is_eq(),
            (Self::Bytes(ref a), Self::Bytes(ref b)) => a == b,
            (..) => false,
        }
    }
}
impl PartialEq<Immediate> for ConstantValue {
    fn eq(&self, other: &Immediate) -> bool {
        match self {
            Self::Imm(ref a) => a.cmp(other).is_eq(),
            _ => false,
        }
    }
}
impl From<Immediate> for ConstantValue {
    fn from(imm: Immediate) -> Self {
        Self::Imm(imm)
    }
}
impl From<&[u8]> for ConstantValue {
    fn from(bytes: &[u8]) -> Self {
        Self::Bytes(SmallVec::from(bytes))
    }
}
impl fmt::Debug for ConstantValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Imm(ref imm) => fmt::Debug::fmt(imm, f),
            Self::Bytes(ref bytes) => {
                if !bytes.is_empty() {
                    write!(f, "Bytes(0x")?;
                    for b in bytes.iter().rev() {
                        write!(f, "{:02x}", b)?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
        }
    }
}

/// Represents the type of operand represented on the operand stack
#[derive(Clone)]
pub enum OperandType {
    /// The operand is a literal, unassociated with any value in the IR
    Const(ConstantValue),
    /// The operand is an SSA value of known type
    Value(TypedValue),
    /// The operand is an intermediate runtime value of a known type, but
    /// unassociated with any value in the IR
    Type(Type),
}
impl OperandType {
    /// Get the type representation of this operand
    pub fn ty(&self) -> Type {
        match self {
            Self::Const(imm) => imm.ty(),
            Self::Value(TypedValue { ref ty, .. }) => ty.clone(),
            Self::Type(ref ty) => ty.clone(),
        }
    }
}
impl fmt::Debug for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Const(value) => write!(f, "Const({value:?})"),
            Self::Value(value) => write!(f, "Value({value})"),
            Self::Type(ty) => write!(f, "Type({ty})"),
        }
    }
}
impl Eq for OperandType {}
impl PartialEq for OperandType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a == b,
            (Self::Value(_), _) | (_, Self::Value(_)) => false,
            (Self::Const(ref a), Self::Const(ref b)) => a == b,
            (Self::Const(_), _) | (_, Self::Const(_)) => false,
            (Self::Type(ref a), Self::Type(ref b)) => a == b,
        }
    }
}
impl PartialEq<Type> for OperandType {
    fn eq(&self, other: &Type) -> bool {
        match self {
            Self::Type(a) => a == other,
            _ => false,
        }
    }
}
impl PartialEq<Immediate> for OperandType {
    fn eq(&self, other: &Immediate) -> bool {
        match self {
            Self::Const(a) => a == other,
            _ => false,
        }
    }
}
impl PartialEq<Value> for OperandType {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Self::Value(this) => &this.value == other,
            _ => false,
        }
    }
}
impl From<TypedValue> for OperandType {
    fn from(value: TypedValue) -> Self {
        Self::Value(value)
    }
}
impl From<Type> for OperandType {
    fn from(ty: Type) -> Self {
        Self::Type(ty)
    }
}
impl From<bool> for OperandType {
    fn from(value: bool) -> Self {
        Self::Const(Immediate::I1(value).into())
    }
}
impl From<u8> for OperandType {
    fn from(value: u8) -> Self {
        Self::Const(Immediate::U8(value).into())
    }
}
impl From<u16> for OperandType {
    fn from(value: u16) -> Self {
        Self::Const(Immediate::U16(value).into())
    }
}
impl From<u32> for OperandType {
    fn from(value: u32) -> Self {
        Self::Const(Immediate::U32(value).into())
    }
}
impl From<u64> for OperandType {
    fn from(value: u64) -> Self {
        Self::Const(Immediate::U64(value).into())
    }
}
impl From<Felt> for OperandType {
    fn from(value: Felt) -> Self {
        Self::Const(Immediate::Felt(value).into())
    }
}
impl From<Immediate> for OperandType {
    fn from(value: Immediate) -> Self {
        Self::Const(value.into())
    }
}
impl From<&[u8]> for OperandType {
    fn from(value: &[u8]) -> Self {
        Self::Const(value.into())
    }
}

/// This type represents a logical operand on the stack, which may consist
/// of one or more "parts", up to a word in size, on the actual stack.
///
/// The [OperandStack] operates in terms of [Operand], but when emitting
/// Miden Assembly, we must know how to translate operand-oriented operations
/// into equivalent element-/word-oriented operations. This is accomplished
/// by tracking the low-level representation of a given operand in this struct.
#[derive(Debug, Clone)]
pub struct Operand {
    /// The section of stack corresponding to this operand, containing
    /// up to a full word of elements. No chunk will ever exceed a word
    /// in size. This field behaves like a miniature [OperandStack], i.e.
    /// elements are pushed and popped off the end to modify it.
    ///
    /// An operand is encoded on this stack in order of lowest
    /// addressed bytes first. For example, given a struct operand,
    /// the first field of the struct will be closest to the top of
    /// the stack.
    word: SmallVec<[Type; 4]>,
    /// The high-level operand represented by this item.
    ///
    /// If the operand stack is manipulated in such a way that the operand
    /// is torn apart, say one field of a struct is popped; then this will
    /// be set to a `Type` operand, representing what high-level information
    /// we have about the remaining parts of the original operand on the stack.
    operand: OperandType,
}
impl Default for Operand {
    fn default() -> Self {
        Self {
            word: smallvec![Type::Felt],
            operand: Felt::ZERO.into(),
        }
    }
}
impl PartialEq<Value> for Operand {
    #[inline(always)]
    fn eq(&self, other: &Value) -> bool {
        self.operand.eq(other)
    }
}
impl PartialEq<Immediate> for Operand {
    #[inline(always)]
    fn eq(&self, other: &Immediate) -> bool {
        self.operand.eq(other)
    }
}
impl PartialEq<Immediate> for &Operand {
    #[inline(always)]
    fn eq(&self, other: &Immediate) -> bool {
        self.operand.eq(other)
    }
}
impl PartialEq<Type> for Operand {
    #[inline(always)]
    fn eq(&self, other: &Type) -> bool {
        self.operand.eq(other)
    }
}
impl PartialEq<Type> for &Operand {
    #[inline(always)]
    fn eq(&self, other: &Type) -> bool {
        self.operand.eq(other)
    }
}
impl From<Immediate> for Operand {
    #[inline]
    fn from(imm: Immediate) -> Self {
        Self::new(imm.into())
    }
}
impl From<u32> for Operand {
    #[inline]
    fn from(imm: u32) -> Self {
        Self::new(Immediate::U32(imm).into())
    }
}
impl TryFrom<&Operand> for Value {
    type Error = ();

    fn try_from(operand: &Operand) -> Result<Self, Self::Error> {
        match operand.operand {
            OperandType::Value(TypedValue { value, .. }) => Ok(value),
            _ => Err(()),
        }
    }
}
#[cfg(test)]
impl TryFrom<&Operand> for Immediate {
    type Error = ();

    fn try_from(operand: &Operand) -> Result<Self, Self::Error> {
        match operand.operand {
            OperandType::Const(ConstantValue::Imm(ref imm)) => Ok(*imm),
            _ => Err(()),
        }
    }
}
#[cfg(test)]
impl TryFrom<&Operand> for Type {
    type Error = ();

    fn try_from(operand: &Operand) -> Result<Self, Self::Error> {
        match operand.operand {
            OperandType::Type(ref ty) => Ok(ty.clone()),
            _ => Err(()),
        }
    }
}
#[cfg(test)]
impl TryFrom<Operand> for Type {
    type Error = ();

    fn try_from(operand: Operand) -> Result<Self, Self::Error> {
        match operand.operand {
            OperandType::Type(ty) => Ok(ty),
            _ => Err(()),
        }
    }
}
impl From<Type> for Operand {
    #[inline]
    fn from(ty: Type) -> Self {
        Self::new(OperandType::Type(ty))
    }
}
impl From<TypedValue> for Operand {
    #[inline]
    fn from(value: TypedValue) -> Self {
        Self::new(OperandType::Value(value))
    }
}
impl Operand {
    pub fn new(operand: OperandType) -> Self {
        let ty = operand.ty();
        let mut word = ty.to_raw_parts().expect("invalid operand type");
        assert!(!word.is_empty(), "invalid operand: must be a sized type");
        assert!(word.len() <= 4, "invalid operand: must be smaller than or equal to a word");
        if word.len() > 1 {
            word.reverse();
        }
        Self { word, operand }
    }

    /// Get the size of this operand in field elements
    pub fn size(&self) -> usize {
        self.word.len()
    }

    /// Get the [OperandType] representing the value of this operand
    #[inline(always)]
    pub fn value(&self) -> &OperandType {
        &self.operand
    }

    /// Get this operand as a [Value]
    #[inline]
    pub fn as_value(&self) -> Option<Value> {
        self.try_into().ok()
    }

    /// Get the [Type] of this operand
    #[inline]
    pub fn ty(&self) -> Type {
        self.operand.ty()
    }

    /// Pop a single field element from the underlying stack segment corresponding to this operand,
    /// as a new [Operand].
    ///
    /// For operands that fit in a field element, this is equivalent to cloning the operand.
    /// For operands that are larger than a field element, the type will be split at the fourth
    /// byte, this may destroy the semantics of a higher-level type (i.e. a large integer might
    /// become a smaller one, or an array of raw bytes). It is assumed that the caller is doing
    /// this intentionally.
    #[allow(unused)]
    pub fn pop(&mut self) -> Operand {
        if self.word.len() == 1 {
            self.clone()
        } else {
            match &mut self.operand {
                OperandType::Const(ref mut imm) => match imm {
                    ConstantValue::Bytes(ref mut bytes) => {
                        assert!(bytes.len() > 4);
                        let taken = ConstantValue::Bytes(SmallVec::from(&bytes[0..4]));
                        let new_bytes = SmallVec::from(&bytes[4..]);
                        *bytes = new_bytes;
                        let ty = self.word.pop().unwrap();
                        Self {
                            word: smallvec![ty],
                            operand: OperandType::Const(taken),
                        }
                    }
                    ConstantValue::Imm(Immediate::U64(i)) => {
                        let lo = *i & (u32::MAX as u64);
                        let hi = *i & !(u32::MAX as u64);
                        *imm = ConstantValue::Imm(Immediate::U32(lo as u32));
                        let ty = self.word.pop().unwrap();
                        Self {
                            word: smallvec![ty],
                            operand: Immediate::U32((hi >> 32) as u32).into(),
                        }
                    }
                    ConstantValue::Imm(Immediate::I64(i)) => {
                        let i = *i as u64;
                        let lo = i & (u32::MAX as u64);
                        let hi = i & !(u32::MAX as u64);
                        *imm = ConstantValue::Imm(Immediate::U32(lo as u32));
                        let ty = self.word.pop().unwrap();
                        Self {
                            word: smallvec![ty],
                            operand: Immediate::U32((hi >> 32) as u32).into(),
                        }
                    }
                    ConstantValue::Imm(Immediate::F64(f)) => {
                        let bytes = f.to_le_bytes();
                        let hi = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                        let lo = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
                        *imm = ConstantValue::Imm(Immediate::U32(lo));
                        let ty = self.word.pop().unwrap();
                        Self {
                            word: smallvec![ty],
                            operand: Immediate::U32(hi).into(),
                        }
                    }
                    ConstantValue::Imm(Immediate::I128(i)) => {
                        let i = *i as u128;
                        let bytes = i.to_le_bytes();
                        let hi = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                        *imm = ConstantValue::Bytes(SmallVec::from(&bytes[4..]));
                        let ty = self.word.pop().unwrap();
                        Self {
                            word: smallvec![ty],
                            operand: Immediate::U32(hi).into(),
                        }
                    }
                    ConstantValue::Imm(_) => unreachable!(),
                },
                OperandType::Value(ref tv) => match tv.ty.clone().split(4) {
                    (ty, Some(rest)) => {
                        let operand = OperandType::Type(ty);
                        self.operand = OperandType::Type(rest);
                        let ty = self.word.pop().unwrap();
                        Self {
                            word: smallvec![ty],
                            operand,
                        }
                    }
                    (_, None) => unreachable!(),
                },
                OperandType::Type(ref ty) => match ty.clone().split(4) {
                    (ty, Some(rest)) => {
                        let operand = OperandType::Type(ty);
                        self.operand = OperandType::Type(rest);
                        let ty = self.word.pop().unwrap();
                        Self {
                            word: smallvec![ty],
                            operand,
                        }
                    }
                    (_, None) => unreachable!(),
                },
            }
        }
    }
}

/// This structure emulates the state of the VM's operand stack while
/// generating code from the SSA representation of a function.
///
/// In order to emit efficient and correct stack manipulation code, we must be able to
/// reason about where values are on the operand stack at a given program point. This
/// structure tracks what SSA values have been pushed on the operand stack, where they are
/// on the stack relative to the top, and whether a given stack slot aliases multiple
/// values.
///
/// In addition to the state tracked, this structure also has an API that mimics the
/// stack manipulation instructions we can emit in the code generator, so that as we
/// emit instructions and modify this structure at the same time, 1:1.
#[derive(Clone)]
pub struct OperandStack {
    stack: Vec<Operand>,
}
impl Default for OperandStack {
    fn default() -> Self {
        Self {
            stack: Vec::with_capacity(16),
        }
    }
}
impl OperandStack {
    /// Renames the `n`th operand from the top of the stack to `value`
    ///
    /// The type is assumed to remain unchanged
    pub fn rename(&mut self, n: usize, value: Value) {
        match &mut self[n].operand {
            OperandType::Value(TypedValue {
                value: ref mut prev_value,
                ..
            }) => {
                *prev_value = value;
            }
            prev => {
                let ty = prev.ty();
                *prev = OperandType::Value(TypedValue { value, ty });
            }
        }
    }

    /// Searches for the position on the stack containing the operand corresponding to `value`.
    ///
    /// NOTE: This function will panic if `value` is not on the stack
    pub fn find(&self, value: &Value) -> Option<usize> {
        self.stack.iter().rev().position(|v| v == value)
    }

    /// Returns true if the operand stack is empty
    #[allow(unused)]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Returns the number of field elements on the stack
    #[inline]
    pub fn raw_len(&self) -> usize {
        self.stack.iter().map(|operand| operand.size()).sum()
    }

    /// Returns the index in the actual runtime stack which corresponds to
    /// the first element of the operand at `index`.
    #[track_caller]
    pub fn effective_index(&self, index: usize) -> usize {
        assert!(
            index < self.stack.len(),
            "expected {} to be less than {}",
            index,
            self.stack.len()
        );

        self.stack.iter().rev().take(index).map(|o| o.size()).sum()
    }

    /// Returns the index in the actual runtime stack which corresponds to
    /// the last element of the operand at `index`.
    #[track_caller]
    pub fn effective_index_inclusive(&self, index: usize) -> usize {
        assert!(index < self.stack.len());

        self.stack.iter().rev().take(index + 1).map(|o| o.size()).sum::<usize>() - 1
    }

    /// Returns the number of operands on the stack
    #[inline]
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Returns the operand on top of the stack, without consuming it
    #[inline]
    pub fn peek(&self) -> Option<&Operand> {
        self.stack.last()
    }

    /// Returns the word on top of the stack, without consuming it
    ///
    /// NOTE: A word will always be 4 field elements, so if an operand is
    /// larger than a field element, it may be split into an appropriately-
    /// sized operand in order to fit.
    #[allow(unused)]
    #[inline]
    pub fn peekw(&self) -> Option<[Operand; 4]> {
        use core::mem::MaybeUninit;

        let end = self.stack.len().checked_sub(1)?;
        if self.raw_len() < 4 {
            return None;
        }

        let mut word = MaybeUninit::<[Operand; 4]>::uninit();
        let mut stack = self.stack[(end - 3)..].to_vec();
        let ptr = word.as_mut_ptr() as *mut Operand;
        let mut index = 0usize;
        while index < 4 {
            let mut elem = stack.pop().unwrap();
            let ptr = unsafe { ptr.add(index) };
            match elem.size() {
                1 => {
                    unsafe {
                        ptr.write(elem);
                    }
                    index += 1;
                }
                _ => {
                    let a = elem.pop();
                    unsafe {
                        ptr.write(a);
                    }
                    index += 1;
                    stack.push(elem);
                }
            }
        }

        Some(unsafe { MaybeUninit::assume_init(word) })
    }

    /// Pushes a word of zeroes on top of the stack
    #[allow(unused)]
    pub fn padw(&mut self) {
        let default = Operand {
            word: smallvec![Type::U32],
            operand: 0u32.into(),
        };
        self.stack.push(default.clone());
        self.stack.push(default.clone());
        self.stack.push(default.clone());
        self.stack.push(default);
    }

    /// Pushes an operand on top of the stack
    #[inline]
    pub fn push<V: Into<Operand>>(&mut self, value: V) {
        self.stack.push(value.into());
    }

    /// Pushes a word of operands on top of the stack
    ///
    /// NOTE: This function will panic if any of the operands are larger than a field element.
    #[inline]
    #[allow(unused)]
    pub fn pushw(&mut self, mut word: [Operand; 4]) {
        assert!(
            word.iter().all(|op| op.size() == 1),
            "a word must be exactly 4 field elements in size"
        );
        word.reverse();
        self.stack.extend(word);
    }

    /// Pops the operand on top of the stack
    #[inline]
    pub fn pop(&mut self) -> Option<Operand> {
        self.stack.pop()
    }

    /// Pops the first word on top of the stack
    ///
    /// NOTE: A word will always be 4 field elements, so if an operand is
    /// larger than a field element, it may be split into an appropriately-
    /// sized operand in order to fit.
    #[allow(unused)]
    pub fn popw(&mut self) -> Option<[Operand; 4]> {
        use core::mem::MaybeUninit;

        if self.raw_len() < 4 {
            return None;
        }

        let mut word = MaybeUninit::<[Operand; 4]>::uninit();
        let ptr = word.as_mut_ptr() as *mut Operand;
        let mut index = 0usize;
        while index < 4 {
            let mut elem = self.stack.pop().unwrap();
            let ptr = unsafe { ptr.add(index) };
            match elem.size() {
                1 => {
                    unsafe {
                        ptr.write(elem);
                    }
                    index += 1;
                }
                _ => {
                    let a = elem.pop();
                    unsafe {
                        ptr.write(a);
                    }
                    index += 1;
                    self.stack.push(elem);
                }
            }
        }

        Some(unsafe { MaybeUninit::assume_init(word) })
    }

    /// Drops the top operand on the stack
    pub fn drop(&mut self) {
        self.stack.pop().expect("operand stack is empty");
    }

    /// Drops the top word on the stack
    ///
    /// NOTE: A word will always be 4 field elements, so if an operand is
    /// larger than a field element, it may be split to accommodate the request.
    #[allow(unused)]
    pub fn dropw(&mut self) {
        assert!(self.raw_len() >= 4, "expected at least a word on the operand stack");
        let mut dropped = 0usize;
        while let Some(mut elem) = self.stack.pop() {
            let needed = 4 - dropped;
            let size = elem.size();
            dropped += size;
            match size {
                n if needed == n => break,
                n if needed < n => {
                    for _ in 0..needed {
                        elem.pop();
                    }
                    self.stack.push(elem);
                    break;
                }
                _ => continue,
            }
        }
    }

    /// Drops the top `n` operands on the stack
    #[inline]
    pub fn dropn(&mut self, n: usize) {
        let len = self.stack.len();
        assert!(n <= len, "unable to drop {} operands, operand stack only has {}", n, len);
        self.stack.truncate(len - n);
    }

    /// Duplicates the operand in the `n`th position on the stack
    ///
    /// If `n` is 0, duplicates the top of the stack.
    pub fn dup(&mut self, n: usize) {
        let operand = self[n].clone();
        self.stack.push(operand);
    }

    /// Swaps the `n`th operand from the top of the stack, with the top of the stack
    ///
    /// If `n` is 1, it swaps the first two operands on the stack.
    ///
    /// NOTE: This function will panic if `n` is 0, or out of bounds.
    pub fn swap(&mut self, n: usize) {
        assert_ne!(n, 0, "invalid swap, index must be in the range 1..=15");
        let len = self.stack.len();
        assert!(
            n < len,
            "invalid operand stack index ({}), only {} operands are available",
            n,
            len
        );
        let a = len - 1;
        let b = a - n;
        self.stack.swap(a, b);
    }

    /// Moves the `n`th operand to the top of the stack
    ///
    /// If `n` is 1, this is equivalent to `swap(1)`.
    ///
    /// NOTE: This function will panic if `n` is 0, or out of bounds.
    pub fn movup(&mut self, n: usize) {
        assert_ne!(n, 0, "invalid move, index must be in the range 1..=15");
        let len = self.stack.len();
        assert!(
            n < len,
            "invalid operand stack index ({}), only {} operands are available",
            n,
            len
        );
        // Pick the midpoint by counting backwards from the end
        let mid = len - (n + 1);
        // Split the stack, and rotate the half that
        // contains our desired value to place it on top.
        let (_, r) = self.stack.split_at_mut(mid);
        r.rotate_left(1);
    }

    /// Makes the operand on top of the stack, the `n`th operand on the stack
    ///
    /// If `n` is 1, this is equivalent to `swap(1)`.
    ///
    /// NOTE: This function will panic if `n` is 0, or out of bounds.
    pub fn movdn(&mut self, n: usize) {
        assert_ne!(n, 0, "invalid move, index must be in the range 1..=15");
        let len = self.stack.len();
        assert!(
            n < len,
            "invalid operand stack index ({}), only {} operands are available",
            n,
            len
        );
        // Split the stack so that the desired position is in the top half
        let mid = len - (n + 1);
        let (_, r) = self.stack.split_at_mut(mid);
        // Move all elements above the `n`th position up by one, moving the top element to the `n`th
        // position
        r.rotate_right(1);
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Operand> {
        self.stack.iter()
    }
}
impl Index<usize> for OperandStack {
    type Output = Operand;

    fn index(&self, index: usize) -> &Self::Output {
        let len = self.stack.len();
        assert!(
            index < len,
            "invalid operand stack index ({}): only {} operands are available",
            index,
            len
        );
        let effective_len: usize = self.stack.iter().rev().take(index + 1).map(|o| o.size()).sum();
        assert!(
            effective_len <= 16,
            "invalid operand stack index ({}): requires access to more than 16 elements, which is \
             not supported in Miden",
            index
        );
        &self.stack[len - index - 1]
    }
}
impl IndexMut<usize> for OperandStack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.stack.len();
        assert!(
            index < len,
            "invalid operand stack index ({}): only {} elements are available",
            index,
            len
        );
        let effective_len: usize = self.stack.iter().rev().take(index + 1).map(|o| o.size()).sum();
        assert!(
            effective_len <= 16,
            "invalid operand stack index ({}): requires access to more than 16 elements, which is \
             not supported in Miden",
            index
        );
        &mut self.stack[len - index - 1]
    }
}

impl fmt::Debug for OperandStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[derive(Debug)]
        #[allow(unused)]
        struct StackEntry<'a> {
            index: usize,
            value: &'a Operand,
        }

        f.debug_list()
            .entries(
                self.stack
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(index, value)| StackEntry { index, value }),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use midenc_hir::StructType;

    use super::*;

    #[test]
    fn operand_stack_homogenous_operand_sizes_test() {
        let mut stack = OperandStack::default();

        let zero = Immediate::U32(0);
        let one = Immediate::U32(1);
        let two = Immediate::U32(2);
        let three = Immediate::U32(3);
        let four = Immediate::U32(4);
        let five = Immediate::U32(5);
        let six = Immediate::U32(6);
        let seven = Immediate::U32(7);

        #[inline]
        fn as_imms(word: [Operand; 4]) -> [Immediate; 4] {
            [
                (&word[0]).try_into().unwrap(),
                (&word[1]).try_into().unwrap(),
                (&word[2]).try_into().unwrap(),
                (&word[3]).try_into().unwrap(),
            ]
        }

        #[inline]
        fn as_imm(operand: Operand) -> Immediate {
            (&operand).try_into().unwrap()
        }

        // push
        stack.push(zero);
        stack.push(one);
        stack.push(two);
        stack.push(three);
        assert_eq!(stack.len(), 4);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], two);
        assert_eq!(stack[2], one);
        assert_eq!(stack[3], zero);

        // peek
        assert_eq!(stack.peek().unwrap(), three);

        // peekw
        assert_eq!(stack.peekw().map(as_imms), Some([three, two, one, zero]));

        // dup
        stack.dup(0);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], three);
        assert_eq!(stack[2], two);
        assert_eq!(stack[3], one);
        assert_eq!(stack[4], zero);

        stack.dup(3);
        assert_eq!(stack.len(), 6);
        assert_eq!(stack[0], one);
        assert_eq!(stack[1], three);
        assert_eq!(stack[2], three);
        assert_eq!(stack[3], two);
        assert_eq!(stack[4], one);
        assert_eq!(stack[5], zero);

        // drop
        stack.drop();
        assert_eq!(stack.len(), 5);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], three);
        assert_eq!(stack[2], two);
        assert_eq!(stack[3], one);
        assert_eq!(stack[4], zero);

        // padw
        stack.padw();
        assert_eq!(stack.len(), 9);
        assert_eq!(stack[0], zero);
        assert_eq!(stack[1], zero);
        assert_eq!(stack[2], zero);
        assert_eq!(stack[3], zero);
        assert_eq!(stack[4], three);
        assert_eq!(stack[5], three);

        // popw
        assert_eq!(stack.popw().map(as_imms), Some([zero, zero, zero, zero]));
        assert_eq!(stack.len(), 5);

        // pushw
        stack.pushw([four.into(), five.into(), six.into(), seven.into()]);
        assert_eq!(stack.len(), 9);
        assert_eq!(stack[0], four);
        assert_eq!(stack[1], five);
        assert_eq!(stack[2], six);
        assert_eq!(stack[3], seven);
        assert_eq!(stack[4], three);
        assert_eq!(stack[5], three);

        // dropw
        stack.dropw();
        assert_eq!(stack.len(), 5);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], three);
        assert_eq!(stack[2], two);
        assert_eq!(stack[3], one);
        assert_eq!(stack[4], zero);

        // swap
        stack.swap(2);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack[0], two);
        assert_eq!(stack[1], three);
        assert_eq!(stack[2], three);
        assert_eq!(stack[3], one);
        assert_eq!(stack[4], zero);

        stack.swap(1);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], two);
        assert_eq!(stack[2], three);
        assert_eq!(stack[3], one);
        assert_eq!(stack[4], zero);

        // movup
        stack.movup(2);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], three);
        assert_eq!(stack[2], two);
        assert_eq!(stack[3], one);
        assert_eq!(stack[4], zero);

        // movdn
        stack.movdn(3);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], two);
        assert_eq!(stack[2], one);
        assert_eq!(stack[3], three);
        assert_eq!(stack[4], zero);

        // pop
        assert_eq!(stack.pop().map(as_imm), Some(three));
        assert_eq!(stack.len(), 4);
        assert_eq!(stack[0], two);
        assert_eq!(stack[1], one);
        assert_eq!(stack[2], three);
        assert_eq!(stack[3], zero);

        // dropn
        stack.dropn(2);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], zero);
    }

    #[test]
    fn operand_stack_values_test() {
        use midenc_hir::Value;
        let mut stack = OperandStack::default();

        let zero = Value::from_u32(0);
        let one = Value::from_u32(1);
        let two = Value::from_u32(2);
        let three = Value::from_u32(3);

        // push
        stack.push(TypedValue {
            value: zero,
            ty: Type::Ptr(Box::new(Type::U8)),
        });
        stack.push(TypedValue {
            value: one,
            ty: Type::Array(Box::new(Type::U8), 4),
        });
        stack.push(TypedValue {
            value: two,
            ty: Type::U32,
        });
        stack.push(TypedValue {
            value: three,
            ty: Type::Struct(StructType::new([Type::U64, Type::U8])),
        });
        assert_eq!(stack.len(), 4);
        assert_eq!(stack.raw_len(), 6);

        assert_eq!(stack.find(&zero), Some(3));
        assert_eq!(stack.find(&one), Some(2));
        assert_eq!(stack.find(&two), Some(1));
        assert_eq!(stack.find(&three), Some(0));

        // dup
        stack.dup(0);
        assert_eq!(stack.find(&three), Some(0));

        stack.dup(3);
        assert_eq!(stack.find(&one), Some(0));

        // drop
        stack.drop();
        assert_eq!(stack.find(&one), Some(3));
        assert_eq!(stack.find(&three), Some(0));
        assert_eq!(stack[1], three);

        // padw
        stack.padw();
        assert_eq!(stack.find(&one), Some(7));
        assert_eq!(stack.find(&three), Some(4));

        // rename
        let four = Value::from_u32(4);
        stack.rename(1, four);
        assert_eq!(stack.find(&four), Some(1));
        assert_eq!(stack.find(&three), Some(4));

        // pop
        let top = stack.pop().unwrap();
        assert_eq!((&top).try_into(), Ok(Immediate::U32(0)));
        assert_eq!(stack.find(&four), Some(0));
        assert_eq!(stack[1], Immediate::U32(0));
        assert_eq!(stack[2], Immediate::U32(0));
        assert_eq!(stack.find(&three), Some(3));

        // dropn
        stack.dropn(3);
        assert_eq!(stack.find(&four), None);
        assert_eq!(stack.find(&three), Some(0));
        assert_eq!(stack[1], three);
        assert_eq!(stack.find(&two), Some(2));
        assert_eq!(stack.find(&one), Some(3));
        assert_eq!(stack.find(&zero), Some(4));

        // swap
        stack.swap(3);
        assert_eq!(stack.find(&one), Some(0));
        assert_eq!(stack.find(&three), Some(1));
        assert_eq!(stack.find(&two), Some(2));
        assert_eq!(stack[3], three);

        stack.swap(1);
        assert_eq!(stack.find(&three), Some(0));
        assert_eq!(stack.find(&one), Some(1));
        assert_eq!(stack.find(&two), Some(2));
        assert_eq!(stack.find(&zero), Some(4));

        // movup
        stack.movup(2);
        assert_eq!(stack.find(&two), Some(0));
        assert_eq!(stack.find(&three), Some(1));
        assert_eq!(stack.find(&one), Some(2));
        assert_eq!(stack.find(&zero), Some(4));

        // movdn
        stack.movdn(3);
        assert_eq!(stack.find(&three), Some(0));
        assert_eq!(stack.find(&one), Some(1));
        assert_eq!(stack[2], three);
        assert_eq!(stack.find(&two), Some(3));
        assert_eq!(stack.find(&zero), Some(4));
    }

    #[test]
    fn operand_stack_heterogenous_operand_sizes_test() {
        let mut stack = OperandStack::default();

        let zero = Immediate::U32(0);
        let one = Immediate::U32(1);
        let two = Type::U64;
        let three = Type::U64;
        let struct_a =
            Type::Struct(StructType::new([Type::Ptr(Box::new(Type::U8)), Type::U16, Type::U32]));

        // push
        stack.push(zero);
        stack.push(one);
        stack.push(two.clone());
        stack.push(three.clone());
        stack.push(struct_a.clone());
        assert_eq!(stack.len(), 5);
        assert_eq!(stack.raw_len(), 9);
        assert_eq!(stack[0], struct_a);
        assert_eq!(stack[1], three);
        assert_eq!(stack[2], two);
        assert_eq!(stack[3], one);
        assert_eq!(stack[4], zero);

        // peek
        assert_eq!(stack.peek().unwrap(), struct_a);

        // peekw
        let word = stack.peekw().unwrap();
        let struct_parts = struct_a.clone().to_raw_parts().unwrap();
        let u64_parts = three.clone().to_raw_parts().unwrap();
        assert_eq!(struct_parts.len(), 3);
        assert_eq!(u64_parts.len(), 2);
        assert_eq!(&word[0], &struct_parts[0]);
        assert_eq!(&word[1], &struct_parts[1]);
        assert_eq!(&word[2], &struct_parts[2]);
        assert_eq!(&word[3], &u64_parts[0]);

        // dup
        stack.dup(0);
        assert_eq!(stack.len(), 6);
        assert_eq!(stack.raw_len(), 12);
        assert_eq!(stack[0], struct_a);
        assert_eq!(stack[1], struct_a);
        assert_eq!(stack[2], three);
        assert_eq!(stack[3], two);
        assert_eq!(stack[4], one);
        assert_eq!(stack[5], zero);
        assert_eq!(stack.effective_index(3), 8);

        stack.dup(3);
        assert_eq!(stack.len(), 7);
        assert_eq!(stack.raw_len(), 14);
        assert_eq!(stack[0], two);
        assert_eq!(stack[1], struct_a);
        assert_eq!(stack[2], struct_a);

        // drop
        stack.drop();
        assert_eq!(stack.len(), 6);
        assert_eq!(stack.raw_len(), 12);
        assert_eq!(stack[0], struct_a);
        assert_eq!(stack[1], struct_a);
        assert_eq!(stack[2], three);
        assert_eq!(stack[3], two);
        assert_eq!(stack[4], one);
        assert_eq!(stack[5], zero);

        // swap
        stack.swap(2);
        assert_eq!(stack.len(), 6);
        assert_eq!(stack.raw_len(), 12);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], struct_a);
        assert_eq!(stack[2], struct_a);
        assert_eq!(stack[3], two);
        assert_eq!(stack[4], one);

        stack.swap(1);
        assert_eq!(stack.len(), 6);
        assert_eq!(stack.raw_len(), 12);
        assert_eq!(stack[0], struct_a);
        assert_eq!(stack[1], three);
        assert_eq!(stack[2], struct_a);
        assert_eq!(stack[3], two);
        assert_eq!(stack[4], one);
        assert_eq!(stack[5], zero);

        // movup
        stack.movup(4);
        assert_eq!(stack.len(), 6);
        assert_eq!(stack.raw_len(), 12);
        assert_eq!(stack[0], one);
        assert_eq!(stack[1], struct_a);
        assert_eq!(stack[2], three);
        assert_eq!(stack[3], struct_a);
        assert_eq!(stack[4], two);
        assert_eq!(stack[5], zero);

        // movdn
        stack.movdn(3);
        assert_eq!(stack.len(), 6);
        assert_eq!(stack.raw_len(), 12);
        assert_eq!(stack[0], struct_a);
        assert_eq!(stack[1], three);
        assert_eq!(stack[2], struct_a);
        assert_eq!(stack[3], one);
        assert_eq!(stack[4], two);

        // pop
        let operand: Type = stack.pop().unwrap().try_into().unwrap();
        assert_eq!(operand, struct_a);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack.raw_len(), 9);
        assert_eq!(stack[0], three);
        assert_eq!(stack[1], struct_a);
        assert_eq!(stack[2], one);
        assert_eq!(stack[3], two);

        // dropn
        stack.dropn(2);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.raw_len(), 4);
        assert_eq!(stack[0], one);
        assert_eq!(stack[1], two);
        assert_eq!(stack[2], zero);
    }
}
