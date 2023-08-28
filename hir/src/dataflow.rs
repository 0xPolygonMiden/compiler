use std::cell::{Ref, RefCell};
use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use cranelift_entity::{EntityRef, PrimaryMap, SecondaryMap};
use intrusive_collections::UnsafeRef;
use smallvec::SmallVec;

use miden_diagnostics::{SourceSpan, Span};

use super::*;

pub struct DataFlowGraph {
    pub signatures: Rc<RefCell<PrimaryMap<FuncRef, Signature>>>,
    pub callees: Rc<RefCell<BTreeMap<String, FuncRef>>>,
    pub blocks: OrderedArenaMap<Block, BlockData>,
    pub insts: ArenaMap<Inst, InstNode>,
    pub results: SecondaryMap<Inst, ValueList>,
    pub values: PrimaryMap<Value, ValueData>,
    pub value_lists: ValueListPool,
}
impl DataFlowGraph {
    pub fn new(
        signatures: Rc<RefCell<PrimaryMap<FuncRef, Signature>>>,
        callees: Rc<RefCell<BTreeMap<String, FuncRef>>>,
    ) -> Self {
        Self {
            signatures,
            callees,
            blocks: OrderedArenaMap::new(),
            insts: ArenaMap::new(),
            results: SecondaryMap::new(),
            values: PrimaryMap::new(),
            value_lists: ValueListPool::new(),
        }
    }

    #[cfg(test)]
    pub fn empty() -> Self {
        Self::new(
            Rc::new(RefCell::new(PrimaryMap::new())),
            Rc::new(RefCell::new(BTreeMap::new())),
        )
    }

