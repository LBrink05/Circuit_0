use bevy::prelude::*;
use crate::menumanager::*;


// Define a struct that will hold fonts
#[derive(Resource)]
pub struct FontECS {
    pub fonthandle_1: Handle<Font>,
    //pub fonthandle_2: Handle<Font>,
}


// Store fonts in resources
pub fn load_fonts (mut commands: Commands, asset_server: Res<AssetServer>, mut game_state: ResMut<NextState<GameState>>) {
    commands.insert_resource(FontECS {
        // Load the fonts
        fonthandle_1: asset_server.load("fonts/departure-mono/DepartureMono-Regular.otf"),
        //fonthandle_2: asset_server.load("fonts/departure-mono/DepartureMono-Regular.woff"),
    });

    //Continue to Splash Screen
    game_state.set(GameState::Splash);
}


    


