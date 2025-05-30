use bevy::prelude::*;
use tacagaita::enums::GameState;

enum PrimaryState {
    MenuState,
    Game2DState,
    Game3DState,
    ExitState
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, initial_setup)
        .run();
}

fn initial_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

