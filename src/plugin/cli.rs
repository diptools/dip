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

    #[clap(short, long, default_value_t = String::from("tailwind.config.js"))]
    pub config: String,

    #[clap(short, long, default_value_t = String::from("styles/style.css"))]
    pub input: String,

    #[clap(short, long, default_value_t = String::from("static/style.css"))]
    pub output: String,

    #[clap(short, long)]
    pub watch: bool,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum ToolAction {
    List,
    Add { name: String },
}
