use bevy::prelude::*;
use crate::input::CountrySelectedEvent;
use crate::map::{CountryDatabase, CountryNode, Selected};
use crate::ui::CountryNameText;

#[derive(Resource)]
pub struct SelectedCountryInfo {
    pub country_id: Option<u16>,
}

impl Default for SelectedCountryInfo {
    fn default() -> Self {
        Self { country_id: None }
    }
}

pub fn spawn_ui(
    mut commands: Commands,
) {
    commands.init_resource::<SelectedCountryInfo>();

    commands
        .spawn(
            TextBundle::from_section(
                "Selected Country: None | Continent: None",
                TextSection {
                    value: "Selected Country: None | Continent: None".to_string(),
                    style: TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                ..default()
            }),
        )
        .insert(CountryNameText);
}

pub fn update_ui_text(
    mut events: EventReader<CountrySelectedEvent>,
    db: Res<CountryDatabase>,
    mut info: ResMut<SelectedCountryInfo>,
    mut text_query: Query<&mut Text, With<CountryNameText>>,
) {
    for event in events.read() {
        if let Some(country) = db.get_country(event.country_id) {
            info.country_id = Some(event.country_id);
            
            if let Ok(mut text) = text_query.get_single_mut() {
                let continent_name = match country.continent {
                    crate::map::Continent::Africa => "Africa",
                    crate::map::Continent::Asia => "Asia",
                    crate::map::Continent::Europe => "Europe",
                    crate::map::Continent::NorthAmerica => "North America",
                    crate::map::Continent::SouthAmerica => "South America",
                    crate::map::Continent::Oceania => "Oceania",
                };

                text.sections[0].value = format!(
                    "Selected Country: {} | Continent: {}",
                    country.name, continent_name
                );
            }
        }
    }
}
