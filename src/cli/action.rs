use dip::{
    builder::Platform,
    cli::{CliPlugin, SubcommandPlugin},
};

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
    Bundle(BundleAction),

    #[clap(subcommand)]
    Device(DeviceAction),
}

#[derive(clap::Args, Clone, Debug)]
pub struct BuildArgs {
    pub project_dir: Option<String>,

    #[clap(long)]
    pub platform: Option<Platform>,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum BundleAction {
    Apply(ApplyBundleArgs),
    // Clean(CleanBundleArgs),
}

#[derive(clap::Args, Clone, Debug)]
pub struct ApplyBundleArgs {
    pub bundle_root: Option<String>,
}

// #[derive(clap::Args, Clone, Debug)]
// pub struct CleanBundleArgs {
//     pub bundle_root: Option<String>,
// }

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum DeviceAction {
    List,
    Info,
}
