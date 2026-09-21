use avian2d::{
    collision::collider::Collider, dynamics::rigid_body::RigidBody, physics_transform::Position,
};
use bevy::prelude::*;

use crate::entities::player::player::Player;

pub fn world_plugin(app: &mut App) {
    app.add_systems(Startup, world_setup)
        .add_systems(Update, camera_follow_player);
}

#[derive(Component)]
struct MainCamera;

///base setup object world
fn world_setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(), MainCamera));

    commands.spawn((Sprite::from_color(
        Color::srgb(0.1, 0.2, 0.3),
        Vec2 {
            x: 1200.0,
            y: 800.0,
        },
    ),));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.6, 0.2, 0.6), Vec2 { x: 600.0, y: 10.0 }),
        Transform::from_xyz(0.0, -300.0, 1.0),
        RigidBody::Static,
        Collider::rectangle(600.0, 10.0),
    ));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.6, 0.2, 0.6), Vec2 { x: 200.0, y: 10.0 }),
        Transform::from_xyz(-200.0, 0.0, 1.0),
        RigidBody::Static,
        Collider::rectangle(200.0, 10.0),
    ));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.6, 0.2, 0.6), Vec2 { x: 10.0, y: 200.0 }),
        Transform::from_xyz(300.0, 50.0, 1.0),
        RigidBody::Static,
        Collider::rectangle(10.0, 200.0),
    ));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.6, 0.2, 0.6), Vec2 { x: 200.0, y: 10.0 }),
        Transform::from_xyz(200.0, 0.0, 1.0),
        RigidBody::Static,
        Collider::rectangle(200.0, 10.0),
    ));
}

///player camera
fn camera_follow_player(
    player_query: Query<&Position, With<Player>>,

    mut camera_query: Query<&mut Transform, With<MainCamera>>,

    time: Res<Time>,
) {
    let Some(player_pos) = player_query.iter().next() else {
        return;
    };

    let Some(mut camera_transform) = camera_query.iter_mut().next() else {
        return;
    };

    let target = Vec3::new(player_pos.x, player_pos.y, camera_transform.translation.z);

    let smooth_speed = 5.0;

    camera_transform.translation = camera_transform
        .translation
        .lerp(target, smooth_speed * time.delta_secs());
}
