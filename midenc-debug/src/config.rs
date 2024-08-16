use std::{
    collections::BTreeMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use miden_processor::{AdviceInputs, ExecutionOptions, Felt as RawFelt, StackInputs};
use serde::Deserialize;

use crate::Felt;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(try_from = "DebuggerConfigFile")]
pub struct DebuggerConfig {
    pub inputs: StackInputs,
    pub advice_inputs: AdviceInputs,
    pub options: ExecutionOptions,
}

impl TryFrom<DebuggerConfigFile> for DebuggerConfig {
    type Error = String;

    #[inline]
    fn try_from(mut file: DebuggerConfigFile) -> Result<Self, Self::Error> {
        Self::from_inputs_file(file, None)
    }
}

impl DebuggerConfig {
    pub fn parse_file<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;

        let file = toml::from_str::<DebuggerConfigFile>(&content).map_err(std::io::Error::other)?;
        Self::from_inputs_file(file, path.parent().map(|p| p.to_path_buf()))
            .map_err(std::io::Error::other)
    }

    pub fn parse_str(content: &str) -> Result<Self, String> {
        let file = toml::from_str::<DebuggerConfigFile>(content).map_err(|err| err.to_string())?;

        Self::from_inputs_file(file, None)
    }

    fn from_inputs_file(
        mut file: DebuggerConfigFile,
        cwd: Option<PathBuf>,
    ) -> Result<Self, String> {
        let inputs = StackInputs::new(file.inputs.stack.into_iter().map(|felt| felt.0).collect())
            .map_err(|err| format!("invalid value for 'stack': {err}"))?;
        let mut advice_inputs = AdviceInputs::default()
            .with_stack(file.inputs.advice.stack.into_iter().rev().map(|felt| felt.0))
            .with_map(file.inputs.advice.map.into_iter().map(|entry| {
                (entry.digest.0, entry.values.into_iter().map(|felt| felt.0).collect::<Vec<_>>())
            }));

        for segment in file.inputs.rodata {
            let data = if let Some(path) = segment.path {
                let path = if let Some(cwd) = cwd.as_ref() {
                    if path.is_relative() {
                        cwd.join(path)
                    } else {
                        path
                    }
                } else {
                    path
                };
                decode_rodata_from_path(&path)?
            } else {
                decode_rodata(&segment.data)?
            };
            advice_inputs.extend_map([(segment.digest.0, data)]);
        }

        Ok(Self {
            inputs,
            advice_inputs,
            options: file.options,
        })
    }
}

fn decode_rodata_from_path(path: &Path) -> Result<Vec<RawFelt>, String> {
    let data = std::fs::read(path)
        .map_err(|err| format!("failed to read rodata from '{}': {err}", path.display()))?;

    decode_rodata(data.as_slice())
}

