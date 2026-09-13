use bevy::prelude::*;

pub fn world_plugin(app: &mut App) {
    app.add_systems(Startup, world_setup);
}

fn world_setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(),));
    commands.spawn((Sprite::from_color(
        Color::srgb(0.1, 0.2, 0.3),
        Vec2 { x: 800.0, y: 800.0 },
    ),));
}
