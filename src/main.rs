// run command: cargo run --features bevy/dynamic_linking --profile dev
use bevy::DefaultPlugins;
use bevy::prelude::*;

//custom libs
pub mod menumanager;
pub mod textstyle;





fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .insert_resource(menumanager::DisplayQuality::Medium)
        .insert_resource(menumanager::Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<menumanager::GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((menumanager::splash::splash_plugin, menumanager::menu::menu_plugin, menumanager::game::game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

