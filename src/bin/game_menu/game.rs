#![allow(unused)]
mod main_menu;
mod settings;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(main_menu::main_menu_plugin)
            .add_plugins(settings::settings_plugin)
            .add_systems(Startup, setup_system);
    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default, Copy)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
}
fn setup_system(mut cmds: Commands, asset: Res<AssetServer>) {
    let _ = asset;
    cmds.spawn(Camera2d);
}

