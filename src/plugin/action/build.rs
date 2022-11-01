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
