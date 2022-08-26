use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use crate::camera::components::CameraMoveBoxDirection;
use crate::creation::components::Element;
use crate::PIXEL;
use crate::creation::components::PartPosition;

/// Material of a `Sprite` with a texture and color
#[derive(Debug, Clone)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}
impl SpriteMaterial {
    pub fn sprite(&self, custom_size: Vec2, transform: Transform) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(custom_size),
                color: self.color,
                ..default()
            },
            texture: self.texture.clone(),
            transform,
            ..default()
        }
    }
    pub fn node(&self, style: Style) -> NodeBundle {
        NodeBundle {
            style,
            color: self.color.into(),
            image: self.texture.clone().into(),
            ..default()
        }
    }
    pub fn button(&self, style: Style) -> ButtonBundle {
        ButtonBundle {
            style,
            color: self.color.into(),
            image: self.texture.clone().into(),
            ..default()
        }
    }
}
impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            texture: DEFAULT_IMAGE_HANDLE.typed(),
        }
    }
}
/// Assets for the board. Must be used as a resource.
///
/// Use the loader for partial setup
#[derive(Debug, Clone)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub mud: SpriteMaterial,
    pub water: SpriteMaterial,
    pub fire: SpriteMaterial,
    pub electric: SpriteMaterial,
    pub button_text: Color,
    pub button: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub size: f32,
}
impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        GameAssets {
            mud: SpriteMaterial {
                color: Color::GREEN,
                texture: asset_server.load("images/sprite.png"),
            },
            water: SpriteMaterial {
                color: Color::MIDNIGHT_BLUE,
                texture: asset_server.load("images/sprite.png"),
            },
            fire: SpriteMaterial {
                color: Color::ORANGE_RED,
                texture: asset_server.load("images/sprite.png"),
            },
            electric: SpriteMaterial {
                color: Color::ALICE_BLUE,
                texture: asset_server.load("images/sprite.png"),
            },
            button_text: Color::rgb(0.9, 0.9, 0.9),
            hovered: Color::rgb(0.25, 0.25, 0.25),
            pressed: Color::rgb(0.35, 0.75, 0.35),
            button:Color::rgb(0.15, 0.15, 0.15),
            font : asset_server.load("fonts/FiraSans-Bold.ttf"),
            size: 720.,
        }
    }
}
impl GameAssets {
    pub fn create_text<S: Into<String>>(&self, label: S) -> TextBundle {
        TextBundle {
            style: Style {
                margin: UiRect {
                    right: Val::Px(10.0),
                    left: Val::Px(10.),
                    ..default()
                },
                flex_basis: Val::Px(0.),
                ..default()
            },
            text: Text::from_section(
                      label.into(),
                      TextStyle {
                          font: self.font.clone(),
                          font_size: self.size/27.,
                          color: self.button_text,
                      }).with_alignment( TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }),
            ..default()
        }
    }
    pub fn button(&self) -> ButtonBundle {
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(40.), Val::Px(40.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: self.button.into(),
            ..default()
        }
    }
    pub fn elem_sprite_material<T: Into<Element>>(&self, elem: T) -> &SpriteMaterial {
        match elem.into() {
            Element::Mud => &self.mud,
            Element::Water => &self.water,
            Element::Fire => &self.fire,
            Element::Electric => &self.electric,
        }
    }
    pub fn spawn_elem<T: Into<Element>>(&self, elem: T, part: &PartPosition) -> SpriteBundle {
        let sprite_material = self.elem_sprite_material(elem);
        let pos = match part {
            PartPosition::Head => 2.,
            PartPosition::Body => 1.,
            PartPosition::Legs => 0.,
        };
        SpriteBundle {
            texture: sprite_material.texture.clone(),
            transform: Transform {
                translation: Vec3::new(0., PIXEL * pos , 1.),
                ..default()
            },
            sprite: Sprite {
                color: sprite_material.color,
                ..default()
            },
            ..default()
        }
    }
}
pub fn button_system(
    button_colors: Res<GameAssets>,
    mut interaction_query: Query<(&Interaction, &mut UiColor),
    (Without<CameraMoveBoxDirection>,Changed<Interaction>)>,
    ) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        // if *interaction == Interaction::Clicked {
        //     (action.action)(asset.as_mut());
        // }
        *color = match *interaction {
            Interaction::Clicked => button_colors.pressed.into(),
            Interaction::Hovered => button_colors.hovered.into(),
            Interaction::None => button_colors.button.into(),
        }
    }
}
