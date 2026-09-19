use avian2d::prelude::*;
use bevy::prelude::*;

pub fn enemies_plugin(app: &mut App) {
    app.add_systems(Startup, enemies_spown)
        .add_systems(Update, enemy_patrol);
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct EnemyBaseStatus {
    pub health: u32,
    pub velocity: f32,
}

#[derive(Component)]
struct PatrolPath {
    points: Vec<Vec3>,
    pub current_index: usize,
}

fn enemies_spown(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let enemy_shap = meshes.add(Rectangle::new(40.0, 40.0));

    let patrol_points = vec![
        Vec3::new(100.0, 200.0, 1.0),
        Vec3::new(0.0, 50.0, 1.0),
        Vec3::new(300.0, 200.0, 1.0),
    ];

    commands.spawn((
        Enemy,
        RigidBody::Dynamic,
        GravityScale(0.0),
        PatrolPath {
            points: patrol_points,
            current_index: 0,
        },
        Mesh2d(enemy_shap),
        MeshMaterial2d(materials.add(Color::srgb(0.9, 0.2, 0.2))),
        Collider::rectangle(40.0, 40.0),
        Transform::from_xyz(300.0, 200.0, 1.0),
        EnemyBaseStatus {
            health: 100,
            velocity: 100.0,
        },
        LockedAxes::ROTATION_LOCKED,
    ));
}

fn enemy_patrol(
    mut enemy_query: Query<
        (
            &mut LinearVelocity,
            &EnemyBaseStatus,
            &mut PatrolPath,
            &Transform,
        ),
        With<Enemy>,
    >,
) {
    for (mut velocity, state, mut patrols, transform) in enemy_query.iter_mut() {
        let current_pos = transform.translation.truncate();

        let mut target_pos = patrols.points[patrols.current_index];

        let mut new_target = target_pos.truncate() - current_pos;

        if new_target.length() < 5.0 {
            patrols.current_index = (patrols.current_index + 1) % patrols.points.len();
            target_pos = patrols.points[patrols.current_index];
            new_target = target_pos.truncate() - current_pos;
        }

        if new_target.length_squared() > 0.0 {
            new_target = new_target.normalize();
            velocity.x = new_target.x * state.velocity;
            velocity.y = new_target.y * state.velocity;
        }
    }
}
