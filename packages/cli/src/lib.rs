pub mod clap {
    pub use clap::*;
    pub mod builder {
        pub use clap::builder::ValueParser;
    }
}
pub use dip_macro::{cli_plugin as plugin, cli_subcommand as subcommand};

pub mod prelude {
    pub use dip_macro::{cli_plugin as plugin, cli_subcommand as subcommand};
    pub use dip_macro::{ui_action, ui_state};
}
