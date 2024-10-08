#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlagAction {
    Set,
    Append,
    SetTrue,
    SetFalse,
    Count,
}
impl FlagAction {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::SetTrue | Self::SetFalse)
    }

    pub fn as_boolean_value(&self) -> bool {
        assert!(self.is_boolean());
        self == &Self::SetTrue
    }
}

#[cfg(feature = "std")]
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
