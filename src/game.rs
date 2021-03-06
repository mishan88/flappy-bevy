use crate::{AppState, STAGE};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, AppState::InGame, setup_game.system())
            .on_state_update(STAGE, AppState::InGame, flapping_wings.system())
            .on_state_update(STAGE, AppState::InGame, spawn_flyingobstacle.system())
            .on_state_update(STAGE, AppState::InGame, move_flyingobstcle.system())
            .on_state_update(STAGE, AppState::InGame, collide_flyingobstacle.system())
            .on_state_update(STAGE, AppState::InGame, text_score_update.system())
            .on_state_exit(STAGE, AppState::InGame, cleanup_ingame.system());
    }
}

type GameEntity = Or<(
    With<Player>,
    With<Obstacle>,
    With<FlyingObstacle>,
    With<ScoreText>,
)>;

fn setup_game(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(CameraUiBundle::default());
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .with(Movement {
            speed: 0.0,
            gravity: -0.1,
        })
        .with(Player);

    // bottom wall
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -250.0, 0.0)),
            sprite: Sprite::new(Vec2::new(500.0, 50.0)),
            ..Default::default()
        })
        .with(Obstacle);

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 250.0, 0.0)),
            sprite: Sprite::new(Vec2::new(500.0, 50.0)),
            ..Default::default()
        })
        .with(Obstacle);
    commands.insert_resource(SpawnTimer(Timer::from_seconds(2.0, true)));
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "Score".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(ScoreText);
    commands.insert_resource(ScoreBoard { score: 0 });
}

struct Player;
struct Obstacle;

struct FlyingObstacle;

struct Movement {
    speed: f32,
    gravity: f32,
}

struct ScoreText;

#[derive(Default)]
pub struct ScoreBoard {
    score: usize,
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

        let touch_margin = f32::EPSILON;

        if (transform.translation.y - 200.0).abs() < touch_margin
            || (transform.translation.y - -200.0).abs() < touch_margin
        {
            state.set_next(AppState::GameOver).unwrap();
        }
    }
}

struct SpawnTimer(Timer);

fn spawn_flyingobstacle(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    let mut rng = rand::thread_rng();

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(
                250.0,
                rng.gen_range(-200.0..200.0),
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(FlyingObstacle);
}

fn move_flyingobstcle(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<FlyingObstacle>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= time.delta_seconds() * 200.0;
        if transform.translation.x < -250.0 {
            commands.despawn(entity);
        }
    }
}

fn collide_flyingobstacle(
    commands: &mut Commands,
    mut scoreboard: ResMut<ScoreBoard>,
    mut flyingobstacle_query: Query<(Entity, &Transform, &Sprite), With<FlyingObstacle>>,
    query: Query<(&Transform, &Sprite), With<Player>>,
) {
    for (flyingobstacle_entity, flyingobstacle_transform, flyingobstacle_sprite) in
        flyingobstacle_query.iter_mut()
    {
        for (player_transform, player_sprite) in query.iter() {
            let collision = collide(
                flyingobstacle_transform.translation,
                flyingobstacle_sprite.size,
                player_transform.translation,
                player_sprite.size,
            );
            if collision.is_some() {
                scoreboard.score += 1;
                commands.despawn(flyingobstacle_entity);
            }
        }
    }
}

fn text_score_update(scoreboard: Res<ScoreBoard>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in query.iter_mut() {
        text.value = scoreboard.score.to_string();
    }
}

fn cleanup_ingame(
    commands: &mut Commands,
    mut scoreboard: ResMut<ScoreBoard>,
    query: Query<Entity, GameEntity>,
) {
    for entity in query.iter() {
        commands.despawn_recursive(entity);
    }
    scoreboard.score = 0;
}
