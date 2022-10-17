use dip::cli::{CliPlugin, SubcommandPlugin};

#[derive(CliPlugin, clap::Parser)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum Action {
    Build(BuildArgs),
    #[clap(subcommand)]
    Tool(ToolAction),
}

#[derive(clap::Args, Clone, Debug)]
pub struct BuildArgs {
    #[clap(short, long, default_value_t = String::from("."))]
    pub path: String,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum ToolAction {
    List,
    Add { name: String },
}
