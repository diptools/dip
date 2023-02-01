use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};

use bevy::{
    app::{App, Plugin},
    ecs::system::Commands,
};
use dotenv::dotenv;
use prettytable::table;
use tokio::runtime::Runtime;

pub struct StarknetLightClientPlugin;

impl Plugin for StarknetLightClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    if dotenv().is_ok() {
        let config = Config::new_from_env().unwrap();

        // Std out light client configs
        let table = table!(
            ["KEY", "VALUE"],
            ["ethereum_network", config.ethereum_network],
            ["ethereum_consensus_rpc", config.ethereum_consensus_rpc],
            ["ethereum_execution_rpc", config.ethereum_execution_rpc],
            ["starknet_rpc", config.starknet_rpc],
            [
                "starknet_core_contract_address",
                config.starknet_core_contract_address
            ]
        );
        table.printstd();

        // Read the config from the environment.
        let config = Config::new_from_env().unwrap();

        Runtime::new().unwrap().block_on(async {
            // Create a new Ethereum light client.
            let ethereum_lightclient = HeliosLightClient::new(config.clone()).await.unwrap();

            // Create a new StarkNet light client.
            let starknet_lightclient = StarkNetLightClientImpl::new(&config).unwrap();

            // Create a new Beerus light client.
            let mut client = BeerusLightClient::new(
                config,
                Box::new(ethereum_lightclient),
                Box::new(starknet_lightclient),
            );

            // Start the Beerus light client.
            client.start().await.unwrap();
        });
    }
}
