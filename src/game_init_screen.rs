use bevy::{color::palettes::css::CRIMSON, prelude::*};

use crate::{enums::GameState, user_interface::{colors, horizontal_layout, vertical_layout, MenuStyles}};
use crate::user_interface::create_text;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameInitState {
    #[default]
    Disabled,
    Setup,
    WaitingForArtist,
    LoadingSongs,
    SelectedSong,
    SelectedDifficulty,
    SelectedMode,
    Start,
    Back
}

enum Difficulty {
    VeryEasy,
    Easy,
    Norma,
    Hard,
    SuperHard
}

#[derive(Component)]
struct SelectedSong(i32);

#[derive(Component)]
struct Artist(String);

#[derive(Component)]
struct Song(String);

#[derive(Component)]
struct SelectedOption(String);

pub fn game_init_screen_plugin(app: &mut App) {
    bevy::log::info!("Starting game intro screen");

    app.init_state::<GameInitState>()
        .add_systems(OnEnter(GameState::Play), menu_setup)
        .add_systems(OnEnter(GameInitState::Setup), this_menu_setup)
        .add_systems(Update, (artists_button_system).run_if(in_state(GameInitState::WaitingForArtist)));
}

fn menu_setup(mut menu_state: ResMut<NextState<GameInitState>>) {
    bevy::log::info!("Setup of the Gameplay Menu");
    menu_state.set(GameInitState::Setup);
}

fn this_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, mut menu_state: ResMut<NextState<GameInitState>>) {
    bevy::log::info!("Initial Gameplay Menu");

    // TODO: Maybe the artists should be an Entity?
    let artists = vec!["Janis Joplin", "Bob Dylan", "Ray Charles", "Joan Baez"];
    let style = MenuStyles::new();

    commands.spawn(horizontal_layout()).with_children(|p|{
        p.spawn(vertical_layout(CRIMSON.into())).with_children(|p|{
            for artist in artists {
            p.spawn((style.button_style.clone(), Button, Artist(artist.into()), children![
                (Text::new(artist), style.text_font.clone(), TextColor(colors::TEXT_COLOR))
                ]));
            }
        });
    });
    menu_state.set(GameInitState::WaitingForArtist);
}


// This system changes the colors of the buttons based on mouse movement.
fn artists_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>, &Artist),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, selected, artist) in &mut interaction_query {
        *bg_color = match (*interaction, selected) {
            (Interaction::Pressed, _) => {
                let log_outptu = format!("Selected {}", artist.0);
                bevy::log::info!(log_outptu);
                colors::PRESSED.into()
            },
            (Interaction::None, Some(_)) => colors::PRESSED.into(),
            (Interaction::Hovered, Some(_)) => colors::HOVER_PRESSED.into(),
            (Interaction::Hovered, None) => colors::HOVER.into(),
            (Interaction::None, None) => colors::NORMAL.into(),
        }
    }
}
