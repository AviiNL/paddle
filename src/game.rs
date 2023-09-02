use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::{
    ai::{AiPlugin, AI},
    ball::{Ball, BallPlugin},
    player::{Player, PlayerPlugin},
    ui::GameUiPlugin,
    AppState,
};

#[derive(Event)]
pub struct GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_plugins(GameUiPlugin)
            .add_plugins(BallPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(AiPlugin)
            .register_type::<Paddle>()
            .register_type::<Score>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (check_goal, check_win).run_if(in_state(AppState::Game)),
            );
    }
}

fn setup(mut commands: Commands) {
    // create the center line, 2px width, dashed line
    commands.spawn((
        Name::new("Center Line"),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1.0, 144.0)),
                anchor: default(),
                ..default()
            },
            ..Default::default()
        },
    ));
}

fn check_goal(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    mut player_score: Query<&mut Score, With<Player>>,
    mut ai_score: Query<&mut Score, (With<AI>, Without<Player>)>,
    mut paddles: Query<&mut Transform, (With<Paddle>, Without<Ball>)>,
) {
    let mut ball = ball_query.single_mut();

    let ball_size = Vec2::new(4.0, 4.0);

    let ball_left = ball.1.translation.x - ball_size.x / 2.0;
    let ball_right = ball.1.translation.x + ball_size.x / 2.0;

    if ball_left < -128.0 {
        info!("Player 2 scores!");
        // reset ball
        ball.0.velocity = Vec2::new(-50.0, 0.0);
        ball.1.translation = Vec3::new(0.0, 0.0, 0.0);
        ai_score.single_mut().value += 1;

        // reset all paddles y to 0
        for mut paddle_transform in &mut paddles {
            paddle_transform.translation.y = 0.0;
        }
    }
    if ball_right > 128.0 {
        info!("Player 1 scores!");
        // reset ball
        ball.0.velocity = Vec2::new(50.0, 0.0);
        ball.1.translation = Vec3::new(0.0, 0.0, 0.0);
        player_score.single_mut().value += 1;

        // reset all paddles y to 0
        for mut paddle_transform in &mut paddles {
            paddle_transform.translation.y = 0.0;
        }
    }
}

/// If a player has 11 points, they win!
fn check_win(
    mut player_score: Query<&mut Score, With<Player>>,
    mut ai_score: Query<&mut Score, (With<AI>, Without<Player>)>,
) {
    let mut player_score = player_score.single_mut();
    let mut ai_score = ai_score.single_mut();

    if player_score.value == 11 {
        info!("Player 1 wins!");
        player_score.value = 0;
        ai_score.value = 0;
    }
    if ai_score.value == 11 {
        info!("Player 2 wins!");
        player_score.value = 0;
        ai_score.value = 0;
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Paddle {
    pub speed: f32,
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Score {
    pub value: u32,
}
