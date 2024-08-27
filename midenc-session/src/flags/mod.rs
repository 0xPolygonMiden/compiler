mod flag;

use alloc::vec::Vec;
use core::fmt;

pub use self::flag::{CompileFlag, FlagAction};
use crate::diagnostics::{IntoDiagnostic, Report};

#[cfg(feature = "std")]
pub struct CompileFlags {
    flags: Vec<CompileFlag>,
    arg_matches: clap::ArgMatches,
}

#[cfg(not(feature = "std"))]
pub struct CompileFlags {
    flags: Vec<CompileFlag>,
    args: alloc::collections::BTreeMap<String, MatchedArg>,
}

#[cfg(feature = "std")]
impl Default for CompileFlags {
    fn default() -> Self {
        Self::new(None::<alloc::string::String>).unwrap()
    }
}

#[cfg(not(feature = "std"))]
impl Default for CompileFlags {
    fn default() -> Self {
        Self::new(None::<String>)
    }
}

#[cfg(feature = "std")]
impl From<clap::ArgMatches> for CompileFlags {
    fn from(arg_matches: clap::ArgMatches) -> Self {
        let flags = inventory::iter::<CompileFlag>.into_iter().cloned().collect();
        Self { flags, arg_matches }
    }
}

impl CompileFlags {
    /// Create a new [CompileFlags] from the given argument vector
    #[cfg(feature = "std")]
    pub fn new<I, V>(argv: I) -> Result<Self, Report>
    where
        I: IntoIterator<Item = V>,
        V: Into<std::ffi::OsString> + Clone,
    {
        let flags = inventory::iter::<CompileFlag>.into_iter().cloned().collect();
        fake_compile_command()
            .try_get_matches_from(argv)
            .into_diagnostic()
            .map(|arg_matches| Self { flags, arg_matches })
    }

    /// Get [clap::ArgMatches] for registered command-line flags, without a [clap::Command]
    #[cfg(not(feature = "std"))]
    pub fn new<I, V>(argv: I) -> Result<Self, Report>
    where
        I: IntoIterator<Item = V>,
        V: Into<String> + Clone,
    {
        use alloc::collections::{BTreeMap, VecDeque};
        let mut argv = argv.into_iter().map(|arg| arg.into()).collect::<VecDeque<String>>();

        let flags = inventory::iter::<CompileFlag>
            .into_iter()
            .map(|flag| (flag.name.clone(), flag))
            .collect::<BTreeMap<_, _>>();
        let this = Self {
            flags: flags.values().cloned().collect(),
            arg_matches: Default::default(),
        };
        let mut this = flags.values().fold(this, |this, flag| {
            if let Some(default_value) = flag.default_value {
                this.arg_matches.insert(flag.name, Some(vec![default_value]));
            } else {
                this.arg_matches.insert(flag.name, None);
            }
        });

        while let Some(arg) = argv.pop_front() {
            let Some(name) = arg.strip_prefix("--").or_else(|| {
                arg.strip_prefix("-").and_then(|name| {
                    flags.values().find_map(|flag| {
                        if flag.short == name {
                            Some(flag.name.clone())
                        } else {
                            None
                        }
                    })
                })
            }) else {
                return Err(Report::msg(format!("unexpected positional argument: '{arg}'")));
            };
            if let Some(flag) = flags.get(&name) {
                match flag.action {
                    FlagAction::Set => {
                        let value = argv
                            .pop_front()
                            .or_else(flag.default_missing_value.clone())
                            .or_else(flag.default_value.clone());
                        if let Some(value) = value {
                            this.arg_matches.insert(name.to_string(), Some(vec![value]))
                        } else {
                            return Err(Report::msg(format!(
                                "missing required value for '--{name}'"
                            )));
                        }
                    }
                    FlagAction::Count => {
                        match this.arg_matches.entry(name.to_string()).or_insert_default() {
                            Some(ref mut values) => {
                                values.push("".to_string());
                            }
                            ref mut entry => {
                                *entry = Some(vec!["".to_string()]);
                            }
                        }
                    }
                    FlagAction::Append => {
                        let value = argv
                            .pop_front()
                            .or_else(flag.default_missing_value.clone())
                            .or_else(flag.default_value.clone());
                        if let Some(value) = value {
                            match this.arg_matches.entry(name.to_string()).or_insert_default() {
                                Some(ref mut values) => {
                                    values.push(value);
                                }
                                ref mut entry => {
                                    *entry = Some(vec![value]);
                                }
                            }
                        } else {
                            return Err(Report::msg(format!(
                                "missing required value for '--{name}'"
                            )));
                        }
                    }
                    FlagAction::SetTrue | FlagAction::SetFalse => {
                        this.arg_matches.insert(
                            name.to_string(),
                            Some(vec![flag.action.as_boolean_value().to_string()]),
                        );
                        continue;
                    }
                }
            }
        }

        Ok(this)
    }

