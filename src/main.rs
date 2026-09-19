use avian2d::PhysicsPlugins;
use bevy::prelude::*;

mod app_state;
mod entities;
mod ui;

use app_state::*;

// use ui::main_menu::menu_plugin;

use crate::entities::{
    enemies::guard::enemies_plugin, player::player::player_plugin, world::world_setup,
};

fn main() {
    App::new()
        .init_resource::<LoadAssets>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // mode: WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Index(1)),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(player_plugin)
        .add_plugins(world_setup::world_plugin)
        .add_plugins(enemies_plugin)
        .add_systems(OnEnter(GameState::Loading), loading_screen)
        .add_systems(
            Update,
            (loading_assets).run_if(in_state(GameState::Loading)),
        )
        // .add_plugins(menu_plugin)
        .run();
    println!("Hello, world!");
}
