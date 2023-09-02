use bevy::prelude::*;

use crate::{ai::AI, game::Score, player::Player, AppState};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_score.run_if(in_state(AppState::Game)));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    justify_items: JustifyItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                // background_color: Color::BLUE.into(),
                ..default()
            },
        ))
        .with_children(|c| {
            c.spawn((
                Name::new("lblScore"),
                ScoreLabel,
                TextBundle {
                    text: Text::from_section(
                        "0   0",
                        TextStyle {
                            font_size: 48.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
            ));
        });
}

fn update_score(
    mut query: Query<&mut Text, With<ScoreLabel>>,
    player_score: Query<&Score, With<Player>>,
    ai_score: Query<&Score, With<AI>>,
) {
    for mut text in &mut query {
        let player_score = player_score.single().value;
        let ai_score = ai_score.single().value;

        text.sections[0].value = format!("{}   {}", player_score, ai_score);
    }
}

#[derive(Component)]
pub struct ScoreLabel;
