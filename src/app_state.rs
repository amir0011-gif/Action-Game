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
    pub loading_timer: Timer,
}

#[derive(Component)]
pub struct loadingScreen;

impl Default for LoadAssets {
    fn default() -> Self {
        LoadAssets {
            loading_timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}

pub fn loading_assets(
    mut res: ResMut<LoadAssets>,
    time: Res<Time>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<Entity, With<loadingScreen>>,
) {
    if res.loading_timer.tick(time.delta()).is_finished() {
        next_state.set(GameState::Game);
        for e in query.iter() {
            commands.entity(e).despawn();
        }
        println!("GameState is change")
    }
}

pub fn loading_screen(mut commands: Commands) {
    commands.spawn((
        loadingScreen,
        Transform::from_xyz(0.0, 0.0, 2.0),
        BackgroundColor(Color::srgb(0.8, 0.1, 0.2)),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![Text::new("Loading".to_string())],
    ));
}
