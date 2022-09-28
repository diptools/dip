use config::{
    builder::{ConfigBuilder, DefaultState},
    File,
};
use dip::prelude::*;
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugin(ConfigPlugin)
        .add_startup_system(log_config.before(build_config))
        .add_startup_system(build_config)
        .run();
}

fn log_config(mut builder: ResMut<ConfigBuilder<DefaultState>>) {
    *builder = builder
        .clone()
        .add_source(File::with_name("examples/cli/config/config/development"));
    println!("{:#?}", *builder);
}

fn build_config(builder: Res<ConfigBuilder<DefaultState>>) {
    let config = builder
        .clone()
        .build()
        .unwrap()
        .try_deserialize::<'static, Config>();
    println!("{:#?}", config);
}

#[config_plugin]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Config {
    base_url: String,
    backend: Backend,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Backend {
    api_key: String,
    api_secret: String,
}
