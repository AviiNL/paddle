use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::{ball::Ball, AppState, Paddle, Score};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AI>()
            .add_systems(Startup, setup)
            .add_systems(Update, ai_controller.run_if(in_state(AppState::Game)));
    }
}

fn setup(mut commands: Commands) {
    // Create the right paddle
    commands.spawn((
        Name::new("Right Paddle"),
        Paddle { speed: 50.0 },
        AI,
        Score::default(),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
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

fn ai_controller(
    mut query: Query<(&Paddle, &mut Transform), With<AI>>,
    ball_query: Query<(&Ball, &Transform), Without<AI>>,
    time: Res<Time>,
) {
    let ball = ball_query.single();
    let ball_transform = ball.1;

    for (paddle, mut transform) in &mut query {
        let speed = paddle.speed * time.delta_seconds();

        if ball_transform.translation.y > transform.translation.y && transform.translation.y < 64.0
        {
            transform.translation.y += speed;
        }
        if ball_transform.translation.y < transform.translation.y && transform.translation.y > -64.0
        {
            transform.translation.y -= speed;
        }
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct AI;
