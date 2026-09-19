use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

pub fn world_plugin(app: &mut App) {
    app.add_systems(Startup, world_setup);
}

fn world_setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(),));
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
}
