use bevy::prelude::*;
use bevy_mogura::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: env!("CARGO_PKG_NAME").to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MoguraPlugins)
        .run();
}
