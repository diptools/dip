use dip::{
    bevy::app::{App, Plugin},
    cli::SubcommandPlugin,
};

mod build;
mod bundle;
mod tool;

pub use build::*;
pub use bundle::*;
pub use tool::*;

pub struct DipActionPlugin;

impl Plugin for DipActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionPlugin).add_plugin(BuildActionPlugin);
    }
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum Action {
    Build(BuildArgs),

    #[clap(subcommand)]
    Bundle(BundleAction),

    #[clap(subcommand)]
    Tool(ToolAction),
}
