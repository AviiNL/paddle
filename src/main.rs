// #![windows_subsystem = "windows"]

mod ai;
mod ball;
mod game;
mod particle;
mod player;
mod ui;

use game::*;

use bevy::{
    input::{common_conditions::input_toggle_active, keyboard::KeyboardInput, ButtonState},
    prelude::*,
    render::{
        camera::ScalingMode,
        settings::{WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use particle::ParticlePlugin;

fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(RenderPlugin { wgpu_settings })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Paddle".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<AppState>()
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .add_plugins(ParticlePlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, camera)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .run();
}

pub fn camera(mut commands: Commands) {
    // Spawn a 2D camera
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);
}

pub fn transition_to_game_state(
    mut key_evr: EventReader<KeyboardInput>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if *app_state.get() == AppState::Game {
        return;
    }

    for ev in key_evr.iter() {
        if matches!(ev.state, ButtonState::Pressed) {
            if ev.key_code == Some(KeyCode::Escape) {
                app_exit_events.send(bevy::app::AppExit);
                return;
            }

            app_state_next_state.set(AppState::Game);
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if *app_state.get() == AppState::MainMenu {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state_next_state.set(AppState::MainMenu);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
