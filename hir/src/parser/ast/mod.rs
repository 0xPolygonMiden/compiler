mod block;
mod convert;
mod functions;
mod globals;
mod instruction;
mod region;

use alloc::collections::BTreeMap;
use core::fmt;

pub use self::{
    block::*, convert::ConvertAstToHir, functions::*, globals::*, instruction::*, region::*,
};
use crate::{
    diagnostics::{DiagnosticsHandler, Severity, SourceSpan, Span, Spanned},
    ExternalFunction, FunctionIdent, Ident,
};

/// This represents the parsed contents of a single Miden IR module
#[derive(Spanned)]
pub struct Module {
    #[span]
    pub span: SourceSpan,
    pub name: Ident,
    pub constants: Vec<ConstantDeclaration>,
    pub global_vars: Vec<GlobalVarDeclaration>,
    pub data_segments: Vec<DataSegmentDeclaration>,
    pub functions: Vec<FunctionDeclaration>,
    pub externals: Vec<Span<ExternalFunction>>,
    pub is_kernel: bool,
}
impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Module")
            .field("name", &self.name.as_symbol())
            .field("constants", &self.constants)
            .field("global_vars", &self.global_vars)
            .field("data_segments", &self.data_segments)
            .field("functions", &self.functions)
            .field("externals", &self.externals)
            .field("is_kernel", &self.is_kernel)
            .finish()
    }
}
impl midenc_session::Emit for Module {
    fn name(&self) -> Option<crate::Symbol> {
        Some(self.name.as_symbol())
    }

    fn output_type(&self, _mode: midenc_session::OutputMode) -> midenc_session::OutputType {
        midenc_session::OutputType::Ast
    }

    fn write_to<W: std::io::Write>(
        &self,
        mut writer: W,
        mode: midenc_session::OutputMode,
        _session: &midenc_session::Session,
    ) -> std::io::Result<()> {
        assert_eq!(
            mode,
            midenc_session::OutputMode::Text,
            "binary mode is not supported for HIR syntax trees"
        );
        writer.write_fmt(format_args!("{:#?}", self))
    }
}

type ConstantsById = BTreeMap<crate::Constant, Span<crate::ConstantData>>;
type RemappedConstants = BTreeMap<crate::Constant, crate::Constant>;
type GlobalVariablesById = BTreeMap<crate::GlobalVariable, Span<crate::GlobalVariableData>>;
type ImportsById = BTreeMap<FunctionIdent, Span<crate::ExternalFunction>>;
type RegionsById = BTreeMap<crate::RegionId, crate::Region>;
type BlocksById = BTreeMap<crate::Block, Block>;
type LocalsById = BTreeMap<crate::LocalId, Span<crate::Local>>;
type ValuesById = BTreeMap<crate::Value, Span<crate::ValueData>>;
type InstResults = BTreeMap<crate::Inst, Vec<crate::Value>>;

impl Module {
    pub fn new(span: SourceSpan, name: Ident, is_kernel: bool, forms: Vec<Form>) -> Self {
        let mut module = Self {
            span,
            name,
            constants: vec![],
            functions: vec![],
            externals: vec![],
            global_vars: vec![],
            data_segments: vec![],
            is_kernel,
        };
        for form in forms.into_iter() {
            match form {
                Form::Constant(constant) => {
                    module.constants.push(constant);
                }
                Form::Global(global) => {
                    module.global_vars.push(global);
                }
                Form::DataSegment(segment) => {
                    module.data_segments.push(segment);
                }
                Form::Function(function) => {
                    module.functions.push(function);
                }
                Form::ExternalFunction(external) => {
                    module.externals.push(external);
                }
            }
        }
        module
    }

