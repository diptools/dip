use crate::resource::tool::{Tool, ToolResult};
use dip::{bevy::ecs::event::EventReader, core::task::async_action};

#[async_action]
impl AsyncActionCreator {
    async fn install(tool: Tool) -> ToolResult<Install> {
        tool.install().await?;

        Ok(Install)
    }

    async fn install_and_build(tool: &Tool) -> ToolResult<InstallAndBuild> {
        tool.install().await?;

        Ok(InstallAndBuild)
    }
}

#[derive(Clone, Debug)]
pub struct Install;

#[derive(Clone, Debug)]
pub struct InstallAndBuild;
