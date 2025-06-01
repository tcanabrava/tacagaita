use bevy::prelude::*;
use tacagaita::enums::{GameState, DisplayQuality};

use tacagaita::main_menu::Volume;
use tacagaita::splash_screen_plugin::splash_screen_plugin;
use tacagaita::main_menu::main_menu_plugin;
use tacagaita::game_init_screen::game_init_screen_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, initial_setup)
        .add_plugins((
            splash_screen_plugin,
            main_menu_plugin,
            game_init_screen_plugin,
        ))
        .run();
}

fn initial_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
