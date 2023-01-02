use crate::cli::action::{DeviceActionPlugin, InfoDeviceAction, ListDeviceAction};
use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::{
            event::{EventReader, EventWriter},
            schedule::ParallelSystemDescriptorCoercion,
        },
    },
    web3::device::{self, DevicePlugin as DevicePluginRaw, InfoDevice, ListDevice},
};

pub struct DevicePlugin;

impl Plugin for DevicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DeviceActionPlugin)
            .add_plugin(DevicePluginRaw)
            .add_system(list_devices.before(device::list_devices))
            .add_system(device_info.before(device::device_info));
    }
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
