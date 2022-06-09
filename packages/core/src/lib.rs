//! Shared resources across platforms

use fermi::AtomRoot;
use std::rc::Rc;

/// Trait to provide custom handerl for global states. This trait is automatically implemented with GlobalStatePlugin macro.
pub trait GlobalStateHandler {
    ///
    fn handler(self, _atom_root: Rc<AtomRoot>);
}

/// Placeholder
pub struct EmptyGlobalState;

impl GlobalStateHandler for EmptyGlobalState {
    fn handler(self, _atom_root: Rc<AtomRoot>) {}
}

pub mod prelude {
    pub use crate::{EmptyGlobalState, GlobalStateHandler};
    pub use bevy_dioxus_macro::{GlobalState, GlobalStatePlugin};
}
