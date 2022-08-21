use bevy::ecs::prelude::*;
use chrono::{DateTime, Utc};

// Component Bundle
#[derive(Bundle, Default)]
pub struct TodoBundle {
    label: Todo,
    title: Title,

    timestamp: Timestamp,
}

impl From<String> for TodoBundle {
    fn from(title: String) -> Self {
        Self {
            title: Title::from(title),
            ..Default::default()
        }
    }
}

// Components
#[derive(Component, Clone, Default)]
pub struct Todo;

#[derive(Component, Clone, Default)]
pub struct Title {
    pub value: String,
}

impl From<String> for Title {
    fn from(value: String) -> Self {
        Self { value }
    }
}

#[derive(Component, Clone, Debug)]
pub struct DoneAt {
    pub time: DateTime<Utc>,
}

impl Default for DoneAt {
    fn default() -> Self {
        Self { time: Utc::now() }
    }
}

#[derive(Component)]
pub struct Timestamp {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Timestamp {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
        }
    }
}

impl Timestamp {
    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }
}
