use dip::prelude::*;
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugin(DesktopPlugin::<UiState, NoUiAction, AsyncAction>::new(Root))
        .add_plugin(UiStatePlugin)
        .add_plugin(AsyncActionPlugin)
        .add_startup_system(get_ip_address)
        .add_system(handle_get_ip_address)
        .run();
}

#[ui_state]
struct UiState {
    ip_address: GetIpAddress,
}

#[allow(dead_code)]
#[derive(Clone, Debug, AsyncActionPlugin)]
enum AsyncAction {
    GetIpAddress(GetIpAddress),
    // PostSomething(PostSomething),
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct GetIpAddress {
    origin: String,
}

// #[derive(Clone, Debug, Deserialize, Default)]
// pub struct PostSomething;

impl AsyncAction {
    async fn get_ip_address() -> AsyncAction {
        let res = AsyncActionCreator::get_ip_address().await;
        AsyncAction::GetIpAddress(res)
    }
}

struct AsyncActionCreator;

impl AsyncActionCreator {
    async fn get_ip_address() -> GetIpAddress {
        reqwest::get("https://httpbin.org/ip")
            .await
            .unwrap()
            .json::<GetIpAddress>()
            .await
            .unwrap()
    }
}

fn get_ip_address(async_action: Res<AsyncActionPool<AsyncAction>>) {
    async_action.send(AsyncAction::get_ip_address());
}

fn handle_get_ip_address(
    mut actions: EventReader<GetIpAddress>,
    mut ip_address: ResMut<GetIpAddress>,
) {
    for action in actions.iter() {
        *ip_address = action.clone();
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let ip_address = use_read(&cx, IP_ADDRESS);

    cx.render(rsx! {
        h1 { "ip address: {ip_address.origin}" }
    })
}
