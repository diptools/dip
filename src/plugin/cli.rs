use crate::plugin::action::{handle_action, Action};
use dip::cli::CliPlugin;

#[derive(CliPlugin, clap::Parser)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}
