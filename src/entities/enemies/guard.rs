use avian2d::prelude::*;
use bevy::prelude::*;

use crate::entities::player::player::Player;

pub fn enemies_plugin(app: &mut App) {
    app.add_systems(Startup, enemies_spown)
        .add_systems(Update, (enemy_patrol, enemy_line_of_sight));
}

#[derive(Component)]
struct Enemy {
    facing_direction: Dir2,

    view_angle_half: f32,
}

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

    commands
        .spawn((
            Enemy {
                facing_direction: Dir2::X,
                view_angle_half: 30.0,
            },
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
                velocity: 50.0,
            },
            RayCaster::new(Vec2::ZERO, Dir2::X).with_max_distance(400.0),
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_child((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            Transform::from_xyz(20.0, 0.0, 1.0),
        ));
}

fn enemy_patrol(
    mut enemy_query: Query<
        (
            &mut LinearVelocity,
            &EnemyBaseStatus,
            &mut PatrolPath,
            &mut Transform,
            &mut Enemy,
        ),
        With<Enemy>,
    >,
    time: Res<Time>,
) {
    
    for (mut velocity, state, mut patrols, mut transform, mut enemy) in enemy_query.iter_mut() {
        
        //enemy 2D postiond
        let current_pos = transform.translation.truncate();

        let mut target_pos = patrols.points[patrols.current_index];
        
        //
        let mut new_target = target_pos.0 - current_pos;

        if new_target.length() < 5.0 {
            patrols.current_index = (patrols.current_index + 1) % patrols.points.len();
            target_pos = patrols.points[patrols.current_index];
            new_target = target_pos.0 - current_pos;
        }

        if let Ok(target_dir) = Dir2::new(new_target) {
            // ۱. محاسبه زاویه‌ای که در نهایت باید به آن نگاه کنیم (زاویه هدف)
            let target_rotation = Quat::from_rotation_z(target_dir.to_angle());

            // ۲. چرخش نرم گرافیک با استفاده از slerp
            // عدد 5.0 سرعت چرخش است. عدد کوچکتر = چرخش کندتر و رباتیک‌تر
            let rotation_speed = 5.0;
            transform.rotation = transform
                .rotation
                .slerp(target_rotation, rotation_speed * time.delta_secs());

            // ۳. آپدیت مخروط دید بر اساس زاویه نرم‌شده فعلی
            // متد local_x جهتی که کاراکتر در این فریم به آن نگاه می‌کند را برمی‌گرداند
            let current_forward = transform.local_x().truncate();
            if let Ok(new_facing_dir) = Dir2::new(current_forward) {
                enemy.facing_direction = new_facing_dir;
            }

            // ۴. حرکت به سمت جلوی خودش (نه مستقیماً به سمت نقطه!)
            // این ترفند باعث می‌شود کاراکتر مثل ماشین/انسان هنگام دور زدن، یک قوس حرکتی زیبا داشته باشد
            velocity.x = enemy.facing_direction.x * state.velocity;
            velocity.y = enemy.facing_direction.y * state.velocity;
        }
    }
}

fn enemy_line_of_sight(
    player_query: Query<(Entity, &Position), With<Player>>,
    // 🐛 تغییر اول: Entity و Transform دشمن را اضافه کردیم
    mut enemy_query: Query<
        (
            Entity,
            &Position,
            &Transform,
            &Enemy,
            &mut RayCaster,
            &RayHits,
        ),
        With<Enemy>,
    >,
    mut gizmos: Gizmos,
) {
    let Some((player_entity, player_pos)) = player_query.iter().next() else {
        return;
    };

    // متغیرهای دشمن را می‌گیریم
    for (enemy_entity, enemy_pos, transform, enemy, mut raycaster, hits) in &mut enemy_query {
        let vector_to_player = player_pos.0 - enemy_pos.0;
        let view_distance = 400.0;
        let view_angle_half = enemy.view_angle_half.to_radians();

        let current_angle = enemy.facing_direction.to_angle();
        let left_edge = Vec2::from_angle(current_angle + view_angle_half) * view_distance;
        let right_edge = Vec2::from_angle(current_angle - view_angle_half) * view_distance;

        gizmos.line_2d(
            enemy_pos.0,
            enemy_pos.0 + left_edge,
            Color::srgb(0.3, 0.3, 0.3),
        );
        gizmos.line_2d(
            enemy_pos.0,
            enemy_pos.0 + right_edge,
            Color::srgb(0.3, 0.3, 0.3),
        );

        if vector_to_player.length() > view_distance {
            continue;
        }

        let Ok(global_dir_to_player) = Dir2::new(vector_to_player) else {
            continue;
        };

        let view_threshold = view_angle_half.cos();
        let dot_product = enemy.facing_direction.dot(*global_dir_to_player);

        if dot_product > view_threshold {
            // 🐛 راه حل باگ اول: به پرتو می‌گوییم برخوردگر (Collider) خودِ این دشمن را نادیده بگیرد
            raycaster.query_filter = SpatialQueryFilter::from_excluded_entities([enemy_entity]);

            // 🐛 راه حل باگ دوم: تبدیل جهت بازیکن از فضای جهانی به فضای محلی
            // چون پرتو به بدن متصل است، باید چرخش بدن را از مسیر بازیکن کم کنیم
            let local_vector = transform.rotation.inverse() * vector_to_player.extend(0.0);

            if let Ok(local_dir) = Dir2::new(local_vector.truncate()) {
                raycaster.direction = local_dir; // پرتوی محلی اصلاح شده
            }
            raycaster.max_distance = vector_to_player.length();

            if let Some(first_hit) = hits.iter_sorted().next() {
                if first_hit.entity == player_entity {
                    gizmos.line_2d(enemy_pos.0, player_pos.0, Color::srgb(0.0, 1.0, 0.0));
                } else {
                    // در اینجا برای رسم خط قرمز، از همان جهت جهانی استفاده می‌کنیم که خط دیباگ کج رسم نشود
                    let hit_point = enemy_pos.0 + (*global_dir_to_player * first_hit.distance);
                    gizmos.line_2d(enemy_pos.0, hit_point, Color::srgb(1.0, 0.0, 0.0));
                    gizmos.circle_2d(hit_point, 5.0, Color::srgb(1.0, 0.0, 0.0));
                }
            }
        }
    }
}
