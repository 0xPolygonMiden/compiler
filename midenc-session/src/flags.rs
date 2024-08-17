#[derive(Debug)]
pub struct CompileFlag {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    pub help: Option<&'static str>,
    pub help_heading: Option<&'static str>,
    pub env: Option<&'static str>,
    pub action: FlagAction,
    pub default_missing_value: Option<&'static str>,
    pub default_value: Option<&'static str>,
    pub hide: Option<bool>,
}
impl CompileFlag {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            short: None,
            long: None,
            help: None,
            help_heading: None,
            env: None,
            action: FlagAction::Set,
            default_missing_value: None,
            default_value: None,
            hide: None,
        }
    }

    pub const fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub const fn long(mut self, long: &'static str) -> Self {
        self.long = Some(long);
        self
    }

    pub const fn action(mut self, action: FlagAction) -> Self {
        self.action = action;
        self
    }

    pub const fn help(mut self, help: &'static str) -> Self {
        self.help = Some(help);
        self
    }

    pub const fn help_heading(mut self, help_heading: &'static str) -> Self {
        self.help_heading = Some(help_heading);
        self
    }

    pub const fn env(mut self, env: &'static str) -> Self {
        self.env = Some(env);
        self
    }

    pub const fn default_value(mut self, value: &'static str) -> Self {
        self.default_value = Some(value);
        self
    }

    pub const fn default_missing_value(mut self, value: &'static str) -> Self {
        self.default_missing_value = Some(value);
        self
    }

    pub const fn hide(mut self, yes: bool) -> Self {
        self.hide = Some(yes);
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FlagAction {
    Set,
    Append,
    SetTrue,
    SetFalse,
    Count,
}
impl From<FlagAction> for clap::ArgAction {
    fn from(action: FlagAction) -> Self {
        match action {
            FlagAction::Set => Self::Set,
            FlagAction::Append => Self::Append,
            FlagAction::SetTrue => Self::SetTrue,
            FlagAction::SetFalse => Self::SetFalse,
            FlagAction::Count => Self::Count,
        }
    }
}

inventory::collect!(CompileFlag);

/// Generate a fake compile command for use with default options
fn fake_compile_command() -> clap::Command {
    let cmd = clap::Command::new("compile")
        .no_binary_name(true)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .disable_help_subcommand(true);
    register_flags(cmd)
}

/// Get [clap::ArgMatches] for registered command-line flags, without a [clap::Command]
pub fn default_arg_matches<I, V>(argv: I) -> Result<clap::ArgMatches, clap::Error>
where
    I: IntoIterator<Item = V>,
    V: Into<std::ffi::OsString> + Clone,
{
    fake_compile_command().try_get_matches_from(argv)
}

/// Register dynamic flags to be shown via `midenc help compile`
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
