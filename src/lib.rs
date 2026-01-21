mod map;
mod input;
mod ui;

use bevy::prelude::*;
use map::MapPlugin;
use input::InputPlugin;
use ui::UIPlugin;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Farming Simulation - World Map".into(),
                resolution: (1400.0, 900.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MapPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(UIPlugin)
        .run();
}
