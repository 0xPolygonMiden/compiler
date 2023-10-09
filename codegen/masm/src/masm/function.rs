use std::fmt;

use cranelift_entity::{EntityRef, PrimaryMap};
use intrusive_collections::{intrusive_adapter, LinkedListLink};
use miden_hir::{FunctionIdent, Signature, Type};
use smallvec::{smallvec, SmallVec};

use super::*;

intrusive_adapter!(pub FunctionListAdapter = Box<Function>: Function { link: LinkedListLink });

/// This represents a function in Miden Assembly
pub struct Function {
    link: LinkedListLink,
    /// The name of this function
    pub name: FunctionIdent,
    /// The type signature of this function
    pub signature: Signature,
    /// The root block of code for this function
    pub body: BlockId,
    /// Storage for the blocks of code in this function's body
    pub blocks: PrimaryMap<BlockId, Block>,
    /// Locals allocated for this function
    locals: SmallVec<[Local; 1]>,
    /// The next available local index
    next_local_id: usize,
}
impl Function {
    pub fn new(name: FunctionIdent, signature: Signature) -> Self {
        let mut blocks = PrimaryMap::<BlockId, Block>::default();
        let body_id = blocks.next_key();
        let body = blocks.push(Block {
            id: body_id,
            ops: smallvec![],
        });
        Self {
            link: Default::default(),
            name,
            signature,
            body,
            blocks,
            locals: Default::default(),
            next_local_id: 0,
        }
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
        self.locals
            .iter()
            .find(|l| l.id == id)
            .expect("invalid local id")
    }

    /// Return the locals allocated in this function as a slice
    #[inline]
    pub fn locals(&self) -> &[Local] {
        self.locals.as_slice()
    }

    /// Allocate a new code block in this function
    pub fn create_block(&mut self) -> BlockId {
        let id = self.blocks.next_key();
        self.blocks.push(Block {
            id,
            ops: smallvec![],
        });
        id
    }

    #[inline]
    pub fn block(&self, id: BlockId) -> &Block {
        &self.blocks[id]
    }

    #[inline]
    pub fn block_mut(&mut self, id: BlockId) -> &mut Block {
        &mut self.blocks[id]
    }

    /// Return an implementation of [std::fmt::Display] for this function
    pub fn display<'a, 'b: 'a>(&'b self, imports: &'b ModuleImportInfo) -> DisplayMasmFunction<'a> {
        DisplayMasmFunction {
            function: self,
            imports,
        }
    }
}
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("body", &self.body)
            .field("blocks", &self.blocks)
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
        use miden_hir::DisplayMasmBlock;

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

        writeln!(
            f,
            "{}",
            DisplayMasmBlock::new(
                Some(self.imports),
                &self.function.blocks,
                self.function.body,
                1
            )
        )?;

        f.write_str("end")
    }
}
