use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::event::EventReader,
        log,
    },
    cli::SubcommandPlugin,
};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ToolActionPlugin)
            .add_system(handle_list_tool)
            .add_system(handle_add_tool);
    }
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum ToolAction {
    List,
    Add(AddAction),
}

#[derive(clap::Args, Clone, Debug)]
pub struct AddAction {
    name: String,
}

pub fn handle_list_tool(mut events: EventReader<ListToolAction>) {
    for e in events.iter() {
        log::info!("{e:?}");
    }
}

pub fn handle_add_tool(mut events: EventReader<AddToolAction>) {
    for e in events.iter() {
        match e.name.as_str() {
            "tailwind" => {
                log::info!("{e:?}");
            }
            _ => {}
        }
    }
}
