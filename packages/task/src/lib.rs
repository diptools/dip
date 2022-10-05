mod async_action;

pub use crate::async_action::{AsyncActionPool, NoAsyncAction};
pub use dip_macro::AsyncActionPlugin;

pub mod prelude {
    pub use crate::{AsyncActionPlugin, AsyncActionPool, NoAsyncAction};
}
