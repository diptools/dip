mod action;
use crate::cli::action::{
    // ActionPlugin,
    // ApplyBundleAction,
    // BundleActionPlugin,
    // CleanBundleAction,
    CliPlugin,
    //     DeviceActionPlugin, InfoDeviceAction, ListDeviceAction,
};

use dip::{
    bevy::{
        app::{App, Plugin},
        // ecs::{
        //     event::{EventReader, EventWriter},
        //     // schedule::ParallelSystemDescriptorCoercion,
        //     system::ResMut,
        // },
    },
    // bundle::{ApplyBundle, BundleConfig, BundlePlugin, CleanBundle},
    // core::task::NoAsyncAction,
    // web3::device::{self, InfoDevice, ListDevice},
};
// use dip_device::DevicePlugin;

pub struct DipCliPlugin;

impl Plugin for DipCliPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CliPlugin::oneshot());
        // .add_plugins(ActionPlugin)
        // .add_plugin(BundlePlugin)
        // .add_plugin(BundleActionPlugin)
        // .add_system(install_bundle)
        // .add_system(clean_bundle)
        // .add_plugin(DeviceActionPlugin)
        // .add_plugin(DevicePlugin)
        // .add_system(list_devices.before(device::list_devices))
        // .add_system(device_info.before(device::device_info));
    }
}

// fn install_bundle(
//     mut actions: EventReader<ApplyBundleAction>,
//     mut apply: EventWriter<ApplyBundle>,
//     mut config: ResMut<BundleConfig>,
// ) {
//     actions.iter().for_each(|a| {
//         if let Some(value) = &a.bundle_root {
//             config
//                 .set_bundle_root(&value)
//                 .expect("Bundle root is not a directry");
//         }

//         apply.send(ApplyBundle);
//     });
// }

// fn clean_bundle(
//     mut actions: EventReader<CleanBundleAction>,
//     mut clean: EventWriter<CleanBundle>,
//     mut config: ResMut<BundleConfig>,
// ) {
//     actions.iter().for_each(|a| {
//         if let Some(value) = &a.bundle_root {
//             config
//                 .set_bundle_root(&value)
//                 .expect("Bundle root is not a directry");
//         }

//         clean.send(CleanBundle)
//     });
// }

// fn list_devices(
//     mut actions: EventReader<ListDeviceAction>,
//     mut list_device: EventWriter<ListDevice>,
// ) {
//     actions.iter().for_each(|_a| {
//         list_device.send(ListDevice);
//     });
// }

// fn device_info(
//     mut actions: EventReader<InfoDeviceAction>,
//     mut info_device: EventWriter<InfoDevice>,
// ) {
//     actions.iter().for_each(|_a| {
//         info_device.send(InfoDevice);
//     });
// }
