use bevy::prelude::*;
use num::*;
pub mod terraingen;

#[derive(Component)]
pub enum Direction {
    Idle,
    Up,
    Down,
    Left,
    Right
}

//Player Marker Component
#[derive(Component)]
pub struct PlayerMarker {
}

#[derive(Component)]
pub struct Movement {
    velocity: Vec3
}

//Playercomposed of Entity Sprite -> Composition over Inheritance
#[derive(Bundle)]
pub struct PlayerSprite {
    pub spritebundle: SpriteBundle,
    pub playermarker: PlayerMarker,
    pub movement: Movement,

}

const VERTICALSPEED: f32 = 1.0;
const HORIZONTALSPEED: f32 = 1.0;
const MAX_VERTICALSPEED: f32 = 3.0;
const MAX_HORIZONTALSPEED: f32 = 3.0;
/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame
fn playermove(mut playertransform: Query<(&mut Movement, &PlayerMarker, &mut Transform)>, keyboardinput: Res<ButtonInput<KeyCode>>, verticalspeed: f32, horizontalspeed: f32){

    for (mut movement, _playermarker, mut transform) in playertransform.iter_mut() {
        // Horizontal movement
        if keyboardinput.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            movement.velocity.x += horizontalspeed;  // Move right
        }
        if keyboardinput.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            movement.velocity.x -= horizontalspeed;  // Move left
        }

        // Vertical movement
        if keyboardinput.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            movement.velocity.y += verticalspeed;  // Move up
        }
        if keyboardinput.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            movement.velocity.y -= verticalspeed;  // Move down
        }

        // Apply the movement velocity to the transform
        movement.velocity.x -= movement.velocity.x * 0.3;
        movement.velocity.y -= movement.velocity.y * 0.3;
        movement.velocity.x = clamp(movement.velocity.x, -MAX_HORIZONTALSPEED, MAX_HORIZONTALSPEED);
        movement.velocity.y = clamp(movement.velocity.y, -MAX_VERTICALSPEED, MAX_VERTICALSPEED);
        transform.translation += movement.velocity;
    }
}
 

//setup game mechanic wise
pub fn game_setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    playertransform: Query<(&PlayerMarker, &mut Transform)>
) {
    
    commands.spawn(PlayerSprite {
                    spritebundle: SpriteBundle {
                        texture: asset_server.load("sprites/player/player.png"),
                        transform: Transform {
                            translation: Vec3::new(0.0,0.0,1.0),
                            scale: Vec3::splat(1.0),
                            ..default()
                        },
                        ..default()
                    },
                   playermarker: PlayerMarker {},
                   movement: Movement {
                    velocity: Vec3::new(0.0, 0.0, 0.0)
                   }
                }
            );

    terraingen::generatechunk(commands, asset_server, playertransform);   
}


pub fn game_core_plugin (playertransform: Query<(&mut Movement,&PlayerMarker, &mut Transform)>, keyboardinput: Res<ButtonInput<KeyCode>>) {

  
    //In gameloop
    playermove(playertransform, keyboardinput,VERTICALSPEED, HORIZONTALSPEED);
}