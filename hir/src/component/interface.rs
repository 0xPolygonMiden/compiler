use miden_hir_symbol::Symbol;

/// A fully-qualified identifier for the interface being imported, e.g.
/// `namespace::package/interface@version`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterfaceIdent {
    /// A fully-qualified identifier for the interface being imported, e.g.
    /// `namespace::package/interface@version`
    pub full_name: Symbol,
}

impl InterfaceIdent {
    /// Create a new [InterfaceIdent] from a fully-qualified interface identifier, e.g.
    /// `namespace::package/interface@version`
    pub fn from_full_ident(full_ident: String) -> Self {
        Self {
            full_name: Symbol::intern(full_ident),
        }
    }
}

/// An identifier for a function in an interface
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterfaceFunctionIdent {
    /// An interface identifier for the interface being imported (e.g.
    /// `namespace::package/interface@version`)
    pub interface: InterfaceIdent,
    /// The name of the function from the interface
    pub function: Symbol,
}

impl InterfaceFunctionIdent {
    /// Create a new [InterfaceFunctionIdent] from a fully-qualified interface
    /// identifier(e.g. `namespace::package/interface@version`) and a function name
    pub fn from_full(interface: String, function: String) -> Self {
        Self {
            interface: InterfaceIdent::from_full_ident(interface.to_string()),
            function: Symbol::intern(function),
        }
    }
}