    /// Returns the signature of the given function reference
    pub fn callee_signature(&self, callee: FuncRef) -> Ref<'_, Signature> {
        Ref::map(self.signatures.borrow(), |sigs| sigs.get(callee).unwrap())
    }

    /// Looks up the function reference for the given name
    pub fn get_callee(&self, name: &str) -> Option<FuncRef> {
        self.callees.borrow().get(name).copied()
    }

    /// Registers a function name as a callable function with the given signature
    pub fn register_callee(&self, name: String, signature: Signature) -> FuncRef {
        let mut callees = self.callees.borrow_mut();
        // Don't register duplicates
        if let Some(func) = callees.get(&name).copied() {
            return func;
        }
        let mut signatures = self.signatures.borrow_mut();
        let func = signatures.push(signature);
        callees.insert(name, func);
        func
    }

    pub fn make_value(&mut self, data: ValueData) -> Value {
        self.values.push(data)
    }

    pub fn value_type(&self, v: Value) -> Type {
        self.values[v].ty()
    }

    pub fn set_value_type(&mut self, v: Value, ty: Type) {
        self.values[v].set_type(ty)
    }

    pub fn get_value(&self, v: Value) -> ValueData {
        self.values[v].clone()
    }

    pub fn push_inst(&mut self, block: Block, data: Instruction, span: SourceSpan) -> Inst {
        let inst = self.insts.alloc_key();
        let node = InstNode::new(inst, block, Span::new(span, data));
        self.insts.append(inst, node);
        self.results.resize(inst.index() + 1);
        let item = unsafe { UnsafeRef::from_raw(&self.insts[inst]) };
        unsafe {
            self.block_data_mut(block).append(item);
        }
        inst
    }

    pub fn inst_args(&self, inst: Inst) -> &[Value] {
        self.insts[inst].arguments(&self.value_lists)
    }

    pub fn make_inst_results(&mut self, inst: Inst, ctrl_ty: Type) -> usize {
        self.results[inst].clear(&mut self.value_lists);

        let opcode = self.insts[inst].opcode();
        if let Some(fdata) = self.call_signature(inst) {
            let mut num_results = 0;
            for ty in fdata.results() {
                self.append_result(inst, ty.clone());
                num_results += 1;
            }
            num_results
        } else {
            let mut args = SmallVec::<[Type; 2]>::default();
            for arg in self.inst_args(inst) {
                args.push(self.value_type(*arg));
            }
            let mut results = opcode.results(ctrl_ty, args.as_slice());
            let num_results = results.len();
            for ty in results.drain(..) {
                self.append_result(inst, ty);
            }
            num_results
        }
    }

    /// Create a `ReplaceBuilder` that will replace `inst` with a new instruction in-place.
    pub fn replace(&mut self, inst: Inst) -> ReplaceBuilder {
        ReplaceBuilder::new(self, inst)
    }

    pub fn append_result(&mut self, inst: Inst, ty: Type) -> Value {
        let res = self.values.next_key();
        let num = self.results[inst].push(res, &mut self.value_lists);
        debug_assert!(num <= u16::MAX as usize, "too many result values");
        self.make_value(ValueData::Inst {
            ty,
            inst,
            num: num as u16,
        })
    }

    pub fn first_result(&self, inst: Inst) -> Value {
        self.results[inst]
            .first(&self.value_lists)
            .expect("instruction has no results")
    }

    pub fn has_results(&self, inst: Inst) -> bool {
        !self.results[inst].is_empty()
    }

    pub fn inst_results(&self, inst: Inst) -> &[Value] {
        self.results[inst].as_slice(&self.value_lists)
    }

    pub fn inst_block(&self, inst: Inst) -> Option<Block> {
        let inst_data = &self.insts[inst];
        if inst_data.link.is_linked() {
            Some(inst_data.block)
        } else {
            None
        }
    }

    pub fn pp_block(&self, pp: ProgramPoint) -> Block {
        match pp {
            ProgramPoint::Block(block) => block,
            ProgramPoint::Inst(inst) => self.inst_block(inst).expect("program point not in layout"),
        }
    }

    pub fn pp_cmp<A, B>(&self, a: A, b: B) -> core::cmp::Ordering
    where
        A: Into<ProgramPoint>,
        B: Into<ProgramPoint>,
    {
        let a = a.into();
        let b = b.into();
        debug_assert_eq!(self.pp_block(a), self.pp_block(b));
        let a_seq = match a {
            ProgramPoint::Block(_) => 0,
            ProgramPoint::Inst(inst) => {
                let block = self.insts[inst].block;
                self.blocks[block].insts().position(|i| i == inst).unwrap()
            }
        };
        let b_seq = match b {
            ProgramPoint::Block(_) => 0,
            ProgramPoint::Inst(inst) => {
                let block = self.insts[inst].block;
                self.blocks[block].insts().position(|i| i == inst).unwrap()
            }
        };
        a_seq.cmp(&b_seq)
    }

    pub fn call_signature(&self, inst: Inst) -> Option<Signature> {
        match self.insts[inst].analyze_call(&self.value_lists) {
            CallInfo::NotACall => None,
            CallInfo::Direct(f, _) => Some(self.callee_signature(f).clone()),
        }
    }

    pub fn analyze_call(&self, inst: Inst) -> CallInfo<'_> {
        self.insts[inst].analyze_call(&self.value_lists)
    }

    pub fn analyze_branch(&self, inst: Inst) -> BranchInfo {
        self.insts[inst].analyze_branch(&self.value_lists)
    }

    pub fn blocks<'f>(&'f self) -> impl Iterator<Item = (Block, &'f BlockData)> {
        Blocks {
            cursor: self.blocks.cursor(),
        }
    }

    pub fn entry_block(&self) -> Option<Block> {
        self.blocks.first().map(|b| b.key())
    }

    pub(super) fn last_block(&self) -> Option<Block> {
        self.blocks.last().map(|b| b.key())
    }

    pub fn num_blocks(&self) -> usize {
        self.blocks.iter().count()
    }

    pub fn block_args(&self, block: Block) -> &[Value] {
        self.blocks[block].params.as_slice(&self.value_lists)
    }

    pub fn block_insts<'f>(&'f self, block: Block) -> impl Iterator<Item = Inst> + 'f {
        self.blocks[block].insts()
    }

    pub fn block_data(&self, block: Block) -> &BlockData {
        &self.blocks[block]
    }

    pub fn block_data_mut(&mut self, block: Block) -> &mut BlockData {
        &mut self.blocks[block]
    }

    pub fn last_inst(&self, block: Block) -> Option<Inst> {
        self.blocks[block].last()
    }

    pub fn is_block_inserted(&self, block: Block) -> bool {
        self.blocks.contains(block)
    }

    pub fn is_block_empty(&self, block: Block) -> bool {
        self.blocks[block].is_empty()
    }

    pub fn make_block(&mut self) -> Block {
        self.blocks.push(BlockData::new())
    }

    pub fn remove_block(&mut self, block: Block) {
        self.blocks.remove(block);
    }

    pub fn num_block_params(&self, block: Block) -> usize {
        self.blocks[block].params.len(&self.value_lists)
    }

    pub fn block_params(&self, block: Block) -> &[Value] {
        self.blocks[block].params.as_slice(&self.value_lists)
    }

    pub fn block_param_types(&self, block: Block) -> Vec<Type> {
        self.block_params(block)
            .iter()
            .map(|&v| self.value_type(v))
            .collect()
    }

    pub fn append_block_param(&mut self, block: Block, ty: Type, span: SourceSpan) -> Value {
        let param = self.values.next_key();
        let num = self.blocks[block].params.push(param, &mut self.value_lists);
        debug_assert!(num <= u16::MAX as usize, "too many parameters on block");
        self.make_value(ValueData::Param {
            ty,
            num: num as u16,
            block,
            span,
        })
    }
}
impl Index<Inst> for DataFlowGraph {
    type Output = Span<Instruction>;

    fn index(&self, inst: Inst) -> &Self::Output {
        &self.insts[inst]
    }
}
impl IndexMut<Inst> for DataFlowGraph {
    fn index_mut(&mut self, inst: Inst) -> &mut Self::Output {
        &mut self.insts[inst]
    }
}

struct Blocks<'f> {
    cursor: intrusive_collections::linked_list::Cursor<'f, LayoutAdapter<Block, BlockData>>,
}
impl<'f> Iterator for Blocks<'f> {
    type Item = (Block, &'f BlockData);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_null() {
            return None;
        }
        let next = self.cursor.get().map(|data| (data.key(), data.value()));
        self.cursor.move_next();
        next
    }
}
