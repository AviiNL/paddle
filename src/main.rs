#![windows_subsystem = "windows"]

mod ai;
mod ball;
mod game;
mod player;
mod ui;

use game::*;

use bevy::{
    input::{common_conditions::input_toggle_active, keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
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
        .add_state::<AppState>()
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .add_plugins(GamePlugin)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .run();
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
