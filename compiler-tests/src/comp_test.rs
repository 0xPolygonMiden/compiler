/// Compile to different stages (e.g. Wasm, IR, MASM) and compare the results against expected output
pub struct CompTest {}

impl CompTest {
    /// Create a new instance with default configuration
    pub fn new() -> Self {
        todo!()
    }

    /// Set the Rust source code to compile
    pub fn rust_source(&mut self, _cargo_project: &str, _bin_bundle_namee: &str) -> &mut Self {
        todo!()
    }

    /// Compare the compiled Wasm against the expected output
    pub fn expect_wasm(&mut self, _expected_wat_file: expect_test::ExpectFile) -> &mut Self {
        todo!()
    }

    /// Compare the compiled IR against the expected output
    pub fn expect_ir(&mut self, _expected_mir_file: expect_test::ExpectFile) -> &mut Self {
        todo!()
    }

    /// Compare the compiled MASM against the expected output
    pub fn expect_masm(&mut self, _expected_masm_file: expect_test::ExpectFile) -> &mut Self {
        todo!()
    }

    /// Get the compiled MASM as [`miden_codegen_masm::Module`]
    pub fn codegen_masm_module(&self) -> miden_codegen_masm::Module {
        todo!()
    }

    /// Get the compiled MASM as [`miden_assembly::Module`]
    pub fn asm_masm_module(&self) -> miden_assembly::Module {
        todo!()
    }
}
