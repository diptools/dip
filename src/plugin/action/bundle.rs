use dip::cli::SubcommandPlugin;

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum BundleAction {
    Apply {
        #[clap(short, long, required = false)]
        verbose: bool,
    },
}
