use crate::{
    plugin::{action::*, async_action::*},
    resource::tool::{Tool, ToolResult},
};
use dip::{
    bevy::{
        app::{App, AppExit, Plugin},
        ecs::{
            event::{EventReader, EventWriter},
            system::Res,
        },
    },
    core::task::AsyncActionPool,
};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ToolActionPlugin)
            .add_system(list_tool)
            .add_system(add_tool)
            .add_system(install_result);
    }
}

fn list_tool(mut events: EventReader<ListToolAction>, mut app_exit: EventWriter<AppExit>) {
    for _ in events.iter() {
        for t in Tool::list().iter() {
            println!("- {t}");
        }
        app_exit.send(AppExit);
    }
}

fn add_tool(
    mut events: EventReader<AddToolAction>,
    async_action: Res<AsyncActionPool<AsyncAction>>,
) {
    for e in events.iter() {
        let name = e.name.as_str();
        let tool = Tool::from_str(name).expect(&format!("Could not find tool: {name}"));

        match tool {
            Tool::Tailwind => async_action.send(AsyncAction::install(tool)),
        }
    }
}

fn install_result(
    mut events: EventReader<ToolResult<Install>>,
    mut app_exit: EventWriter<AppExit>,
) {
    for e in events.iter() {
        if let Err(e) = e {
            println!("{:?}", e.error);
        }
        app_exit.send(AppExit);
    }
}
