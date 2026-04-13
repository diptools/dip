use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use bevy::{
    app::{App, Plugin},
    ecs::system::{Commands, Res},
};
use dotenv::dotenv;
use prettytable::{format::consts::FORMAT_BOX_CHARS, table};
use tokio::{runtime::Runtime, sync::mpsc::Sender};

pub struct StarknetLightClientPlugin;

impl Plugin for StarknetLightClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RpcRequest>();

        if dotenv().is_ok() {
            let config = Config::new_from_env().unwrap();

            Self::print_config(&config);

            let runtime = Runtime::new().unwrap();

            let client = runtime.block_on(async move {
                let ethereum_lightclient = HeliosLightClient::new(config.clone()).await.unwrap();
                let starknet_lightclient = StarkNetLightClientImpl::new(&config.clone()).unwrap();

                let mut client = BeerusLightClient::new(
                    config.clone(),
                    Box::new(ethereum_lightclient),
                    Box::new(starknet_lightclient),
                );

                println!("Light Client initialized");

                client
            });

            let runner = StarketLightClientRunner::new();
            let tx = runner.start(client);

            app.insert_resource(runner).insert_resource(tx);
        }
    }
}

impl StarknetLightClientPlugin {
    fn print_config(config: &Config) {
        let mut table = table!(
            ["KEY", "VALUE"],
            ["ethereum_network", config.ethereum_network],
            ["ethereum_consensus_rpc", &config.ethereum_consensus_rpc],
            [
                "ethereum_execution_rpc",
                Self::hide_credential(&config.ethereum_execution_rpc)
            ],
            ["starknet_rpc", Self::hide_credential(&config.starknet_rpc)],
            [
                "starknet_core_contract_address",
                config.starknet_core_contract_address
            ]
        );
        table.set_format(*FORMAT_BOX_CHARS);
        table.printstd();
    }

    fn hide_credential(value: &str) -> String {
        let mut v = value.split("/").take(4).collect::<Vec<&str>>();
        v.push("***");
        v.join("/")
    }
}

struct StarketLightClientRunner {
    runtime: Runtime,
}

#[derive(Debug)]
pub enum RpcRequest {
    GetBlockNumber,
}

impl StarketLightClientRunner {
    fn new() -> Self {
        Self {
            runtime: Runtime::new().unwrap(),
        }
    }

    fn start(&self, mut client: BeerusLightClient) -> Sender<RpcRequest> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<RpcRequest>(8);

        let client = self.runtime.block_on(async move {
            client.start().await.unwrap();
            println!("LightClient Synced");

            client
        });

        self.runtime.spawn(async move {
            while let Some(msg) = rx.recv().await {
                let block_tx_count = client.starknet_lightclient.block_number().await.unwrap();

                println!("{block_tx_count}");
            }
        });

        tx
    }
}
