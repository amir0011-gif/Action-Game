use bevy::{color::palettes::tailwind::RED_950, prelude::*};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerState {
    pub health: u32,
    pub stamina: u32,
    pub velocity: f32,
    pub hideTime: Timer,
}

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, player_setup);
}

const X_EXTENT: f32 = 1000.;
const Y_EXTENT: f32 = 150.;
const THICKNESS: f32 = 5.0;

pub fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut res: ResMut<crate::app_state::LoadAssets>,
) {
    let shape = meshes.add(Rectangle::new(50.0, 100.0).to_ring(THICKNESS));

    commands.spawn((
        Player,
        PlayerState {
            health: 100,
            stamina: 60,
            hideTime: Timer::from_seconds(3.0, TimerMode::Once),
            velocity: 50.0,
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        // Sprite::from_color(Color::srgb(0.1, 0.6, 0.3), Vec2 { x: 50.0, y: 50.0 }),
        Mesh2d(shape),
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.8, 0.1))),
    ));
    res.loade_player = true;
}
