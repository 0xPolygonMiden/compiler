use std::{ffi::OsStr, path::Path};

use miden_processor::{AdviceInputs, ExecutionOptions, StackInputs};

#[derive(Debug, Clone, Default)]
pub struct ProgramInputs {
    pub inputs: StackInputs,
    pub advice_inputs: AdviceInputs,
    pub options: ExecutionOptions,
}

impl clap::builder::ValueParserFactory for ProgramInputs {
    type Parser = ProgramInputsParser;

    fn value_parser() -> Self::Parser {
        ProgramInputsParser
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct ProgramInputsParser;
impl clap::builder::TypedValueParser for ProgramInputsParser {
    type Value = ProgramInputs;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::error::Error> {
        use clap::error::{Error, ErrorKind};

        let inputs_path = Path::new(value);
        if !inputs_path.is_file() {
            return Err(Error::raw(
                ErrorKind::InvalidValue,
                format!("invalid inputs file: '{}' is not a file", inputs_path.display()),
            ));
        }

        todo!()
    }
}
