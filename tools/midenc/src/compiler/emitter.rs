use miden_assembly::ast::{self as masm};

/// The emitter for MASM output.

pub enum MASMAst {
    Program(masm::ProgramAst),
    Module(masm::ModuleAst),
}

pub struct MASMEmitter {
    writer: BufferWriter,
}

impl miden_diagnostics::Emitter for MASMEmitter {

    fn buffer(&self) -> Buffer {
        self.writer.buffer()
    }

    fn print(&self, buffer: Buffer) -> std::io::Result<()> {
        self.writer.print(&buffer)
    }
}

impl MASMEmitter {

    pub fn new () -> Self {
        Self {
            writer: BufferWriter::new(ColorChoice::Auto),
        }
    }
}

impl Pass for Emitter {
    type Input = MASMAst;
    type Output = ();

    /// Runs the emitter on the AST 
    ///
    /// Errors should be reported via the registered error handler,
    /// Passes should return `Err` to signal that the pass has failed
    /// and compilation should be aborted
    fn run(&mut self, input: Self::Input) -> anyhow::Result<Self::Output> {
        self.writer.write!("{}", input);
    }

    /// Implementation of Pass::chain
    ///
    /// # Panics
    /// Panics if called. No chaining is possible after the emitter.
    fn chain<P>(self, pass: P) -> Chain<Self, P>
    where
        Self: Sized,
        P: for Pass<Input = Self::Output>
    {
        panic!("Attempting to chain a pass after the emitter.");
    }
}
