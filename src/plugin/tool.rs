use crate::{
    plugin::cli::*,
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
    core::task::{async_action, AsyncActionPool},
};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ToolActionPlugin)
            .add_plugin(AsyncActionPlugin)
            .add_system(handle_list_tool)
            .add_system(handle_add_tool)
            .add_system(handle_install);
    }
}

fn handle_list_tool(mut events: EventReader<ListToolAction>, mut app_exit: EventWriter<AppExit>) {
    for _ in events.iter() {
        for t in Tool::list().iter() {
            println!("- {t}");
        }
        app_exit.send(AppExit);
    }
}

fn handle_add_tool(
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

fn handle_install(
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

#[async_action]
impl AsyncActionCreator {
    async fn install(tool: Tool) -> ToolResult<Install> {
        tool.install().await?;

        Ok(Install)
    }
}

#[derive(Clone, Debug)]
pub struct Install;
