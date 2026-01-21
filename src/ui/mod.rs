pub mod systems;

pub use systems::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct CountryNameText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, update_ui_text);
    }
}
