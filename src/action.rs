use crate::tool::{handle_tool_action, ToolAction};
use dip::cli::SubcommandPlugin;

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum Action {
    #[clap(subcommand)]
    Tool(ToolAction),
}
