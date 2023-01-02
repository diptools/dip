mod action;
mod build;
mod bundle;
mod device;

use self::{
    action::{ActionPlugin, CliPlugin},
    build::BuildPlugin,
    bundle::BundlePlugin,
    device::DevicePlugin,
};
use dip::{
    bevy::app::{App, Plugin},
    core::task::NoAsyncAction,
};

pub struct DipCliPlugin;

impl Plugin for DipCliPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CliPlugin::<NoAsyncAction>::oneshot())
            .add_plugin(ActionPlugin)
            .add_plugin(BuildPlugin)
            .add_plugin(BundlePlugin)
            .add_plugin(DevicePlugin);
    }
}
