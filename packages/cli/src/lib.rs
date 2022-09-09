pub use dip_macro::{cli_plugin as plugin, cli_subcommand as subcommand};

pub mod prelude {
    pub use dip_macro::{cli_plugin as plugin, cli_subcommand as subcommand};
    pub use dip_macro::{ui_action, ui_state};
}
