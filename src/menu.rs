use crate::{AppState, STAGE};
use bevy::prelude::*;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, AppState::Menu, setup_menu.system())
            .on_state_update(STAGE, AppState::Menu, to_in_game.system())
            .on_state_exit(STAGE, AppState::Menu, cleanup_menu.system());
    }
}

#[derive(Debug)]
struct MenuData;

fn setup_menu(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "Press Space!".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(MenuData);
}

fn to_in_game(mut state: ResMut<State<AppState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set_next(AppState::InGame).unwrap();
    }
}

fn cleanup_menu(commands: &mut Commands, query: Query<Entity, With<MenuData>>) {
    for entity in query.iter() {
        commands.despawn_recursive(entity);
    }
}
