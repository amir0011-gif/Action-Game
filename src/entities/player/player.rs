use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{LinearVelocity, LockedAxes, RigidBody},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerState {
    pub health: u32,
    pub stamina: u32,
    pub velocity: f32,
    pub hide_time: Timer,
}

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, player_setup)
        .add_systems(Update, player_movment);
}

// const X_EXTENT: f32 = 1000.;
// const Y_EXTENT: f32 = 150.;
const THICKNESS: f32 = 5.0;

pub fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //  mut res: ResMut<crate::app_state::LoadAssets>,
) {
    let shape = meshes.add(Rectangle::new(25.0, 25.0).to_ring(THICKNESS));

    commands.spawn((
        Player,
        PlayerState {
            health: 100,
            stamina: 60,
            hide_time: Timer::from_seconds(3.0, TimerMode::Once),
            velocity: 5000.0,
        },
        RigidBody::Dynamic,
        Collider::rectangle(25.0, 25.0),
        Transform::from_xyz(0.0, 0.0, 1.0),
        // Sprite::from_color(Color::srgb(0.1, 0.6, 0.3), Vec2 { x: 50.0, y: 50.0 }),
        Mesh2d(shape),
        LockedAxes::ROTATION_LOCKED,
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.8, 0.1))),
    ));
}

fn player_movment(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &PlayerState), With<Player>>,
) {
    for (mut velocity, state) in &mut query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            velocity.x = state.velocity * time.delta_secs();
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            velocity.x = -state.velocity * time.delta_secs();
        } else {
            velocity.x = 0.0; // توقف در صورت رها کردن کلید
        }

        if velocity.y.abs() < 0.1
            && !keyboard_input.pressed(KeyCode::ArrowDown)
            && keyboard_input.just_pressed(KeyCode::ArrowUp)
        {
            velocity.y = state.velocity * time.delta_secs();
        }
    }
}
