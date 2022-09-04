use dip::cli::{DipCli, Parser};

fn main() {
    let cli = DipCli::parse();
    cli.run();
}
