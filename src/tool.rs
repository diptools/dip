use dip::{bevy::ecs::system::Res, cli::Subcommand};

#[derive(Clone, Debug, Subcommand, clap::Subcommand)]
#[clap(name = "tool")]
pub enum Tool {
    Install,
}

impl Default for Tool {
    fn default() -> Self {
        Self::Install
    }
}
