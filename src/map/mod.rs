pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CountryDatabase::new())
            .add_systems(Startup, spawn_map)
            .add_systems(Update, highlight_selected_country);
    }
}
