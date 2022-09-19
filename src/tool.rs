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

fn handle_list_tool(mut events: EventReader<ListToolAction>) {
    for _ in events.iter() {
        for tool in list_tools().iter() {
            println!("- {tool}");
        }
    }
}

fn handle_add_tool(mut events: EventReader<AddToolAction>) {
    for e in events.iter() {
        match e.name.as_str() {
            "tailwindcss" => {
                log::info!("{e:?}");
            }
            name => {
                log::error!("Could not find tool: {name}");
            }
        }
    }
}

fn list_tools() -> Vec<&'static str> {
    vec!["tailwindcss"]
}
