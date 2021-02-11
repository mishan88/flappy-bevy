use bevy::prelude::*;

const STAGE: &str = "app";
#[derive(Clone)]
enum AppState {
    Menu,
    InGame,
}

struct MenuData {
    text_entity: Entity,
}

fn setup_menu(
    commands: &mut Commands,
    asset_server: Res<AssetServer>
) {
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
                }
            },
            ..Default::default()
        });
    commands.insert_resource(MenuData {
        text_entity: commands.current_entity().unwrap()
    });
}

fn to_in_game(
    mut state: ResMut<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set_next(AppState::InGame).unwrap();
    }
}

fn cleanup_menu(
    commands: &mut Commands,
    menu_data: Res<MenuData>
) {
    commands.despawn_recursive(menu_data.text_entity);
}

fn setup_game(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("branding/bigusu.png");
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .with(Movement {
            speed: 0.0,
            gravity: -0.1,
        })
        .with(Player);
}


struct Player;

struct Movement {
    speed: f32,
    gravity: f32,
}

fn flapping_wings(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Movement), With<Player>>,
) {
    for (mut transform, mut movement) in query.iter_mut() {
        movement.speed += movement.gravity;

        if keyboard_input.just_pressed(KeyCode::Space) {
            movement.speed = 3.0;
        }
        transform.translation.y += movement.speed;
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(WindowDescriptor {
            width: 500.0,
            height: 500.0,
            title: "Flappy Bevy".to_string(),
            ..Default::default()
        })
        .add_resource(State::new(AppState::Menu))
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
        .on_state_enter(STAGE, AppState::Menu, setup_menu.system())
        .on_state_update(STAGE, AppState::Menu, to_in_game.system())
        .on_state_exit(STAGE, AppState::Menu, cleanup_menu.system())
        .on_state_enter(STAGE, AppState::InGame, setup_game.system())
        .on_state_update(STAGE, AppState::InGame, flapping_wings.system())
        .run();
}
