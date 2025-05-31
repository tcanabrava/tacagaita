use bevy::prelude::*;
use crate::enums::GameState;

/* Remove every element within the screen */
pub fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands) {
    bevy::log::info!("Despawning screen");
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}


/* Return to the main menu from within the game */
pub fn cancel_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::MainMenu)
    }
}

pub fn exit_game_mode<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    commands: Commands) {
    despawn_screen::<T>(to_despawn, commands)
}
