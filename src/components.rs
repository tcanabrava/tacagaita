use bevy::prelude::*;

#[derive(Component)]
struct Player {
    name: String,
}
#[derive(Component)]
struct Xp(i32);

#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

enum Bend {
    None,
    Hole,
    Half,
    OverBend,
}

#[derive(Component)]
struct Note {
    hole: u8,
    bend: Bend,
    start: i32,
    duration: i32,
}

#[derive(Component)]
struct Harmonica {
    name: String,
    brand: String,
    image: String,
}
