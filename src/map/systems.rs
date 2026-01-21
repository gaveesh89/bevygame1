use bevy::prelude::*;
use crate::map::{CountryNode, CountryDatabase, Selected, MapCamera};

pub fn spawn_map(
    mut commands: Commands,
    db: Res<CountryDatabase>,
) {
    commands.spawn(Camera2d::default()).insert(MapCamera);

    for country in &db.countries {
        let color = Color::srgb(0.2, 0.6, 0.8);
        
        commands
            .spawn(Sprite {
                color,
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            })
            .insert(Transform::from_translation(country.map_position.extend(0.0)))
            .insert(CountryNode {
                country_id: country.id,
            });
    }
}

pub fn highlight_selected_country(
    mut query: Query<(&CountryNode, &mut Sprite), Changed<Sprite>>,
    selected_query: Query<(), With<Selected>>,
) {
    for (node, mut sprite) in query.iter_mut() {
        if selected_query.contains(Entity::from_raw(node.country_id as u32)) {
            sprite.color = Color::srgb(1.0, 0.84, 0.0);
        } else {
            sprite.color = Color::srgb(0.2, 0.6, 0.8);
        }
    }
}
