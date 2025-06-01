use std::{fmt::Display, path::{Path, PathBuf}};

use bevy::{color::palettes::css::CRIMSON, ecs::spawn::SpawnWith, prelude::*};
use serde::Deserialize;
use strum::Display;

use crate::{
    enums::GameState,
    user_interface::{colors, create_button, create_button_with_flag, horizontal_layout, vertical_layout, MenuStyles},
};

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
    Back,
}

enum Difficulty {
    VeryEasy,
    Easy,
    Norma,
    Hard,
    SuperHard,
}

#[derive(Resource, Component, PartialEq, Clone)]
struct CurrentArtist(
    String
);

#[derive(Component)]
struct Value(String);

#[derive(Resource)]
struct CurrentSong(String);

#[derive(Component)]
struct Song(String);

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
struct ArtistChoice;

#[derive(Component)]
struct SongChoice;

#[derive(Deserialize, Debug)]
struct ArtistInfo {
    name: String,

    #[serde(skip)]
    path: PathBuf
}

pub fn game_init_screen_plugin(app: &mut App) {
    bevy::log::info!("Starting game intro screen");

    app.init_state::<GameInitState>()
        .add_systems(OnEnter(GameState::Play), menu_setup)
        .add_systems(OnEnter(GameInitState::Setup), this_menu_setup)
        .add_systems(
            Update,
            (artists_button_system, button_system::<CurrentArtist, ArtistChoice>).run_if(in_state(GameInitState::WaitingForArtist)),
        ).insert_resource(CurrentArtist("None".into()));
}

fn menu_setup(mut menu_state: ResMut<NextState<GameInitState>>) {
    bevy::log::info!("Setup of the Gameplay Menu");
    menu_state.set(GameInitState::Setup);
}

fn this_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_state: ResMut<NextState<GameInitState>>,
) {
    bevy::log::info!("Initial Gameplay Menu");
    let mut artist_infos = Vec::new();

    for folder in std::fs::read_dir("./assets/songs").unwrap() {
        let folder: String = folder.unwrap().file_name().into_string().unwrap();
        let artist_json = format!("./assets/songs/{folder}/info.json");
        let artist_folder = format!("./assets/songs/{folder}/");

        let file_contents = std::fs::read_to_string(&artist_json).unwrap();
        let mut artist_info: ArtistInfo = serde_json::from_str(&file_contents).unwrap();

        artist_info.path = artist_folder.into();
        bevy::log::info!("AAA {:?}", &artist_info);

        artist_infos.push(artist_info);
    }

    let selected_artist = artist_infos[0].name.clone();
    // TODO: Maybe the artists should be an Entity?
    let style = MenuStyles::new();

    let v_layout = (
        vertical_layout(CRIMSON.into()),
        Children::spawn(SpawnWith(move |p: &mut ChildSpawner| {
            for artist in &artist_infos {
                let entity = create_button(&artist.name, None, &style);
                let mut entity = p.spawn(entity);
                entity.insert(CurrentArtist(artist.name.clone().into()));
                entity.insert(ArtistChoice);
                if artist.name == *selected_artist {
                    entity.insert(SelectedOption);
                }
            }
        })),
    );

    commands.spawn((horizontal_layout(), children![v_layout]));

    menu_state.set(GameInitState::WaitingForArtist);
}

// This system changes the colors of the buttons based on mouse movement.
fn artists_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SelectedOption>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, selected) in &mut interaction_query {
        *bg_color = match (*interaction, selected) {
            (Interaction::Pressed, _) => colors::PRESSED.into(),
            (Interaction::None, Some(_)) => colors::PRESSED.into(),
            (Interaction::Hovered, Some(_)) => colors::HOVER_PRESSED.into(),
            (Interaction::Hovered, None) => colors::HOVER.into(),
            (Interaction::None, None) => colors::NORMAL.into(),
        }
    }
}


fn button_system<SettingType: Resource + Component + PartialEq + Clone, MarkerType: Component>(
    mut interaction_query: Query<
        (&Interaction, &SettingType, Entity),
        (Changed<Interaction>, With<Button>, With<MarkerType>, Without<SelectedOption>),
    >,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<SettingType>,
) {
    let (previous_btn, mut previous_color) = selected_query.into_inner();
    for (interaction, btn_setting, entity) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if *setting == *btn_setting {
            continue;
        }
        *previous_color = colors::NORMAL.into();
        commands.entity(previous_btn).remove::<SelectedOption>();
        commands.entity(entity).insert(SelectedOption);
        *setting = btn_setting.clone();
    }
}
