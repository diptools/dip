mod render_mode;

pub use dip_macro::{CliPlugin, SubcommandPlugin};
pub use render_mode::RenderMode;

pub mod prelude {
    pub use crate::render_mode::RenderMode;
    pub use dip_macro::{CliPlugin, SubcommandPlugin};
}
