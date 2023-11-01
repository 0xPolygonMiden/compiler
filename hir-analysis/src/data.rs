use miden_hir::pass::{Analysis, AnalysisManager, AnalysisResult};
use miden_hir::{
    Function, FunctionIdent, GlobalValue, GlobalValueData, GlobalVariableTable, Program,
};
use midenc_session::Session;
use rustc_hash::FxHashMap;

/// This analysis calculates the addresses/offsets of all global variables in a [Program]
#[derive(Default)]
pub struct GlobalVariableAnalysis {
    global_table_offset: u32,
    offsets: FxHashMap<FunctionIdent, FxHashMap<GlobalValue, u32>>,
}
impl Analysis for GlobalVariableAnalysis {
    type Entity = Program;

    fn analyze(
        program: &Self::Entity,
        _analyses: &mut AnalysisManager,
        _session: &Session,
    ) -> AnalysisResult<Self> {
        let mut result = Self::default();

        result.global_table_offset = program.segments().next_available_offset();
        let globals = program.globals();
        for module in program.modules().iter() {
            for function in module.functions() {
                let mut function_offsets = FxHashMap::default();
                for gv in function.dfg.globals.keys() {
                    if let Some(addr) =
                        compute_global_value_addr(gv, result.global_table_offset, function, globals)
                    {
                        function_offsets.insert(gv, addr);
                    }
                }
                result.offsets.insert(function.id, function_offsets);
            }
        }

        Ok(result)
    }
}
impl GlobalVariableAnalysis {
    /// Get the address/offset at which global variables will start being allocated
    pub fn global_table_offset(&self) -> u32 {
        self.global_table_offset
    }

    /// Get the statically-allocated address at which the global value `gv` for `function` is stored.
    ///
    /// This function returns `None` if the analysis does not know about `function`, `gv`, or if
    /// the symbol which `gv` resolves to was undefined.
    pub fn get_computed_addr(&self, function: &FunctionIdent, gv: GlobalValue) -> Option<u32> {
        self.offsets
            .get(function)
            .and_then(|offsets| offsets.get(&gv).copied())
    }
}

/// Computes the absolute offset (address) represented by the given global value
fn compute_global_value_addr(
    mut gv: GlobalValue,
    global_table_offset: u32,
    function: &Function,
    globals: &GlobalVariableTable,
) -> Option<u32> {
    let mut relative_offset = 0;
    loop {
        let gv_data = function.dfg.global_value(gv);
        relative_offset += gv_data.offset();
        match gv_data {
            GlobalValueData::Symbol { name, .. } => {
                let var = globals.find(*name)?;
                let base_offset = unsafe { globals.offset_of(var) };
                if relative_offset >= 0 {
                    return Some((global_table_offset + base_offset) + relative_offset as u32);
                } else {
                    return Some(
                        (global_table_offset + base_offset) - relative_offset.unsigned_abs(),
                    );
                }
            }
            GlobalValueData::IAddImm { base, .. } => {
                gv = *base;
            }
            GlobalValueData::Load { base, .. } => {
                gv = *base;
            }
        }
    }
}
