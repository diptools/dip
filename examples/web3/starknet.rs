use dip::{prelude::*, web3::starknet::StarknetLightClientPlugin};

fn main() {
    App::new().add_plugin(StarknetLightClientPlugin).run();
}
