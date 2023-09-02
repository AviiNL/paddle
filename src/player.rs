use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::{AppState, Paddle, Score};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems(Startup, setup)
            .add_systems(Update, player_controller.run_if(in_state(AppState::Game)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Left Paddle"),
        Paddle { speed: 100.0 },
        Player,
        Score::default(),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-100.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(4.0, 16.0)),
                anchor: default(),
                ..default()
            },
            ..Default::default()
        },
    ));
}

fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    for (paddle, mut transform) in &mut query {
        let speed = paddle.speed * time.delta_seconds();

        if (keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up))
            && transform.translation.y < 64.0
        {
            transform.translation.y += speed;
        }
        if (keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down))
            && transform.translation.y > -64.0
        {
            transform.translation.y -= speed;
        }
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player;