    pub fn flags(&self) -> &[CompileFlag] {
        self.flags.as_slice()
    }

    /// Get the value of a custom flag with action `FlagAction::SetTrue` or `FlagAction::SetFalse`
    #[cfg(feature = "std")]
    pub fn get_flag(&self, name: &str) -> bool {
        self.arg_matches.get_flag(name)
    }

    /// Get the value of a custom flag with action `FlagAction::SetTrue` or `FlagAction::SetFalse`
    #[cfg(not(feature = "std"))]
    pub fn get_flag(&self, name: &str) -> bool {
        let flag =
            self.flags.iter().find(|flag| flag.name == name).unwrap_or_else(|| {
                panic!("invalid flag '--{name}', did you forget to register it?")
            });
        self.arg_matches
            .get(name)
            .and_then(|maybe_values| match maybe_values {
                Some(values) => Some(values[0].as_str() == "true"),
                None => match flag.default_missing_value.as_deref() {
                    None => None,
                    Some("true") => Some(true),
                    Some("false") => Some(false),
                    Some(other) => unreachable!(
                        "should not be possible to set '{other}' for boolean flag '--{name}'"
                    ),
                },
            })
            .unwrap_or_else(|| match flag.default_value.as_deref() {
                None => {
                    if flag.action == FlagAction::SetTrue {
                        false
                    } else {
                        true
                    }
                }
                Some("true") => Some(true),
                Some("false") => Some(false),
                Some(other) => {
                    panic!("invalid default_value for boolean flag '--{name}': '{other}'")
                }
            })
    }

    /// Get the count of a specific custom flag with action `FlagAction::Count`
    #[cfg(feature = "std")]
    pub fn get_flag_count(&self, name: &str) -> usize {
        self.arg_matches.get_count(name) as usize
    }

    /// Get the count of a specific custom flag with action `FlagAction::Count`
    #[cfg(not(feature = "std"))]
    pub fn get_flag_count(&self, name: &str) -> usize {
        let flag =
            self.flags.iter().find(|flag| flag.name == name).unwrap_or_else(|| {
                panic!("invalid flag '--{name}', did you forget to register it?")
            });
        self.arg_matches
            .get(name)
            .map(|maybe_values| match maybe_values {
                Some(values) => values.len(),
                None => match flag.default_missing_value.as_deref() {
                    None => 0,
                    Some(n) => n
                        .parse::<usize>()
                        .expect("invalid default_missing_value for '--{name}': '{n}'"),
                },
            })
            .unwrap_or_else(|| match flag.default_value.as_deref() {
                None => 0,
                Some(n) => n.parse::<usize>().expect("invalid default_value for '--{name}': '{n}'"),
            })
    }

    /// Get the value of a specific custom flag
    #[cfg(feature = "std")]
    pub fn get_flag_value<T>(&self, name: &str) -> Option<&T>
    where
        T: core::any::Any + Clone + Send + Sync + 'static,
    {
        self.arg_matches.get_one(name)
    }

