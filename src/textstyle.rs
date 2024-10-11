use bevy::prelude::*;

// Define a struct that will hold fonts
#[derive(Resource)]
pub struct FontECS {
    pub fonthandle_1: Handle<Font>,
    //pub fonthandle_2: Handle<Font>,
}


// Store fonts in resources
pub fn load_fonts (mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontECS {
        // Load the fonts
        fonthandle_1: asset_server.load("fonts/departure-mono/DepartureMono-Regular.otf"),
        //fonthandle_2: asset_server.load("fonts/departure-mono/DepartureMono-Regular.woff"),
    });
}

//font plugin
pub fn font_plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(Startup, load_fonts);

}

    


