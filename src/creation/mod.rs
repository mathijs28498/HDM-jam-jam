use bevy::prelude::*;

use crate::NORMAL_BUTTON;

use self::components::SwapButtonPosition;

pub(crate) mod components;
pub(crate) mod systems;

pub(crate) fn spawn_button<'a>(
    commands: &'a mut Commands,
    asset_server: &Res<AssetServer>,
    position: UiRect<Val>,
    text: &str,
    swap_button_type: SwapButtonPosition,
) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(40.), Val::Px(40.)),
                position_type: PositionType::Absolute,
                position,
                // Make text align in center
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(swap_button_type);
}
