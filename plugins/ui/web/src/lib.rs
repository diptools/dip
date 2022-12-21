#![allow(non_snake_case)]

use bevy::app::{App, Plugin};
use dioxus_core::Component as DioxusComponent;

pub mod prelude {
    pub use crate::WebPlugin;
}

pub struct WebPlugin<RootProps = ()> {
    /// Root component
    pub Root: DioxusComponent<RootProps>,
}

impl Plugin for WebPlugin {
    fn build(&self, app: &mut App) {
        let Root = self.Root.clone();

        app.add_system(setup).set_runner(move |_app| {
            dioxus_web::launch(Root);
        });
    }
}

impl<RootProps> WebPlugin<RootProps> {
    pub fn new(Root: DioxusComponent<RootProps>) -> Self {
        Self { Root }
    }
}

fn setup() {
    println!("hey");
}
