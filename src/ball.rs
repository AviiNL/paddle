use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::{AppState, Paddle};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ball>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (move_ball, ball_paddle_collision, ball_bounds_collision)
                    .run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Ball {
    pub velocity: Vec2,
}

fn setup(mut commands: Commands) {
    // create the ball, center screen 4x4px
    commands.spawn((
        Name::new("Ball"),
        Ball {
            velocity: Vec2::new(-50.0, 0.0),
        },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(4.0, 4.0)),
                anchor: default(),
                ..default()
            },
            ..Default::default()
        },
    ));
}

fn move_ball(mut query: Query<(&Ball, &mut Transform)>, time: Res<Time>) {
    for (ball, mut transform) in &mut query {
        transform.translation += ball.velocity.extend(0.0) * time.delta_seconds();
    }
}

fn ball_paddle_collision(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    paddle_query: Query<&Transform, With<Paddle>>,
) {
    let mut ball = ball_query.single_mut();
    let ball_transform = ball.1;

    for paddle_transform in &paddle_query {
        let paddle_size = Vec2::new(4.0, 16.0);
        let ball_size = Vec2::new(4.0, 4.0);

        let paddle_top = paddle_transform.translation.y + paddle_size.y / 2.0;
        let paddle_bottom = paddle_transform.translation.y - paddle_size.y / 2.0;
        let paddle_left = paddle_transform.translation.x - paddle_size.x / 2.0;
        let paddle_right = paddle_transform.translation.x + paddle_size.x / 2.0;

        let ball_top = ball_transform.translation.y + ball_size.y / 2.0;
        let ball_bottom = ball_transform.translation.y - ball_size.y / 2.0;
        let ball_left = ball_transform.translation.x - ball_size.x / 2.0;
        let ball_right = ball_transform.translation.x + ball_size.x / 2.0;

        if ball_bottom < paddle_top
            && ball_top > paddle_bottom
            && ball_left < paddle_right
            && ball_right > paddle_left
        {
            // how far from the center of the paddle did we hit?
            let offset = ball_transform.translation.y - paddle_transform.translation.y;

            // how far should we move the ball away from the paddle?
            let adjustment = offset / (paddle_size.y / 2.0);

            // adjust the velocity of the ball
            ball.0.velocity.x *= -1.1;
            // clamp the x velocity between -100 and 100
            ball.0.velocity.x = ball.0.velocity.x.clamp(-100.0, 100.0);

            ball.0.velocity.y += adjustment * 50.0;
        }
    }
}

fn ball_bounds_collision(mut ball_query: Query<(&mut Ball, &Transform)>) {
    // y bounds, -64 ~ 64

    let mut ball = ball_query.single_mut();
    let ball_transform = ball.1;

    let ball_size = Vec2::new(4.0, 4.0);

    let ball_top = ball_transform.translation.y + ball_size.y / 2.0;
    let ball_bottom = ball_transform.translation.y - ball_size.y / 2.0;

    if ball_top > 72.0 || ball_bottom < -72.0 {
        ball.0.velocity.y *= -1.0;
    }
}
