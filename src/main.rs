// run command: cargo run --features bevy/dynamic_linking --profile dev
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;

//custom mods
#[path = "ui/menumanager.rs"]
pub mod menumanager;

#[path = "ui/textstyle.rs"]
pub mod textstyle;

#[path = "mechanics/game.rs"]
pub mod game;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest(),)
                //change window title
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Circuit_0".to_string(),
                        resolution: WindowResolution::default().with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
                
            ),
        )
        // One of the most useful things about bevy is that the engine code is very similar to it's
        // Insert as resource the initial value for the settings resources
        .insert_resource(Msaa::Off)
        .insert_resource(menumanager::DisplayQuality::Medium)
        .insert_resource(menumanager::Volume(7))
        //Startup
        .add_systems(Startup, (setup, textstyle::load_fonts))
        // Adds the plugins for each state
        // Declare the game state, whose starting value is determined by the `Default` trait //Entering splash first
        .init_state::<menumanager::GameState>()
        //menumanager
        .add_plugins((
            menumanager::splash::SplashPlugin,
            menumanager::menu::menu_plugin,
            menumanager::game::game_plugin,
        ))
        // gameplay code, so we can add GameCorePlugin here just like the Default Plugins
        .add_plugins(game::GameCorePlugin)
        .run();
}

//Camera settings
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
