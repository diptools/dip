use crate::{
    plugin::action::BuildAction,
    resource::tool::{Tool, ToolResult},
};
use dip::{bevy::ecs::event::EventReader, core::task::async_action};

#[async_action]
impl AsyncActionCreator {
    async fn install(tool: Tool) -> ToolResult<Install> {
        tool.install().await?;

        Ok(Install)
    }

    async fn install_and_build(tool: &Tool, action: BuildAction) -> BuildAction {
        tool.install().await.unwrap();

        action
    }
}

#[derive(Clone, Debug)]
pub struct Install;
