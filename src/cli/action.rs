// mod build;

// pub use build::*;

use dip::{
    cli::{CliPlugin, SubcommandPlugin},
    prelude::Resource,
};

#[derive(CliPlugin, clap::Parser, Resource)]
#[clap(version)]
struct Cli {
    // #[clap(subcommand)]
    // action: Action,
}

// #[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
// pub enum Action {
//     Build(BuildArgs),
//     #[clap(subcommand)]
//     Bundle(BundleAction),
//     #[clap(subcommand)]
//     Device(DeviceAction),
// }

// #[derive(clap::Args, Clone, Debug)]
// pub struct BuildArgs {
//     #[clap(short, long, default_value_t = String::from("."))]
//     pub path: String,

//     #[clap(short, long, default_value_t = String::from("tailwind.config.js"))]
//     pub config: String,

//     #[clap(short, long, default_value_t = String::from("styles/style.css"))]
//     pub input: String,

//     #[clap(short, long, default_value_t = String::from("static/style.css"))]
//     pub output: String,

//     #[clap(short, long)]
//     pub watch: bool,
// }

// #[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
// pub enum BundleAction {
//     Apply(ApplyBundleArgs),
//     Clean(CleanBundleArgs),
// }

#[derive(clap::Args, Clone, Debug)]
pub struct ApplyBundleArgs {
    pub bundle_root: Option<String>,
}

#[derive(clap::Args, Clone, Debug)]
pub struct CleanBundleArgs {
    pub bundle_root: Option<String>,
}

// #[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
// pub enum DeviceAction {
//     List,
//     Info,
// }
