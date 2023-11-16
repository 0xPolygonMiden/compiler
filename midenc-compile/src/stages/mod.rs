use super::Stage;
use crate::{CompilerError, CompilerResult};
use miden_codegen_masm as masm;
use miden_frontend_wasm as wasm;
use miden_hir::pass::{AnalysisManager, ConversionPass, RewritePass};
use miden_hir::{self as hir, parser::ast, PassInfo};
use midenc_session::Session;

mod codegen;
mod link;
mod parse;
mod rewrite;
mod sema;

pub use self::codegen::{CodegenStage, Compiled};
pub use self::link::{LinkerStage, MaybeLinked};
pub use self::parse::{ParseOutput, ParseStage};
pub use self::rewrite::ApplyRewritesStage;
pub use self::sema::SemanticAnalysisStage;
