use bevy::{DefaultPlugins, app::App, state::app::AppExtStates};

mod app_state;
mod entities;
mod ui;

use app_state::GameState;

use ui::main_menu::menu_plugin;

use crate::entities::world::world_setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(world_setup::world_plugin)
        .add_plugins(menu_plugin)
        .run();
    println!("Hello, world!");
}
