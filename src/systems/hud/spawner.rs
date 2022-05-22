use bevy::{
    math::Rect,
    prelude::{Color, Commands, Res, TextBundle, UiCameraBundle},
    text::{HorizontalAlign, Text, TextAlignment, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};

use crate::systems::resources::fonts::MainFont;

use super::HudText;

pub fn generate_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn spawn_hud_text(mut commands: Commands, main_font: Res<MainFont>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "",
                TextStyle {
                    font: main_font.handle.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(HudText);
}
