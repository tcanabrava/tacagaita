use crate::enums::GameState;
use crate::helpers::despawn_screen;
use bevy::prelude::*;

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub fn splash_screen_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    bevy::log::info!("Splash setup");
    commands.spawn((Text2d::new("Splash Text"), OnSplashScreen));
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        bevy::log::info!("Timer!");
        game_state.set(GameState::MainMenu)
    }
}
