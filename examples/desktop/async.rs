use dip::prelude::*;
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugin(DesktopPlugin::<UiState, NoUiAction, AsyncAction>::new(Root))
        .add_plugin(UiStatePlugin)
        .add_startup_system(fetch)
        .add_system(log_fetch_result)
        .run();
}

#[ui_state]
struct UiState {
    ip_address: GetIpAddress,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum AsyncAction {
    GetIpAddress(GetIpAddress),
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct GetIpAddress {
    origin: String,
}

fn fetch(async_action: Res<AsyncActionPool<AsyncAction>>) {
    async_action.send(async move {
        let res = reqwest::get("https://httpbin.org/ip")
            .await
            .unwrap()
            .json::<GetIpAddress>()
            .await
            .unwrap();

        AsyncAction::GetIpAddress(res)
    });
}

fn log_fetch_result(mut events: EventReader<AsyncAction>, mut ip_address: ResMut<GetIpAddress>) {
    for res in events.iter() {
        match res {
            AsyncAction::GetIpAddress(res) => {
                *ip_address = res.clone();
            }
        }
    }
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let ip_address = use_read(&cx, IP_ADDRESS);
    cx.render(rsx! {
        h1 { "ip address: {ip_address.origin}" }
    })
}
