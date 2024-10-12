use bevy::prelude::*;
 
#[derive(Component)]
pub enum Direction {
    Up,
    Down,
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn spritemove(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}



//setup game mechanic wise
pub fn game_setup (mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));

}


pub fn game_core_plugin (time: Res<Time>, sprite_position: Query<(&mut Direction, &mut Transform)>) {
    spritemove(time,sprite_position);
}