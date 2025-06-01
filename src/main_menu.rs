use bevy::ecs::spawn::{SpawnIter, SpawnWith};
use bevy::{color::palettes::css::CRIMSON, log, prelude::*};
use enums::{DisplayQuality, GameState};
use helpers::despawn_screen;
use strum::IntoEnumIterator;

use crate::user_interface::{
    MenuStyles, colors, create_button_2, create_text, horizontal_layout, main_bundle,
    vertical_layout,
};
use crate::{enums, helpers};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

// Tag Components for actions in the menu.
#[derive(Component)]
struct OnMainMenu;
#[derive(Component)]
struct OnSettings;
#[derive(Component)]
struct OnDisplay;
#[derive(Component)]
struct OnSound;

// Current selected menu option.
#[derive(Component)]
struct SelectedOption(String);

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

pub fn main_menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        // Initial Menu
        .add_systems(OnEnter(GameState::MainMenu), menu_setup)
        // Main Menu
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenu>)
        // Settings Menu
        .add_systems(OnEnter(MenuState::Settings), settings_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            helpers::despawn_screen::<OnSettings>,
        )
        // Graphics Menu
        .add_systems(OnEnter(MenuState::SettingsDisplay), display_setup)
        .add_systems(
            Update,
            setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),
        )
        .add_systems(
            OnExit(MenuState::SettingsDisplay),
            helpers::despawn_screen::<OnDisplay>,
        )
        // Audio Menu
        .add_systems(OnEnter(MenuState::SettingsSound), sound_setup)
        .add_systems(
            Update,
            setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
        )
        .add_systems(
            OnExit(MenuState::SettingsSound),
            helpers::despawn_screen::<OnSound>,
        )
        .add_systems(
            Update,
            (button_system, menu_action).run_if(in_state(GameState::MainMenu)),
        );
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    log::info!("Setting up menu");
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    bevy::log::info!("Setting up main menu");
    let styles = MenuStyles::new();

    let inner_layout = (
        vertical_layout(CRIMSON.into()),
        children![
            create_text("Tacagaita"),
            create_button_2(
                "Play",
                Some(asset_server.load("textures/icons/exitRight.png")),
                MenuButtonAction::Play,
                &styles
            ),
            create_button_2(
                "Settings",
                Some(asset_server.load("textures/icons/exitRight.png")),
                MenuButtonAction::Settings,
                &styles
            ),
            create_button_2(
                "Quit",
                Some(asset_server.load("textures/icons/exitRight.png")),
                MenuButtonAction::Quit,
                &styles
            )
        ],
    );

    commands.spawn((main_bundle(OnMainMenu), children![inner_layout]));
}

fn settings_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    _ = asset_server;
    bevy::log::info!("Setting up the Settings Menu");
    let styles = MenuStyles::new();
    let main_layout = (
        vertical_layout(CRIMSON.into()),
        children![
            create_text("Settings"),
            create_button_2("Video", None, MenuButtonAction::SettingsDisplay, &styles),
            create_button_2("Audio", None, MenuButtonAction::SettingsSound, &styles),
            create_button_2("Back", None, MenuButtonAction::BackToMainMenu, &styles),
        ],
    );

    commands.spawn((main_bundle(OnSettings), children![main_layout]));
}

