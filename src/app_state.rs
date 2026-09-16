use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Menu,
    #[default]
    Loading,
    Game,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    #[default]
    Disabled,
}

#[derive(Resource)]
pub struct LoadAssets {
    pub loade_player: bool,
    pub loade_world: bool,
}

impl Default for LoadAssets {
    fn default() -> Self {
        LoadAssets {
            loade_player: false,
            loade_world: false,
        }
    }
}

pub fn loading_assets(res: Res<LoadAssets>, mut next_state: ResMut<NextState<GameState>>) {
    if res.loade_player && res.loade_world {
        next_state.set(GameState::Game);
    }
}

pub fn loading_screen(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![Text::new("Loading".to_string())],
    ));
}
