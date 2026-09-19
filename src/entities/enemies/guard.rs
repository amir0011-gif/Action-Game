use avian2d::prelude::*;
use bevy::prelude::*;

use crate::entities::player::player::Player;

pub fn enemies_plugin(app: &mut App) {
    app.add_systems(Startup, enemies_spown)
        .add_systems(Update, (enemy_patrol, enemy_line_of_sight));
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
    points: Vec<Position>,
    pub current_index: usize,
}

fn enemies_spown(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let enemy_shap = meshes.add(Rectangle::new(40.0, 40.0));

    let patrol_points = vec![
        Position::from_xy(100.0, 200.0),
        Position::from_xy(0.0, 50.0),
        Position::from_xy(300.0, 200.0),
    ];

    commands.spawn((
        Enemy,
        RigidBody::Dynamic,
        GravityScale(0.0),
        PatrolPath {
            points: patrol_points,
            current_index: 0,
        },
        Position::from_xy(300.0, 200.0),
        Mesh2d(enemy_shap),
        MeshMaterial2d(materials.add(Color::srgb(0.9, 0.2, 0.2))),
        Collider::rectangle(40.0, 40.0),
        Transform::from_xyz(300.0, 200.0, 1.0),
        EnemyBaseStatus {
            health: 100,
            velocity: 100.0,
        },
        RayCaster::new(Vec2::ZERO, Dir2::X).with_max_distance(400.0),
        LockedAxes::ROTATION_LOCKED,
    ));
}

fn enemy_patrol(
    mut enemy_query: Query<
        (
            &mut LinearVelocity,
            &EnemyBaseStatus,
            &mut PatrolPath,
            &Position,
        ),
        With<Enemy>,
    >,
) {
    for (mut velocity, state, mut patrols, transform) in enemy_query.iter_mut() {
        let current_pos = transform.0;

        let mut target_pos = patrols.points[patrols.current_index];

        let mut new_target = target_pos.0 - current_pos;

        if new_target.length() < 5.0 {
            patrols.current_index = (patrols.current_index + 1) % patrols.points.len();
            target_pos = patrols.points[patrols.current_index];
            new_target = target_pos.0 - current_pos;
        }

        if new_target.length_squared() > 0.0 {
            new_target = new_target.normalize();
            velocity.x = new_target.x * state.velocity;
            velocity.y = new_target.y * state.velocity;
        }
    }
}

fn enemy_line_of_sight(
    player_query: Query<(Entity, &Position), With<Player>>,

    mut enemy_query: Query<(&Position, &mut RayCaster, &RayHits), With<Enemy>>,
) {
    let Some((player_entity, player_pos)) = player_query.iter().next() else {
        return;
    };

    for (enemy_pos, mut raycaster, hits) in &mut enemy_query {
        let vector_to_player = player_pos.0 - enemy_pos.0;

        if vector_to_player.length() < 400.0 {
            if let Ok(direction) = Dir2::new(vector_to_player) {
                raycaster.direction = direction;

                raycaster.max_distance = vector_to_player.length();
            }

            if let Some(firs_hit) = hits.iter_sorted().next() {
                if firs_hit.entity == player_entity {
                    println!("I see the player! Attack!");
                } else {
                }
            }
        }
    }
}
