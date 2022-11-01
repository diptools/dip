use dip::cli::SubcommandPlugin;

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum ToolAction {
    List,
    Add { name: String },
}
