// -----------------------------------------------------------------------------
// Seal

mod private {
    pub trait Seal {}
}

// -----------------------------------------------------------------------------
// ComponentMutability

pub trait ComponentMutability: private::Seal + 'static {
    /// Boolean to indicate if this mutability setting implies
    /// a mutable or immutable component.
    const MUTABLE: bool;
}

// -----------------------------------------------------------------------------
// Immutable

/// Parameter indicating a `Component` is immutable.
pub struct Immutable;

impl private::Seal for Immutable {}

impl ComponentMutability for Immutable {
    const MUTABLE: bool = false;
}

// -----------------------------------------------------------------------------
// Mutable

/// Parameter indicating a `Component` is mutable.
pub struct Mutable;

impl private::Seal for Mutable {}

impl ComponentMutability for Mutable {
    const MUTABLE: bool = true;
}
