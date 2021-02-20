use crate::{AppState, STAGE};
use bevy::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, AppState::GameOver, setup_gameover.system())
            .on_state_update(STAGE, AppState::GameOver, return_menu.system())
            .on_state_exit(STAGE, AppState::GameOver, cleanup_gameover.system());
    }
}

struct GameOverData;

fn setup_gameover(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "GameOver\nreturn menu Press Space!".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(GameOverData);
}

fn return_menu(mut state: ResMut<State<AppState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set_next(AppState::Menu).unwrap();
    }
}

fn cleanup_gameover(commands: &mut Commands, query: Query<Entity, With<GameOverData>>) {
    for entity in query.iter() {
        commands.despawn_recursive(entity);
    }
}