fn display_setup(
    mut commands: Commands,
    display_quality: Res<DisplayQuality>,
    asset_server: Res<AssetServer>,
) {
    bevy::log::info!("Setting up the Video Menu");
    let styles = MenuStyles::new();
    let mut selected_entity: Option<Entity> = None;

    commands.spawn(main_bundle(OnDisplay)).with_children(|p| {
        p.spawn(vertical_layout(CRIMSON.into())).with_children(|p| {
            p.spawn(create_text("Video Settings"));
            for val in DisplayQuality::iter() {
                let entity = create_button_2(
                    format!("{val:?}").as_str(),
                    Some(asset_server.load("")),
                    val,
                    &styles,
                );
                let entity = p.spawn(entity);
                // We can't borrow commands again here and change it
                // directly, so we create a temporary `selected_entity`
                // and set it.
                // when we finish the setup of the menu, we check
                // if there's anything selected, and add the entity of
                // Selected Option to it.
                if *display_quality == val {
                    selected_entity = Some(entity.id());
                }
            }
            let entity = create_button_2(
                "Back",
                Some(asset_server.load("")),
                MenuButtonAction::BackToSettings,
                &styles,
            );
            p.spawn(entity);
        });
    });

    if let Some(entity) = selected_entity {
        commands
            .get_entity(entity)
            .unwrap()
            .insert(SelectedOption("".into()));
    }
}

fn sound_setup(mut commands: Commands, asset_server: Res<AssetServer>, volume: Res<Volume>) {
    bevy::log::info!("Setting up the Sound Menu");
    let styles = MenuStyles::new();

    let mut btn_style = MenuStyles::new();
    btn_style.button_style.width = Val::Px(30.0);
    btn_style.button_style.height = Val::Px(65.0);

    let volume = *volume;

    let volume_layout = (
        horizontal_layout(),
        Children::spawn((
            Spawn(Name::new("volume_labels")),
            SpawnWith(move |p: &mut ChildSpawner| {
                for idx in 1..=10 {
                    let mut entity = p.spawn(create_button_2(
                        idx.to_string().as_str(),
                        None,
                        Volume(idx),
                        &btn_style,
                    ));

                    if idx == volume.0 {
                        entity.insert(SelectedOption("".into()));
                    }
                }
            }),
        )),
    );

    let vertical_layout = (
        vertical_layout(CRIMSON.into()),
        children![
            create_text("Audio Settings"),
            create_text("Volume"),
            volume_layout,
            create_button_2(
                "Back",
                Some(asset_server.load("")),
                MenuButtonAction::BackToSettings,
                &styles,
            )
        ],
    );

    commands.spawn((main_bundle(OnSound), children![vertical_layout]));
}

// This system changes the colors of the buttons based on mouse movement.
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => colors::PRESSED.into(),
            (Interaction::Hovered, Some(_)) => colors::HOVER_PRESSED.into(),
            (Interaction::Hovered, None) => colors::HOVER.into(),
            (Interaction::None, None) => colors::NORMAL.into(),
        }
    }
}

// Here we configure the actions on the main menu.
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_event: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match menu_button_action {
            MenuButtonAction::Quit => {
                bevy::log::info!("Requesting to exit game");
                app_exit_event.write(AppExit::Success);
            }
            MenuButtonAction::Settings => {
                bevy::log::info!("Entering the Settings Menu");
                menu_state.set(MenuState::Settings);
            }
            MenuButtonAction::Play => {
                bevy::log::info!("Requesting to play the game.");
                menu_state.set(MenuState::Disabled);
                game_state.set(GameState::Play);
            }
            MenuButtonAction::SettingsDisplay => {
                bevy::log::info!("Entering the Display Settings Menu");
                menu_state.set(MenuState::SettingsDisplay);
            }
            MenuButtonAction::SettingsSound => {
                bevy::log::info!("Entering the Sound Setings Menu");
                menu_state.set(MenuState::SettingsSound);
            }
            MenuButtonAction::BackToMainMenu => {
                bevy::log::info!("Returning to the Main Menu Screen");
                menu_state.set(MenuState::Main);
            }
            MenuButtonAction::BackToSettings => {
                bevy::log::info!("Returning to the Settings Screen");
                menu_state.set(MenuState::Settings)
            }
        }
    }
}

fn setting_button<T: Resource + Component + PartialEq + Copy>(
    mut interaction_query: Query<
        (&Interaction, &T, Entity),
        (Changed<Interaction>, With<Button>, Without<SelectedOption>),
    >,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
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
        commands.entity(entity).insert(SelectedOption("".into()));
        *setting = *btn_setting;
    }
}
