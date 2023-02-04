use dip::{
    prelude::*,
    web3::starknet::{RpcRequest, StarknetLightClientPlugin},
};
use tokio::sync::mpsc::Sender;

fn main() {
    App::new()
        .add_plugin(DesktopPlugin::<UiState, UiAction, NoAsyncAction>::new(Root))
        .add_plugin(UiStatePlugin)
        .add_plugin(UiActionPlugin)
        .add_plugin(StarknetLightClientPlugin)
        .add_system(get_block_number)
        .run();
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let count = use_read(&cx, BLOCK_NUMBER);

    let window = use_window::<UiAction, NoAsyncAction>(&cx);

    cx.render(rsx! {
        h1 { "Starknet Light Client (Beerus) Plugin Example" }
        // p { "count: {count.value}" }
        button {
            onclick: move |_| window.send(UiAction::get_block_number()),
            // disabled: "{disabled}",
            "Get Block Number",
        }
    })
}

#[ui_state]
struct UiState {
    block_number: BlockNumber,
}

#[derive(Clone, Debug, Default)]
pub struct BlockNumber {
    block_number: u32,
}

#[derive(Clone, Debug)]
pub struct GetBlockNumber;

#[ui_action]
impl ActionCreator {
    fn get_block_number() -> GetBlockNumber {
        GetBlockNumber
    }
}

fn get_block_number(mut events: EventReader<GetBlockNumber>, rpc: Res<Sender<RpcRequest>>) {
    for _ in events.iter() {
        info!("🧠 GetBlockNumber");
        rpc.try_send(RpcRequest::GetBlockNumber).unwrap();
    }
}
