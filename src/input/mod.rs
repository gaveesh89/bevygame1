pub mod systems;

pub use systems::*;

use bevy::prelude::*;

#[derive(Event)]
pub struct CountrySelectedEvent {
    pub country_id: u16,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CountrySelectedEvent>()
            .add_systems(Update, (detect_country_click, update_selection));
    }
}
