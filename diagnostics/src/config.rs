use crate::term::Config;

#[derive(Debug, Clone)]
pub struct DiagnosticsConfig {
    pub verbosity: Verbosity,
    pub warnings_as_errors: bool,
    pub no_warn: bool,
    pub display: Config,
}
impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            verbosity: Verbosity::Info,
            warnings_as_errors: false,
            no_warn: false,
            display: Config::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    Debug,
    Info,
    Warning,
    Error,
    Silent,
}
impl Verbosity {
    pub fn from_level(level: isize) -> Self {
        if level < 0 {
            return Verbosity::Silent;
        }

        match level {
            0 => Verbosity::Warning,
            1 => Verbosity::Info,
            _ => Verbosity::Debug,
        }
    }

    pub fn is_silent(&self) -> bool {
        match self {
            Self::Silent => true,
            _ => false,
        }
    }
}
