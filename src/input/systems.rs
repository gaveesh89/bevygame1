use bevy::prelude::*;
use crate::input::CountrySelectedEvent;
use crate::map::{CountryNode, Selected};

pub fn detect_country_click(
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    country_query: Query<(&CountryNode, &Transform, &Sprite)>,
    windows: Query<&Window>,
    mut events: EventWriter<CountrySelectedEvent>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();
    let Some(world_pos) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        return;
    };
    let world_pos = world_pos.origin.xy();

    for (node, transform, sprite) in country_query.iter() {
        if let Some(size) = sprite.custom_size {
            let half_size = size / 2.0;
            let rect = Rect {
                min: transform.translation.xy() - half_size,
                max: transform.translation.xy() + half_size,
            };

            if rect.contains(world_pos) {
                events.send(CountrySelectedEvent {
                    country_id: node.country_id,
                });
                return;
            }
        }
    }
}

pub fn update_selection(
    mut commands: Commands,
    mut events: EventReader<CountrySelectedEvent>,
    mut previously_selected: Local<Option<Entity>>,
    mut selected_query: Query<Entity, With<Selected>>,
    country_query: Query<(Entity, &CountryNode)>,
) {
    for event in events.read() {
        if let Some(prev_entity) = *previously_selected {
            if let Ok(entity) = selected_query.get_single_mut() {
                commands.entity(entity).remove::<Selected>();
            }
        }

        for (entity, node) in country_query.iter() {
            if node.country_id == event.country_id {
                commands.entity(entity).insert(Selected);
                *previously_selected = Some(entity);
                break;
            }
        }
    }
}
