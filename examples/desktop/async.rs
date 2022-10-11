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
        .add_system(handle_get_ip_address_wrongly)
        .add_system(handle_get_user_agent)
        .run();
}

#[ui_state]
struct UiState {
    ip_address: GetIpAddress,
    async_action_error: AsyncActionError,
    user_agent: GetUserAgent,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct GetIpAddress {
    origin: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct GetIpAddressWrongly {
    blabla: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct GetUserAgent {
    #[serde(rename = "user-agent")]
    user_agent: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AsyncActionError {
    None,
    NetworkError(std::sync::Arc<reqwest::Error>),
}

impl Default for AsyncActionError {
    fn default() -> Self {
        Self::None
    }
}

impl From<reqwest::Error> for AsyncActionError {
    fn from(error: reqwest::Error) -> Self {
        AsyncActionError::NetworkError(std::sync::Arc::new(error))
    }
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

    async fn get_ip_address_wrongly() -> Result<GetIpAddressWrongly, AsyncActionError> {
        match reqwest::get("https://httpbin.org/ip").await {
            Ok(res) => match res.json::<GetIpAddressWrongly>().await {
                Ok(json) => Ok(json),
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e.into()),
        }
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
    async_action.send(AsyncAction::get_ip_address_wrongly());
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

fn handle_get_ip_address_wrongly(
    mut actions: EventReader<Result<GetIpAddressWrongly, AsyncActionError>>,
    mut async_action_error: ResMut<AsyncActionError>,
) {
    for action in actions.iter() {
        if let Err(e) = action {
            *async_action_error = e.clone();
        }
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
    let async_action_error = use_read(&cx, ASYNC_ACTION_ERROR);

    cx.render(rsx! {
        ul {
            li { "ip address: {ip_address.origin}" }
            li { "user_agent: {user_agent.user_agent:?}" }
            li { "async_action_error: {async_action_error:?}" }
        }
    })
}
