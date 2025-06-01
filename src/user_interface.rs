use bevy::{
    ecs::{
        relationship::{RelatedSpawnerCommands, Relationship},
        world,
    },
    prelude::*,
};

pub mod colors {
    use bevy::color::Color;
    pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const HOVER: Color = Color::srgb(0.25, 0.25, 0.25);
    pub const HOVER_PRESSED: Color = Color::srgb(0.25, 0.65, 0.25);
    pub const PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
}

pub struct MenuStyles {
    pub button_style: Node,
    pub icon_style: Node,
    pub text_font: TextFont,
}

impl MenuStyles {
    pub fn new() -> Self {
        MenuStyles {
            button_style: Node {
                width: Val::Px(300.0),
                height: Val::Px(65.0),
                margin: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            icon_style: Node {
                width: Val::Px(30.0),
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                ..default()
            },
            text_font: TextFont {
                font_size: 33.0,
                ..default()
            },
        }
    }
}

pub fn create_text(text: &str) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 67.0,
            ..default()
        },
        TextColor(colors::TEXT_COLOR),
    )
}

pub fn create_button_2<T: Bundle>(
    text: &str,
    icon: Option<Handle<Image>>,
    handle_flag: T,
    style: &MenuStyles,
) -> impl Bundle {
    _ = icon;
    (
        style.button_style.clone(),
        Button,
        handle_flag,
        children![(
            Text::new(text),
            style.text_font.clone(),
            TextColor(colors::TEXT_COLOR)
        )],
    )
}

pub fn main_bundle<T: Bundle>(val: T) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        val,
    )
}

pub fn horizontal_layout() -> Node {
    Node {
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn vertical_layout(color: BackgroundColor) -> impl Bundle {
    _ = color;
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    }
}
