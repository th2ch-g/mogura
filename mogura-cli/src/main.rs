use bevy::prelude::*;
use bevy_mogura::prelude::*;

mod arg;

fn main() {
    let cli = arg::arg();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: env!("CARGO_PKG_NAME").to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MoguraPlugins {
            input_structure: cli.structure_file,
        })
        .run();
}
