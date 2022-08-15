//! UiStage for Bevy

use bevy::ecs::prelude::*;

/// The names of the default [`Ui`] stages.
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum UiStage {
    /// The [`Stage`](bevy_ecs::schedule::Stage) that runs before all other ui stages.
    First,
    /// The [`Stage`](bevy_ecs::schedule::Stage) that runs before [`UiStage::Update`].
    PreUpdate,
    /// The [`Stage`](bevy_ecs::schedule::Stage) responsible for doing most ui logic. Systems should be registered here by default.
    Update,
    /// The [`Stage`](bevy_ecs::schedule::Stage) that runs after [`UiStage::Update`].
    PostUpdate,
    /// The [`Stage`](bevy_ecs::schedule::Stage) that runs after all other ui stages.
    Last,
}
