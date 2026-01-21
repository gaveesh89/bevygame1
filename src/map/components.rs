use bevy::prelude::*;

#[derive(Component)]
pub struct CountryNode {
    pub country_id: u16,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct MapCamera;
