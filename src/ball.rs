use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, sprite::collide_aabb::Collision};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_magic_light_2d::prelude::*;

use crate::particle::SpawnParticle;
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
        OmniLightSource2D {
            intensity: 0.05,
            color: Color::rgb_u8(255, 255, 255), // randomize on each hit
            falloff: Vec3::new(0.15, 0.25, 0.005),
            ..default()
        },
        LightOccluder2D {
            h_size: Vec2::new(4.0, 4.0),
        },
    ));
}

fn move_ball(mut query: Query<(&Ball, &mut Transform)>, time: Res<Time>) {
    for (ball, mut transform) in &mut query {
        transform.translation += ball.velocity.extend(0.0) * time.delta_seconds();
    }
}

fn ball_paddle_collision(
    mut ball_query: Query<(&mut Ball, &mut Transform, &mut OmniLightSource2D), Without<Paddle>>,
    paddle_query: Query<(&OmniLightSource2D, &Transform), With<Paddle>>,
    mut particle_event: EventWriter<SpawnParticle>,
) {
    let mut ball = ball_query.single_mut();
    let mut ball_transform = ball.1;
    let mut ball_light = ball.2;

    for (paddle_light, paddle_transform) in &paddle_query {
        let paddle_size = Vec2::new(4.0, 16.0);
        let ball_size = Vec2::new(4.0, 4.0);

        let Some(collision) = collide(
            paddle_transform.translation,
            paddle_size,
            ball_transform.translation,
            ball_size,
        ) else {
            continue;
        };

        let offset = ball_transform.translation.y - paddle_transform.translation.y;
        let adjustment = offset / (paddle_size.y / 2.0);

        match collision {
            // top
            Collision::Top => {
                ball_transform.translation.y =
                    paddle_transform.translation.y - paddle_size.y / 2.0 + ball_size.y / 2.0;
                ball.0.velocity.y *= -1.0;
            }
            // bottom
            Collision::Bottom => {
                ball_transform.translation.y =
                    paddle_transform.translation.y + paddle_size.y / 2.0 - ball_size.y / 2.0;
                ball.0.velocity.y *= -1.0;
            }
            // left
            Collision::Left => {
                ball_transform.translation.x =
                    paddle_transform.translation.x + paddle_size.x / 2.0 + ball_size.x / 2.0;

                ball.0.velocity.x *= -1.1;
                ball.0.velocity.x = ball.0.velocity.x.clamp(-100.0, 100.0);
                ball.0.velocity.y += adjustment * 50.0;
            }
            // right
            Collision::Right => {
                ball_transform.translation.x =
                    paddle_transform.translation.x - paddle_size.x / 2.0 - ball_size.x / 2.0;

                ball.0.velocity.x *= -1.1;
                ball.0.velocity.x = ball.0.velocity.x.clamp(-100.0, 100.0);
                ball.0.velocity.y += adjustment * 50.0;
            }
            Collision::Inside => {}
        }

        let particle_rotation = Quat::from_rotation_z(ball.0.velocity.y.atan2(ball.0.velocity.x))
            * -Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)
            * Quat::from_rotation_z(std::f32::consts::PI);

        // particle position is in between the ball and the paddle, calculate
        let particle_position = ball_transform.translation
            + (paddle_transform.translation - ball_transform.translation) / 2.0;

        ball_light.color = paddle_light.color;
        particle_event.send(SpawnParticle {
            position: particle_position,
            rotation: particle_rotation,
        });
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
