use dip::{
    bevy::{ecs::event::EventReader, log},
    cli::SubcommandPlugin,
};

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum ToolAction {
    Install,
}

impl Default for ToolAction {
    fn default() -> Self {
        Self::Install
    }
}

pub fn handle_tool_install(mut events: EventReader<InstallToolAction>) {
    for e in events.iter() {
        log::info!("{e:#?}");
    }
}