    /// Get the value of a specific custom flag
    #[cfg(not(feature = "std"))]
    pub fn get_flag_value<T>(&self, name: &str) -> Option<&T>
    where
        T: core::any::Any + FromStr + Clone + Send + Sync + 'static,
    {
        let flag =
            self.flags.iter().find(|flag| flag.name == name).unwrap_or_else(|| {
                panic!("invalid flag '--{name}', did you forget to register it?")
            });
        match self.arg_matches.get(name)? {
            Some(values) => values
                .last()
                .as_deref()?
                .parse::<T>()
                .unwrap_or_else(|err| panic!("failed to parse value for --{name}: {err}")),
            None => match flag.default_missing_value.as_deref() {
                None => flag
                    .default_value
                    .as_deref()?
                    .parse::<T>()
                    .unwrap_or_else(|err| panic!("invalid default_value for --{name}: {err}")),
                Some(value) => value.parse::<T>().unwrap_or_else(|err| {
                    panic!("invalid default_missing_value for --{name}: {err}")
                }),
            },
        }
    }

    /// Iterate over values of a specific custom flag
    #[cfg(feature = "std")]
    pub fn get_flag_values<T>(&self, name: &str) -> Option<clap::parser::ValuesRef<'_, T>>
    where
        T: core::any::Any + Clone + Send + Sync + 'static,
    {
        self.arg_matches.get_many(name)
    }

    /// Get the remaining [clap::ArgMatches] left after parsing the base session configuration
    #[cfg(feature = "std")]
    pub fn matches(&self) -> &clap::ArgMatches {
        &self.arg_matches
    }
}

impl fmt::Debug for CompileFlags {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for id in self.arg_matches.ids() {
            use clap::parser::ValueSource;
            // Don't print CompilerOptions arg group
            if id.as_str() == "CompilerOptions" {
                continue;
            }
            // Don't print default values
            if matches!(self.arg_matches.value_source(id.as_str()), Some(ValueSource::DefaultValue))
            {
                continue;
            }
            map.key(&id.as_str()).value_with(|f| {
                let mut list = f.debug_list();
                if let Some(occurs) =
                    self.arg_matches.try_get_raw_occurrences(id.as_str()).expect("expected flag")
                {
                    list.entries(occurs.flatten());
                }
                list.finish()
            });
        }
        map.finish()
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for (name, value) in self.arg_matches.iter() {
            let flag = self.flags.iter().find(|flag| flag.name == name).unwrap();
            match value {
                Some(values) => {
                    map.key(&name).value_with(|f| f.debug_list().entries(values.iter()).finish());
                }
                None => {
                    map.key(&name).value(&None::<&str>);
                }
            }
        }
        map.finish()
    }
}

/// Generate a fake compile command for use with default options
#[cfg(feature = "std")]
fn fake_compile_command() -> clap::Command {
    let cmd = clap::Command::new("compile")
        .no_binary_name(true)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .disable_help_subcommand(true);
    register_flags(cmd)
}

/// Register dynamic flags to be shown via `midenc help compile`
#[cfg(feature = "std")]
pub fn register_flags(cmd: clap::Command) -> clap::Command {
    inventory::iter::<CompileFlag>.into_iter().fold(cmd, |cmd, flag| {
        let arg = clap::Arg::new(flag.name)
            .long(flag.long.unwrap_or(flag.name))
            .action(clap::ArgAction::from(flag.action));
        let arg = if let Some(help) = flag.help {
            arg.help(help)
        } else {
            arg
        };
        let arg = if let Some(help_heading) = flag.help_heading {
            arg.help_heading(help_heading)
        } else {
            arg
        };
        let arg = if let Some(short) = flag.short {
            arg.short(short)
        } else {
            arg
        };
        let arg = if let Some(env) = flag.env {
            arg.env(env)
        } else {
            arg
        };
        let arg = if let Some(value) = flag.default_missing_value {
            arg.default_missing_value(value)
        } else {
            arg
        };
        let arg = if let Some(value) = flag.default_value {
            arg.default_value(value)
        } else {
            arg
        };
        let arg = if let Some(value) = flag.hide {
            arg.hide(value)
        } else {
            arg
        };
        cmd.arg(arg)
    })
}