    fn take_and_validate_constants(
        &mut self,
        diagnostics: &DiagnosticsHandler,
    ) -> (ConstantsById, bool) {
        use alloc::collections::btree_map::Entry;

        let mut constants_by_id = ConstantsById::default();
        let constants = core::mem::take(&mut self.constants);
        let mut is_valid = true;
        for constant in constants.into_iter() {
            match constants_by_id.entry(constant.id) {
                Entry::Vacant(entry) => {
                    entry.insert(Span::new(constant.span, constant.init));
                }
                Entry::Occupied(entry) => {
                    let prev = entry.get().span();
                    diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("invalid constant declaration")
                        .with_primary_label(
                            constant.span,
                            "a constant with this identifier has already been declared",
                        )
                        .with_secondary_label(prev, "previously declared here")
                        .emit();
                    is_valid = false;
                }
            }
        }

        (constants_by_id, is_valid)
    }

    fn take_and_validate_globals(
        &mut self,
        remapped_constants: &RemappedConstants,
        diagnostics: &DiagnosticsHandler,
    ) -> (GlobalVariablesById, bool) {
        use alloc::collections::btree_map::Entry;

        let mut globals_by_id = GlobalVariablesById::default();
        let global_vars = core::mem::take(&mut self.global_vars);
        let mut is_valid = true;
        for global in global_vars.into_iter() {
            match globals_by_id.entry(global.id) {
                Entry::Vacant(entry) => {
                    if let Some(id) = global.init {
                        if !remapped_constants.contains_key(&id) {
                            let id = id.as_u32();
                            diagnostics
                                .diagnostic(Severity::Error)
                                .with_message("invalid global variable declaration")
                                .with_primary_label(
                                    global.span,
                                    format!(
                                        "invalid initializer: no constant named '{id}' in this \
                                         module"
                                    ),
                                )
                                .emit();
                            is_valid = false;
                        }
                    }
                    let gv = crate::GlobalVariableData::new(
                        global.id,
                        global.name,
                        global.ty,
                        global.linkage,
                        global.init.map(|id| remapped_constants[&id]),
                    );
                    entry.insert(Span::new(global.span, gv));
                }
                Entry::Occupied(entry) => {
                    let prev = entry.get().span();
                    diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("invalid global variable declaration")
                        .with_primary_label(
                            global.span,
                            "a global variable with the same id has already been declared",
                        )
                        .with_secondary_label(prev, "previously declared here")
                        .emit();
                    is_valid = false;
                }
            }
        }

        (globals_by_id, is_valid)
    }

    fn take_and_validate_imports(
        &mut self,
        diagnostics: &DiagnosticsHandler,
    ) -> (ImportsById, bool) {
        use alloc::collections::btree_map::Entry;

        let mut imports_by_id = ImportsById::default();
        let mut is_valid = true;
        for external in core::mem::take(&mut self.externals).into_iter() {
            if external.id.module == self.name {
                diagnostics
                    .diagnostic(Severity::Error)
                    .with_message("invalid external function declaration")
                    .with_primary_label(
                        external.span(),
                        "external function declarations may not reference functions in the \
                         current module",
                    )
                    .emit();
                is_valid = false;
                continue;
            }

            match imports_by_id.entry(external.id) {
                Entry::Vacant(entry) => {
                    entry.insert(external);
                }
                Entry::Occupied(entry) => {
                    let prev = entry.get().span();
                    diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("invalid external function declaration")
                        .with_primary_label(
                            external.span(),
                            "an external function with the same name has already been declared",
                        )
                        .with_secondary_label(prev, "previously declared here")
                        .emit();
                    is_valid = false;
                }
            }
        }

        (imports_by_id, is_valid)
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.is_kernel == other.is_kernel
            && self.global_vars == other.global_vars
            && self.data_segments == other.data_segments
            && self.functions == other.functions
            && self.externals == other.externals
    }
}

/// This represents one of the top-level forms which a [Module] can contain
#[derive(Debug)]
pub enum Form {
    Constant(ConstantDeclaration),
    Global(GlobalVarDeclaration),
    DataSegment(DataSegmentDeclaration),
    Function(FunctionDeclaration),
    ExternalFunction(Span<ExternalFunction>),
}
