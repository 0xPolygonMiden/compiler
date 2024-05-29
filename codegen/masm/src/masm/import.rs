use core::{
    hash::{Hash, Hasher},
    str::FromStr,
};

use anyhow::bail;
use miden_diagnostics::{SourceSpan, Spanned};
use midenc_hir::Symbol;

/// This represents an import statement in Miden Assembly
#[derive(Debug, Copy, Clone, Spanned)]
pub struct Import {
    /// The source span corresponding to this import statement, if applicable
    #[span]
    pub span: SourceSpan,
    /// The fully-qualified name of the imported module, e.g. `std::math::u64`
    pub name: Symbol,
    /// The name to which the imported module is aliased locally, e.g. `u64`
    /// is the alias for `use std::math::u64`, which is the default behavior.
    ///
    /// However, custom aliases are permitted, and we may use this to disambiguate
    /// imported modules, e.g. `use std::math::u64->my_u64` will result in the
    /// alias for this import being `my_u64`.
    pub alias: Symbol,
}
impl Import {
    /// Returns true if this import has a custom alias, or if it uses the
    /// default aliasing behavior for imports
    pub fn is_aliased(&self) -> bool {
        !self.name.as_str().ends_with(self.alias.as_str())
    }

    /// Returns true if this import conflicts with `other`
    ///
    /// A conflict arises when the same name is used to reference two different
    /// imports locally within a module, i.e. the aliases conflict
    pub fn conflicts_with(&self, other: &Self) -> bool {
        self.alias == other.alias && self.name != other.name
    }
}
impl Eq for Import {}
impl PartialEq for Import {
    fn eq(&self, other: &Self) -> bool {
        // If the names are different, the imports can't be equivalent
        if self.name != other.name {
            return false;
        }
        // Otherwise, equivalence depends on the aliasing of the import
        match (self.is_aliased(), other.is_aliased()) {
            (true, true) => {
                // Two imports that are custom aliased are equivalent only if
                // both the fully-qualified name and the alias are identical
                self.alias == other.alias
            }
            (true, false) | (false, true) => {
                // If one import is aliased and the other is not, the imports
                // are never equivalent, because they can't possibly refer to
                // the same module by the same name
                false
            }
            (false, false) => {
                // Two unaliased imports are the same if their names are the same
                true
            }
        }
    }
}
impl PartialOrd for Import {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Import {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.alias.cmp(&other.alias))
    }
}
impl Hash for Import {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.alias.hash(state);
    }
}
impl FromStr for Import {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = s.strip_prefix("use ").unwrap_or(s);
        if s.contains(char::is_whitespace) {
            bail!(
                "invalid import '{}': unexpected whitespace in identifier",
                s
            );
        }
        let (name, alias) = match s.rsplit_once("->") {
            None => match s.rsplit_once("::") {
                None => {
                    let name = Symbol::intern(s);
                    (name, name)
                }
                Some((_, alias)) if alias.is_empty() => {
                    bail!("invalid import '{}': trailing '::' is not allowed", s)
                }
                Some((_, alias)) => {
                    let name = Symbol::intern(s);
                    let alias = Symbol::intern(alias);
                    (name, alias)
                }
            },
            Some((_, alias)) if alias.is_empty() => {
                bail!("invalid import '{}': alias cannot be empty", s)
            }
            Some((fqn, alias)) => {
                let name = Symbol::intern(fqn);
                let alias = Symbol::intern(alias);
                (name, alias)
            }
        };

        Ok(Self {
            span: SourceSpan::UNKNOWN,
            name,
            alias,
        })
    }
}
