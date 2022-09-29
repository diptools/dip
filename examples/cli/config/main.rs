use config::{
    builder::{ConfigBuilder, DefaultState},
    File,
};
use dip::prelude::*;
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugin(ConfigPlugin::default())
        .add_startup_system(add_config_source.before(build_config))
        .add_system(log_config.after(build_config))
        .run();
}

fn add_config_source(mut builder: ResMut<ConfigBuilder<DefaultState>>) {
    *builder = builder
        .clone()
        .add_source(File::with_name("examples/cli/config/config/development"));
}

fn log_config(config: Res<Config>) {
    println!("{:#?}", *config);
}

#[config_plugin]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    base_url: String,
    backend: Backend,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Backend {
    api_key: String,
    api_secret: String,
}
