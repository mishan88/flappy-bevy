use bevy::prelude::*;
mod menu;
use menu::MenuPlugin;
mod game;
use game::GamePlugin;
mod gameover;
use gameover::GameOverPlugin;

const STAGE: &str = "app";
#[derive(Clone)]
enum AppState {
    Menu,
    InGame,
    GameOver,
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
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        .run();
}
