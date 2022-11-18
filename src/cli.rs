mod action;

<<<<<<< HEAD
use crate::{
    cli::action::{
        ActionPlugin, ApplyBundleAction, BundleActionPlugin, CleanBundleAction, CliPlugin,
    },
    config::DipConfig,
=======
use crate::cli::action::{
    ActionPlugin, ApplyBundleAction, BundleActionPlugin, CleanBundleAction, CliPlugin,
<<<<<<< HEAD
>>>>>>> 0a64aae (Replace ConfigPlugin with BundleConfigPlugin)
=======
    DeviceActionPlugin, InfoDeviceAction, ListDeviceAction,
>>>>>>> 24b4c2e (It's totally unrelated but succeeded to reteive Ledger device info)
};

use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::{
            event::{EventReader, EventWriter},
            schedule::ParallelSystemDescriptorCoercion,
            system::ResMut,
        },
    },
    bundle::{ApplyBundle, BundleConfig, BundlePlugin, CleanBundle},
    core::task::NoAsyncAction,
    web3::device::{self, InfoDevice, ListDevice},
};
use dip_device::DevicePlugin;

pub struct DipCliPlugin;

impl Plugin for DipCliPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CliPlugin::<NoAsyncAction>::oneshot())
            .add_plugin(ActionPlugin)
            .add_plugin(BundlePlugin)
            .add_plugin(BundleActionPlugin)
<<<<<<< HEAD
            .add_plugin(BundlePlugin::<DipConfig>::new())
=======
>>>>>>> 0a64aae (Replace ConfigPlugin with BundleConfigPlugin)
            .add_system(install_bundle)
            .add_system(clean_bundle)
            .add_plugin(DeviceActionPlugin)
            .add_plugin(DevicePlugin)
            .add_system(list_devices.before(device::list_devices))
            .add_system(device_info.before(device::device_info));
    }
}

fn install_bundle(
    mut actions: EventReader<ApplyBundleAction>,
    mut apply: EventWriter<ApplyBundle>,
    mut config: ResMut<BundleConfig>,
) {
    actions.iter().for_each(|a| {
<<<<<<< HEAD
<<<<<<< HEAD
        apply.send(ApplyBundle {
            path: PathBuf::from(&a.path),
        });
=======
        if let Some(value) = a.repo.clone() {
=======
        if let Some(value) = &a.bundle_root {
>>>>>>> 24b4c2e (It's totally unrelated but succeeded to reteive Ledger device info)
            config
                .set_bundle_root(&value)
                .expect("Bundle root is not a directry");
        }

        apply.send(ApplyBundle);
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)
    });
}

fn clean_bundle(
    mut actions: EventReader<CleanBundleAction>,
    mut clean: EventWriter<CleanBundle>,
    mut config: ResMut<BundleConfig>,
) {
    actions.iter().for_each(|a| {
<<<<<<< HEAD
<<<<<<< HEAD
        clean.send(CleanBundle {
            path: PathBuf::from(&a.path),
        })
=======
        if let Some(value) = a.repo.clone() {
=======
        if let Some(value) = &a.bundle_root {
>>>>>>> 24b4c2e (It's totally unrelated but succeeded to reteive Ledger device info)
            config
                .set_bundle_root(&value)
                .expect("Bundle root is not a directry");
        }

        clean.send(CleanBundle)
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)
    });
}

fn list_devices(
    mut actions: EventReader<ListDeviceAction>,
    mut list_device: EventWriter<ListDevice>,
) {
    actions.iter().for_each(|_a| {
        list_device.send(ListDevice);
    });
}

fn device_info(
    mut actions: EventReader<InfoDeviceAction>,
    mut info_device: EventWriter<InfoDevice>,
) {
    actions.iter().for_each(|_a| {
        info_device.send(InfoDevice);
    });
}
