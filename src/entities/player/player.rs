use avian2d::prelude::*;
use bevy::{math::VectorSpace, prelude::*};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerState {
    pub health: u32,
    pub stamina: u32,
    pub velocity: f32,
    pub hide_time: Timer,
    pub dodge_coldown_time: Timer,
    pub dodge_timer: Timer,
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
        GravityScale(0.0),
        PlayerState {
            health: 100,
            stamina: 60,
            hide_time: Timer::from_seconds(3.0, TimerMode::Once),
            dodge_coldown_time: Timer::from_seconds(1.0, TimerMode::Once),
            dodge_timer: Timer::from_seconds(0.2, TimerMode::Once),

            velocity: 250.0,
        },
        Position::from_xy(0.0, 0.0),
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
    mut query: Query<(&mut LinearVelocity, &mut PlayerState), With<Player>>,
    time: Res<Time>,
) {
    for (mut velocity, mut state) in &mut query.iter_mut() {
        let mut direction = Vec2::ZERO;

        state.dodge_coldown_time.tick(time.delta());
        state.dodge_timer.tick(time.delta());

        // move right or left
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x = 1.0;
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x = -1.0;
        }

        // move up or down
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y = 1.0;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y = -1.0;
        }

        //normalize vector to set speed
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let mut fast_speed = 1.0;

        if keyboard_input.just_pressed(KeyCode::Space) {
            if state.dodge_coldown_time.is_finished() {
                state.dodge_coldown_time.reset();
                state.dodge_timer.reset();
            }
        }

        if !state.dodge_timer.is_finished() {
            fast_speed = 4.0;
        }

        velocity.x = direction.x * state.velocity * fast_speed;
        velocity.y = direction.y * state.velocity * fast_speed;

        //logic of jump if gravity is sets
        // if velocity.y.abs() < 0.1
        //     && !keyboard_input.pressed(KeyCode::ArrowDown)
        //     && keyboard_input.just_pressed(KeyCode::ArrowUp)
        // {
        //     velocity.y = state.velocity * time.delta_secs();
        // }
    }
}
