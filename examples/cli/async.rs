use dip::{
    bevy::{
        app::AppExit,
        log::{self, LogPlugin},
    },
    prelude::*,
};
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugin(CliPlugin::<AsyncAction>::continuous())
        .add_plugin(ActionPlugin)
        .add_plugin(AsyncActionPlugin)
        .add_plugin(LogPlugin)
        .add_startup_system(fetch_ip_address)
        .add_startup_system(fetch_user_agent)
        .add_system(handle_get_ip_address)
        .add_system(handle_get_user_agent)
        .run();
}

#[derive(CliPlugin, clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone)]
pub enum Action {
    IpAddress,
    UserAgent,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct GetIpAddress {
    origin: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct GetUserAgent {
    #[serde(rename = "user-agent")]
    user_agent: Option<String>,
}

#[async_action]
impl AsyncActionCreator {
    async fn get_ip_address() -> GetIpAddress {
        reqwest::get("https://httpbin.org/ip")
            .await
            .unwrap()
            .json::<GetIpAddress>()
            .await
            .unwrap()
    }

    async fn get_user_agent() -> GetUserAgent {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        client
            .get("https://httpbin.org/user-agent")
            .send()
            .await
            .unwrap()
            .json::<GetUserAgent>()
            .await
            .unwrap()
    }
}

fn fetch_ip_address(
    mut events: EventReader<IpAddressAction>,
    async_action: Res<AsyncActionPool<AsyncAction>>,
) {
    for _ in events.iter() {
        async_action.send(AsyncAction::get_ip_address());
    }
}

fn fetch_user_agent(
    mut events: EventReader<UserAgentAction>,
    async_action: Res<AsyncActionPool<AsyncAction>>,
) {
    for _ in events.iter() {
        async_action.send(AsyncAction::get_user_agent());
    }
}

fn handle_get_ip_address(
    mut actions: EventReader<GetIpAddress>,
    mut app_exit: EventWriter<AppExit>,
) {
    for action in actions.iter() {
        log::info!("{action:#?}");
        app_exit.send(AppExit);
    }
}

fn handle_get_user_agent(
    mut actions: EventReader<GetUserAgent>,
    mut app_exit: EventWriter<AppExit>,
) {
    for action in actions.iter() {
        log::info!("{action:#?}");
        app_exit.send(AppExit);
    }
}
