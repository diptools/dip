use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
    log,
};
use hidapi::HidApi;
use ledger_transport_hid::TransportNativeHID;
use ledger_zondax_generic::{App as LApp, AppExt};
use once_cell::sync::Lazy;

pub struct DevicePlugin;

impl Plugin for DevicePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ListDevice>()
            .add_event::<InfoDevice>()
            .add_system(list_devices)
            .add_system(device_info);
    }
}

pub struct ListDevice;

pub struct InfoDevice;

pub fn list_devices(mut actions: EventReader<ListDevice>) {
    actions.iter().for_each(|_a| {
        let api = hidapi();

        let devices = TransportNativeHID::list_ledgers(api);
        let mut is_empty = true;
        devices.for_each(|device| {
            log::info!("{device:?}");
            is_empty = false;
        });

        if is_empty {
            println!("Cannot find any Ledger devices.");
        }
    });
}

pub fn device_info(mut actions: EventReader<InfoDevice>) {
    actions.iter().for_each(|_a| {
        struct Dummy;
        impl LApp for Dummy {
            const CLA: u8 = 0;
        }

        match TransportNativeHID::new(hidapi()) {
            Ok(ledger) => {
                let result = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(Dummy::get_device_info(&ledger))
                    .expect("Error during exchange");
                log::info!("{result:?}");
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    });
}

fn hidapi() -> &'static HidApi {
    static HIDAPI: Lazy<HidApi> = Lazy::new(|| HidApi::new().expect("unable to get HIDAPI"));

    &HIDAPI
}
