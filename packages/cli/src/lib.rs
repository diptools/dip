pub use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct DipCli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Test,
}

impl DipCli {
    pub fn run(&self) {
        match self.command {
            Commands::Test => {
                println!("Test subcommand");
            }
        }
    }
}
