use bevy::prelude::*;

const STAGE: &str = "app";
#[derive(Clone)]
enum AppState {
    Menu,
    InGame,
    GameOver,
}

#[derive(Debug)]
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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0 ,0.0).into()),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .with(Movement {
            speed: 0.0,
            gravity: -0.1,
        })
        .with(Player);
    commands.insert_resource(PlayerData {
        player_entity: commands.current_entity().unwrap(),
    });
    
    // bottom wall
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5 ,0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -250.0, 0.0)),
            sprite: Sprite::new(Vec2::new(500.0, 50.0)),
            ..Default::default()
        })
        .with(Obstacle);
    commands.insert_resource(BottomWallData {
        wall_entity: commands.current_entity().unwrap(),
    });
    
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5 ,0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 250.0, 0.0)),
            sprite: Sprite::new(Vec2::new(500.0, 50.0)),
            ..Default::default()
        })
        .with(Obstacle);

    commands.insert_resource(TopWallData {
        wall_entity: commands.current_entity().unwrap(),
    });
}


struct Player;
struct Obstacle;

#[derive(Debug)]
struct PlayerData {
    player_entity: Entity,
}

struct TopWallData {
    wall_entity: Entity,
}

struct BottomWallData {
    wall_entity: Entity,
}

struct Movement {
    speed: f32,
    gravity: f32,
}

fn flapping_wings(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut query: Query<(&mut Transform, &mut Movement), With<Player>>,
) {
    for (mut transform, mut movement) in query.iter_mut() {
        movement.speed += movement.gravity;

        if keyboard_input.just_pressed(KeyCode::Space) {
            movement.speed = 3.0;
        }
        transform.translation.y += movement.speed;
        transform.translation.y = transform.translation.y.min(200.0).max(-200.0);

        if transform.translation.y == 200.0 || transform.translation.y == -200.0 {
            state.set_next(AppState::GameOver).unwrap();
        }
    }
}

fn cleanup_ingame(
    commands: &mut Commands,
    player_data: Res<PlayerData>,
    bottom_wall_data: Res<BottomWallData>,
    top_wall_data: Res<TopWallData>,
) {
    commands.despawn_recursive(player_data.player_entity);
    commands.despawn_recursive(bottom_wall_data.wall_entity);
    commands.despawn_recursive(top_wall_data.wall_entity);
}

struct GameOverData {
    text_entity: Entity,
}

fn setup_gameover(
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
                value: "GameOver\nreturn menu Press Space!".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..Default::default()
                }
            },
            ..Default::default()
        });
    commands.insert_resource(GameOverData {
        text_entity: commands.current_entity().unwrap()
    });
}

fn return_menu(
    mut state: ResMut<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set_next(AppState::Menu).unwrap();
    }
}

fn cleanup_gameover(
    commands: &mut Commands,
    gameover_data: Res<GameOverData>
) {
    commands.despawn_recursive(gameover_data.text_entity);
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: 500.0,
            height: 500.0,
            resizable: false,
            title: "Flappy Bevy".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_resource(State::new(AppState::Menu))
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
        .on_state_enter(STAGE, AppState::Menu, setup_menu.system())
        .on_state_update(STAGE, AppState::Menu, to_in_game.system())
        .on_state_exit(STAGE, AppState::Menu, cleanup_menu.system())
        .on_state_enter(STAGE, AppState::InGame, setup_game.system())
        .on_state_update(STAGE, AppState::InGame, flapping_wings.system())
        .on_state_exit(STAGE, AppState::InGame, cleanup_ingame.system())
        .on_state_enter(STAGE, AppState::GameOver, setup_gameover.system())
        .on_state_update(STAGE, AppState::GameOver, return_menu.system())  
        .on_state_exit(STAGE, AppState::GameOver, cleanup_gameover.system())      
        .run();
}
