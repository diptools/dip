use dip::{
    bevy::{ecs::event::EventReader, log},
    cli::Subcommand,
};

#[derive(Clone, Debug, Subcommand, clap::Subcommand)]
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
