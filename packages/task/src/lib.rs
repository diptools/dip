mod async_action;

pub use crate::async_action::{AsyncActionPool, NoAsyncAction};
pub use dip_macro::async_action;

pub mod prelude {
    pub use crate::{async_action, AsyncActionPool, NoAsyncAction};
}
