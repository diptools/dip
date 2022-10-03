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
#[derive(Clone, Debug)]
enum AsyncAction {
    GetIpAddress(GetIpAddress),
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct GetIpAddress {
    origin: String,
}

fn get_ip_address(async_action: Res<AsyncActionPool<AsyncAction>>) {
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

pub struct AsyncActionPlugin;

impl Plugin for AsyncActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GetIpAddress>()
            .add_system_to_stage(UiStage::Action, handle_async_action);
    }
}

fn handle_async_action(
    mut events: EventReader<AsyncAction>,
    mut get_ip_address: EventWriter<GetIpAddress>,
) {
    for action in events.iter() {
        match action {
            AsyncAction::GetIpAddress(res) => {
                get_ip_address.send(res.clone());
            }
        }
    }
}

fn handle_get_ip_address(
    mut events: EventReader<GetIpAddress>,
    mut ip_address: ResMut<GetIpAddress>,
) {
    for action in events.iter() {
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
