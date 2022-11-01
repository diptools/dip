use dip::cli::SubcommandPlugin;

mod build;
mod bundle;
mod tool;

pub use build::*;
pub use bundle::*;
pub use tool::*;

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum Action {
    Build(BuildArgs),

    #[clap(subcommand)]
    Bundle(BundleAction),

    #[clap(subcommand)]
    Tool(ToolAction),
}
