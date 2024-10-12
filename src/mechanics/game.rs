use bevy::prelude::*;

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

//Playercomposed of Entity Sprite -> Composition over Inheritance
#[derive(Bundle)]
pub struct PlayerSprite {
    pub spritebundle: SpriteBundle,
    pub playermarker: PlayerMarker
}

const VERTICALSPEED: f32 = 10.0;
const HORIZONTALSPEED: f32 = 10.0;
/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame
fn playermove(mut playertransform: Query<(&PlayerMarker, &mut Transform)>, keyboardinput: Res<ButtonInput<KeyCode>>, verticalspeed: f32, horizontalspeed: f32){
    

    for (_playermarker, mut transform) in playertransform.iter_mut() {
        if keyboardinput.pressed(KeyCode::KeyD){
            transform.translation.x += horizontalspeed;
        }
        if keyboardinput.pressed(KeyCode::KeyA){
            transform.translation.x -= horizontalspeed;
        }
        if keyboardinput.pressed(KeyCode::KeyW){
            transform.translation.y += verticalspeed;
        }
        if keyboardinput.pressed(KeyCode::KeyS){
            transform.translation.y -= verticalspeed;
        }
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
                            translation: Vec3::new(0.0,0.0,1.),
                            scale: Vec3::splat(1.0),
                            ..default()
                        },
                        ..default()
                    },
                   playermarker: PlayerMarker {},
                }
            );

    terraingen::generatechunk(commands, asset_server, playertransform);   
}


pub fn game_core_plugin (playertransform: Query<(&PlayerMarker, &mut Transform)>, keyboardinput: Res<ButtonInput<KeyCode>>) {

  
    //In gameloop
    playermove(playertransform, keyboardinput,VERTICALSPEED, HORIZONTALSPEED);
}