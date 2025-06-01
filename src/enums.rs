use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Play,
    Exit,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, strum::EnumIter)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}
