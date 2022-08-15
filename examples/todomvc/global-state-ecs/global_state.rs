use crate::component::*;
use bevy::prelude::*;
use bevy_dioxus::core::prelude::*;
use chrono::{DateTime, Utc};

#[derive(GlobalStateCommand, Clone, Debug)]
pub enum GlobalStateCommand {
    TodoList(Vec<UiTodo>),
}

#[derive(Component, Clone, Debug)]
pub struct UiTodo {
    pub entity: Entity,
    pub title: String,
    pub done_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<(Entity, &Title, Option<&DoneAt>, &Timestamp)> for UiTodo {
    fn from(
        (entity, title, done_at, timestamp): (Entity, &Title, Option<&DoneAt>, &Timestamp),
    ) -> Self {
        Self {
            entity,
            title: title.value.clone(),
            done_at: match done_at {
                Some(done_at) => Some(done_at.time),
                None => None,
            },
            created_at: timestamp.created_at,
            updated_at: timestamp.updated_at,
        }
    }
}

// Generated
// use bevy_dioxus::desktop::event::VDomCommand;
// use futures_intrusive::channel::{shared::Sender, TrySendError};
// use std::rc::Rc;

// pub static TODO_LIST: Atom<Vec<UiTodo>> = |_| vec![];

// impl GlobalStateHandler for GlobalStateCommand {
//     fn handler(self, root: Rc<AtomRoot>) {
//         match self {
//             GlobalStateCommand::TodoList(x) => root.set(TODO_LIST.unique_id(), x),
//         }
//     }
// }

// pub struct GlobalStatePlugin;

// impl Plugin for GlobalStatePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_event::<GlobalStateCommand>()
//             .add_system(apply_global_state_command);
//     }
// }

// fn apply_global_state_command(
//     mut events: EventReader<GlobalStateCommand>,
//     vdom_tx: Res<Sender<VDomCommand<GlobalStateCommand>>>,
// ) {
//     for e in events.iter() {
//         match vdom_tx.try_send(VDomCommand::GlobalState(e.clone())) {
//             Ok(()) => {}
//             Err(e) => match e {
//                 TrySendError::Full(e) => {
//                     error!(
//                         "Failed to send VDomCommand: channel is full: event: {:?}",
//                         e
//                     );
//                 }
//                 TrySendError::Closed(e) => {
//                     error!(
//                         "Failed to send VDomCommand: channel is closed: event: {:?}",
//                         e
//                     );
//                 }
//             },
//         }
//     }
// }
