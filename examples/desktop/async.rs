use dip::{bevy::log::LogPlugin, prelude::*};
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugin(DesktopPlugin::<UiState, NoUiAction, AsyncAction>::new(Root))
        .add_plugin(LogPlugin)
        .add_plugin(UiStatePlugin)
        .add_plugin(AsyncActionPlugin)
        .add_startup_system(fetch_all)
        .add_system(handle_get_ip_address)
        .add_system(handle_get_user_agent)
        .run();
}

#[ui_state]
struct UiState {
    ip_address: GetIpAddress,
    user_agent: GetUserAgent,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct GetIpAddress {
    origin: String,
}

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

fn fetch_all(async_action: Res<AsyncActionPool<AsyncAction>>) {
    async_action.send(AsyncAction::get_ip_address());
    async_action.send(AsyncAction::get_user_agent());
}

fn handle_get_ip_address(
    mut actions: EventReader<GetIpAddress>,
    mut ip_address: ResMut<GetIpAddress>,
) {
    for action in actions.iter() {
        *ip_address = action.clone();
    }
}

fn handle_get_user_agent(
    mut actions: EventReader<GetUserAgent>,
    mut user_agent: ResMut<GetUserAgent>,
) {
    for action in actions.iter() {
        *user_agent = action.clone();
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let ip_address = use_read(&cx, IP_ADDRESS);
    let user_agent = use_read(&cx, USER_AGENT);

    cx.render(rsx! {
        h1 { "ip address: {ip_address.origin}" }
        h1 { "user_agent: {user_agent.user_agent:?}" }
    })
}
