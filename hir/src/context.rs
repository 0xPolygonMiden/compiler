use core::ptr::NonNull;

use blink_alloc::Blink;
use cranelift_entity::{PrimaryMap, SecondaryMap};

use crate::*;

pub struct Context {
    pub allocator: Blink,
    pub ops: PrimaryMap<OpId, NonNull<dyn Op>>,
    pub regions: PrimaryMap<RegionId, NonNull<Region>>,
    pub blocks: PrimaryMap<Block, NonNull<BlockData>>,
    pub globals: PrimaryMap<GlobalValue, GlobalValueData>,
    pub locals: PrimaryMap<LocalId, Local>,
    pub values: PrimaryMap<Value, NonNull<ValueData>>,
    pub constants: ConstantPool,
}

impl Context {
    pub fn new() -> Self {
        let allocator = Blink::new();
        Self {
            allocator,
            ops: SecondaryMap::new(),
            regions: PrimaryMap::new(),
            blocks: PrimaryMap::new(),
            globals: PrimaryMap::new(),
            locals: PrimaryMap::new(),
            values: PrimaryMap::new(),
            constants: Default::default(),
        }
    }

    pub fn create_op<T: Op>(&mut self, mut op: T) -> OpId {
        let key = self.ops.next_key();
        let op = self.allocator.put(op);
        let ptr = op as *mut T;
        {
            let operation = op.as_operation_mut();
            operation.key = key;
            operation.vtable.set_data_ptr(ptr);
        }
        let op = unsafe { NonNull::new_unchecked(op) };
        self.ops.push(op.cast());
        key
    }

    pub fn op(&self, id: OpId) -> &dyn Op {
        self.ops[id].as_ref()
    }
}
