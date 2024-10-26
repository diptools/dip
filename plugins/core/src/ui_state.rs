// use dioxus::fermi::AtomRoot;
use std::rc::Rc;

/// Trait to provide custom handerl for Ui states. This trait is automatically implemented with UiStatePlugin macro.
pub trait UiStateHandler {
    ///
    fn handler(self, _atom_root: Rc<AtomRoot>);
}

/// Placeholder
pub struct NoUiState;

impl UiStateHandler for NoUiState {
    fn handler(self, _atom_root: Rc<AtomRoot>) {}
}

pub type NoUiAction = ();

pub type NoRootProps = ();