fn decode_rodata(data: &[u8]) -> Result<Vec<RawFelt>, String> {
    let mut felts = Vec::with_capacity(data.len() / 4);
    let mut iter = data.iter().copied().array_chunks::<4>();
    felts.extend(iter.by_ref().map(|bytes| RawFelt::new(u32::from_be_bytes(bytes) as u64)));
    if let Some(remainder) = iter.into_remainder() {
        let mut chunk = [0u8; 4];
        for (i, byte) in remainder.into_iter().enumerate() {
            chunk[i] = byte;
        }
        felts.push(RawFelt::new(u32::from_be_bytes(chunk) as u64));
    }

    Ok(felts)
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct DebuggerConfigFile {
    inputs: Inputs,
    #[serde(deserialize_with = "deserialize_execution_options")]
    options: ExecutionOptions,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct Inputs {
    /// The rodata segments to place in the advice map
    rodata: Vec<DataSegment>,
    /// The contents of the operand stack, top is leftmost
    stack: Vec<crate::Felt>,
    /// The inputs to the advice provider
    advice: Advice,
}

#[derive(Debug, Clone, Deserialize)]
struct DataSegment {
    /// The commitment digest for this segment
    digest: Digest,
    /// A path to the file containing the raw binary data for this segment
    #[serde(default)]
    path: Option<PathBuf>,
    /// The raw data for this segment (mutually exclusive with `path`)
    #[serde(default, deserialize_with = "deserialize_rodata_bytes")]
    data: Vec<u8>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct Advice {
    /// The contents of the advice stack, top is leftmost
    stack: Vec<crate::Felt>,
    /// Entries to populate the advice map with
    map: Vec<AdviceMapEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct AdviceMapEntry {
    digest: Digest,
    /// Values that will be pushed to the advice stack when this entry is requested
    #[serde(default)]
    values: Vec<crate::Felt>,
}

impl clap::builder::ValueParserFactory for DebuggerConfig {
    type Parser = DebuggerConfigParser;

    fn value_parser() -> Self::Parser {
        DebuggerConfigParser
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct DebuggerConfigParser;
impl clap::builder::TypedValueParser for DebuggerConfigParser {
    type Value = DebuggerConfig;

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

        let content = std::fs::read_to_string(inputs_path).map_err(|err| {
            Error::raw(ErrorKind::ValueValidation, format!("failed to read inputs file: {err}"))
        })?;
        let inputs_file = toml::from_str::<DebuggerConfigFile>(&content).map_err(|err| {
            Error::raw(ErrorKind::ValueValidation, format!("invalid inputs file: {err}"))
        })?;

        DebuggerConfig::from_inputs_file(inputs_file, Some(inputs_path.to_path_buf())).map_err(
            |err| Error::raw(ErrorKind::ValueValidation, format!("invalid inputs file: {err}")),
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Digest(miden_processor::Digest);
impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let digest = String::deserialize(deserializer)?;
        miden_processor::Digest::try_from(&digest)
            .map_err(|err| serde::de::Error::custom(format!("invalid digest: {err}")))
            .map(Self)
    }
}

fn deserialize_rodata_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use midenc_hir::ConstantData;

    String::deserialize(deserializer).and_then(|hex| {
        ConstantData::from_str_be(hex.as_str())
            .map_err(|err| serde::de::Error::custom(format!("invalid rodata: {err}")))
            .map(Vec::<u8>::from)
    })
}

fn deserialize_execution_options<'de, D>(deserializer: D) -> Result<ExecutionOptions, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Default, Deserialize)]
    #[serde(default)]
    struct ExecOptions {
        max_cycles: Option<u32>,
        expected_cycles: u32,
    }

    ExecOptions::deserialize(deserializer).and_then(|opts| {
        ExecutionOptions::new(
            opts.max_cycles,
            opts.expected_cycles,
            /* enable_tracing= */ true,
        )
        .map(|exec_opts| exec_opts.with_debugging())
        .map_err(|err| serde::de::Error::custom(format!("invalid execution options: {err}")))
    })
}

#[cfg(test)]
mod tests {
    use toml::toml;

    use super::*;

    #[test]
    fn debugger_config_empty() {
        let text = toml::to_string_pretty(&toml! {
            [inputs]
            [options]
        })
        .unwrap();

        let file = toml::from_str::<DebuggerConfig>(&text).unwrap();
        assert!(file.inputs.values().is_empty());
        assert!(file.advice_inputs.stack().is_empty());
        assert_eq!(file.options.enable_tracing(), true);
        assert_eq!(file.options.enable_debugging(), true);
        assert_eq!(file.options.max_cycles(), u32::MAX);
        assert_eq!(file.options.expected_cycles(), 64);
    }

    #[test]
    fn debugger_config_with_options() {
        let text = toml::to_string_pretty(&toml! {
            [inputs]
            [options]
            max_cycles = 1000
        })
        .unwrap();

        let file = DebuggerConfig::parse_str(&text).unwrap();
        assert!(file.inputs.values().is_empty());
        assert!(file.advice_inputs.stack().is_empty());
        assert_eq!(file.options.enable_tracing(), true);
        assert_eq!(file.options.enable_debugging(), true);
        assert_eq!(file.options.max_cycles(), 1000);
        assert_eq!(file.options.expected_cycles(), 64);
    }

    #[test]
    fn debugger_config_with_operands() {
        let text = toml::to_string_pretty(&toml! {
            [inputs]
            stack = [1, 2, 3]

            [options]
            max_cycles = 1000
        })
        .unwrap();

        let file = DebuggerConfig::parse_str(&text).unwrap();
        assert_eq!(file.inputs.values(), &[RawFelt::new(3), RawFelt::new(2), RawFelt::new(1)]);
        assert!(file.advice_inputs.stack().is_empty());
        assert_eq!(file.options.enable_tracing(), true);
        assert_eq!(file.options.enable_debugging(), true);
        assert_eq!(file.options.max_cycles(), 1000);
        assert_eq!(file.options.expected_cycles(), 64);
    }

    #[test]
    fn debugger_config_with_advice() {
        let text = toml::to_string_pretty(&toml! {
            [inputs]
            stack = [1, 2, 0x3]

            [inputs.advice]
            stack = [1, 2, 3, 4]

            [[inputs.advice.map]]
            digest = "0x3cff5b58a573dc9d25fd3c57130cc57e5b1b381dc58b5ae3594b390c59835e63"
            values = [1, 2, 3, 4]

            [options]
            max_cycles = 1000
        })
        .unwrap();
        let digest = miden_processor::Digest::try_from(
            "0x3cff5b58a573dc9d25fd3c57130cc57e5b1b381dc58b5ae3594b390c59835e63",
        )
        .unwrap();
        let file = DebuggerConfig::parse_str(&text).unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(file.inputs.values(), &[RawFelt::new(3), RawFelt::new(2), RawFelt::new(1)]);
        assert_eq!(
            file.advice_inputs.stack(),
            &[RawFelt::new(4), RawFelt::new(3), RawFelt::new(2), RawFelt::new(1)]
        );
        assert_eq!(
            file.advice_inputs.mapped_values(&digest),
            Some([RawFelt::new(1), RawFelt::new(2), RawFelt::new(3), RawFelt::new(4)].as_slice())
        );
        assert_eq!(file.options.enable_tracing(), true);
        assert_eq!(file.options.enable_debugging(), true);
        assert_eq!(file.options.max_cycles(), 1000);
        assert_eq!(file.options.expected_cycles(), 64);
    }

    #[test]
    fn debugger_config_with_rodata() {
        const RODATA_SAMPLE: &[u8] = "hello world\0data\0strings\nü".as_bytes();

        let rodata_cwd = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let rodata_path = rodata_cwd.join("testdata").join("rodata-sample.bin");
        let text = toml::to_string_pretty(&toml! {
            [inputs]
            stack = [1, 2, 3]

            [[inputs.rodata]]
            digest = "0x2786346021744030bf0b9eb930712993609fb0425f7bda70e38ffc23c2f11df2"
            path = "testdata/rodata-sample.bin"

            [inputs.advice]
            stack = [1, 2, 3, 4]

            [options]
            max_cycles = 1000
        })
        .unwrap();

        let mut expected = decode_rodata(RODATA_SAMPLE).unwrap();
        let digest = miden_processor::crypto::Rpo256::hash_elements(&expected);
        assert_eq!(expected[0], RawFelt::new(u32::from_be_bytes([b'h', b'e', b'l', b'l']) as u64));

        // Bypass parse_str so that we can specify the working directory context
        let file =
            toml::from_str::<DebuggerConfigFile>(&text).unwrap_or_else(|err| panic!("{err}"));
        let file = DebuggerConfig::from_inputs_file(file, Some(rodata_cwd.to_path_buf())).unwrap();

        assert_eq!(file.inputs.values(), &[RawFelt::new(3), RawFelt::new(2), RawFelt::new(1)]);
        assert_eq!(file.advice_inputs.stack().len(), 4);
        assert_eq!(
            file.advice_inputs.stack(),
            &[RawFelt::new(4), RawFelt::new(3), RawFelt::new(2), RawFelt::new(1)]
        );
        assert_eq!(file.advice_inputs.mapped_values(&digest), Some(expected.as_slice()));
    }
}
