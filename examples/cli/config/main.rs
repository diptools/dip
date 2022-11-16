use config::{
    builder::{ConfigBuilder, DefaultState},
    File,
};
use dip::prelude::*;
use serde::Deserialize;

fn main() {
    App::new()
<<<<<<< HEAD
        .add_plugin(ConfigPlugin::new())
        .add_startup_system(add_config_source.before(build_config))
        .add_system(log_config.after(build_config))
=======
        .add_plugin(ConfigPlugin::<AppConfig>::new())
        .add_startup_system_to_stage(ConfigStartupStage::Setup, add_custom_sources)
        .add_system(log_config)
>>>>>>> 0f1f59e (Pass Config type to ConfigPlugin)
        .run();
}

fn add_custom_sources(mut builder: ResMut<ConfigBuilder<DefaultState>>) {
    *builder = builder
        .clone()
        .add_source(File::with_name("examples/cli/config/config/development"));
}

fn log_config(config: Res<Config>) {
    println!("{:#?}", *config);
}

#[derive(ConfigPlugin, Debug, Deserialize)]
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
