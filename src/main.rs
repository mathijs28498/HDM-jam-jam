use bevy::{
    ecs::{
        query::WorldQuery,
        system::{EntityCommands, Resource},
    },
    prelude::*,
    render::render_resource::Texture,
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct SpawnButton;

// The type of button to swap an element
// The boolean says which button it is
// False is the left button, true is the right button
#[derive(Component)]
enum SwapButtonPosition {
    HEAD(bool),
    BODY(bool),
    LEGS(bool),
}

#[derive(Component)]
enum PartPosition {
    HEAD,
    BODY,
    LEGS,
}

#[derive(Component, Clone, Debug)]
struct PartType {
    color: Color,
}

#[derive(Component)]
struct PartTypeList {
    part_type_list: Vec<PartType>,
}

#[derive(Component)]
struct AllyCreature;

fn main() {
    // Window size: 1280x720
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PartTypeList {
            part_type_list: vec![
                PartType { color: Color::RED },
                PartType { color: Color::BLUE },
                PartType {
                    color: Color::GREEN,
                },
                PartType {
                    color: Color::YELLOW,
                },
            ],
        })
        .add_startup_system(setup)
        .add_system(swap_button_system)
        .add_system(spawn_button_system)
        .add_system(move_ally_creature_system)
        .run();
}

fn move_ally_creature_system(
    mut creature_query: Query<&mut Transform, With<AllyCreature>>
) {
    for mut transform in creature_query.iter_mut() {
        transform.translation.x -= 1.;
    }
}

fn spawn_button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<SpawnButton>),
    >,
    mut part_query: Query<(&PartType, &PartPosition)>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let mut head_part_type = None;
                let mut body_part_type = None;
                let mut legs_part_type = None;
                for (mut part_type, part_position) in part_query.iter_mut() {
                    match part_position {
                        PartPosition::HEAD => {
                            head_part_type = Some(part_type.clone());
                        }
                        PartPosition::BODY => {
                            body_part_type = Some(part_type.clone());
                        }
                        PartPosition::LEGS => {
                            legs_part_type = Some(part_type.clone());
                        }
                    }
                }

                // TODO: Spawn new monster based on types                    
                commands.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("images/white.png"),
                    transform: Transform {
                        translation: Vec3::new(250., -60., 2.),
                        scale: Vec3::new(40., 40., 1.),
                        ..default()
                    },
                    sprite: Sprite {
                        color: head_part_type.unwrap().color,
                        ..default()
                    },
                    ..default()
                })
                .insert(AllyCreature);    

                commands.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("images/white.png"),
                    transform: Transform {
                        translation: Vec3::new(250., -100., 2.),
                        scale: Vec3::new(40., 40., 1.),
                        ..default()
                    },
                    sprite: Sprite {
                        color: body_part_type.unwrap().color,
                        ..default()
                    },
                    ..default()
                })
                .insert(AllyCreature);   

                commands.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("images/white.png"),
                    transform: Transform {
                        translation: Vec3::new(250., -140., 2.),
                        scale: Vec3::new(40., 40., 1.),
                        ..default()
                    },
                    sprite: Sprite {
                        color: legs_part_type.unwrap().color,
                        ..default()
                    },
                    ..default()
                })
                .insert(AllyCreature);

            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn swap_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &SwapButtonPosition),
        (Changed<Interaction>, With<Button>),
    >,
    part_type_list: Res<PartTypeList>,
    asset_server: Res<AssetServer>,
    mut part_query: Query<(&mut PartType, &mut Sprite, &PartPosition)>,
) {
    for (interaction, mut color, swap_button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match swap_button_type {
                    SwapButtonPosition::HEAD(is_right) => {
                        for (mut part_type, mut sprite, part_position) in part_query.iter_mut() {
                            match part_position {
                                PartPosition::HEAD => {}
                                _ => continue,
                            }
                            let old_index = part_type_list
                                .part_type_list
                                .iter()
                                .position(|pt| pt.color == part_type.color)
                                .unwrap();
                            let addition = if *is_right { 1 } else { -1 };
                            let new_index = (old_index as i32 + addition)
                                .rem_euclid(part_type_list.part_type_list.len() as i32)
                                as usize;

                            *part_type = part_type_list.part_type_list[new_index].clone();
                            sprite.color = part_type.color;

                            break;
                        }
                    }
                    SwapButtonPosition::BODY(is_right) => {
                        for (mut part_type, mut sprite, part_position) in part_query.iter_mut() {
                            match part_position {
                                PartPosition::BODY => {}
                                _ => continue,
                            }
                            let old_index = part_type_list
                                .part_type_list
                                .iter()
                                .position(|pt| pt.color == part_type.color)
                                .unwrap();
                            let addition = if *is_right { 1 } else { -1 };
                            let new_index = (old_index as i32 + addition)
                                .rem_euclid(part_type_list.part_type_list.len() as i32)
                                as usize;

                            *part_type = part_type_list.part_type_list[new_index].clone();
                            sprite.color = part_type.color;

                            break;
                        }
                    }
                    SwapButtonPosition::LEGS(is_right) => {
                        for (mut part_type, mut sprite, part_position) in part_query.iter_mut() {
                            match part_position {
                                PartPosition::LEGS => {}
                                _ => continue,
                            }
                            let old_index = part_type_list
                                .part_type_list
                                .iter()
                                .position(|pt| pt.color == part_type.color)
                                .unwrap();
                            let addition = if *is_right { 1 } else { -1 };
                            let new_index = (old_index as i32 + addition)
                                .rem_euclid(part_type_list.part_type_list.len() as i32)
                                as usize;

                            *part_type = part_type_list.part_type_list[new_index].clone();
                            sprite.color = part_type.color;

                            break;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    part_type_list: Res<PartTypeList>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/light_blue.png"),
        transform: Transform {
            translation: Vec3::new(-160., 100., 1.),
            scale: Vec3::new(960., 520., 1.),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/green.png"),
        transform: Transform {
            translation: Vec3::new(-160., -260., 1.),
            scale: Vec3::new(960., 200., 1.),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/gray.png"),
        transform: Transform {
            translation: Vec3::new(480., 0., 1.),
            scale: Vec3::new(320., 720., 1.),
            ..default()
        },
        ..default()
    });

    spawn_swap_buttons(&mut commands, &asset_server);

    let head_part_type = part_type_list.part_type_list[0].clone();
    let body_part_type = part_type_list.part_type_list[1].clone();
    let legs_part_type = part_type_list.part_type_list[2].clone();

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.), Val::Px(40.)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(60.),
                    bottom: Val::Px(100.),
                    ..default()
                },
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
                "Spawn",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(SpawnButton);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/white.png"),
            transform: Transform {
                translation: Vec3::new(480., 120., 2.),
                scale: Vec3::new(120., 120., 1.),
                ..default()
            },
            sprite: Sprite {
                color: head_part_type.color.clone(),
                ..default()
            },
            ..default()
        })
        .insert(PartPosition::HEAD)
        .insert(head_part_type);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/white.png"),
            transform: Transform {
                translation: Vec3::new(480., 0., 2.),
                scale: Vec3::new(120., 120., 1.),
                ..default()
            },
            sprite: Sprite {
                color: body_part_type.color.clone(),
                ..default()
            },
            ..default()
        })
        .insert(PartPosition::BODY)
        .insert(body_part_type);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/white.png"),
            transform: Transform {
                translation: Vec3::new(480., -120., 2.),
                scale: Vec3::new(120., 120., 1.),
                ..default()
            },
            sprite: Sprite {
                color: legs_part_type.color.clone(),
                ..default()
            },
            ..default()
        })
        .insert(PartPosition::LEGS)
        .insert(legs_part_type);
}

fn spawn_swap_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    spawn_button(
        commands,
        asset_server,
        UiRect {
            right: Val::Px(240.),
            top: Val::Px(220.),
            ..default()
        },
        "<",
        SwapButtonPosition::HEAD(false),
    );

    spawn_button(
        commands,
        &asset_server,
        UiRect {
            right: Val::Px(40.),
            top: Val::Px(220.),
            ..default()
        },
        ">",
        SwapButtonPosition::HEAD(true),
    );

    spawn_button(
        commands,
        &asset_server,
        UiRect {
            right: Val::Px(240.),
            top: Val::Px(340.),
            ..default()
        },
        "<",
        SwapButtonPosition::BODY(false),
    );

    spawn_button(
        commands,
        &asset_server,
        UiRect {
            right: Val::Px(40.),
            top: Val::Px(340.),
            ..default()
        },
        ">",
        SwapButtonPosition::BODY(true),
    );

    spawn_button(
        commands,
        &asset_server,
        UiRect {
            right: Val::Px(240.),
            top: Val::Px(460.),
            ..default()
        },
        "<",
        SwapButtonPosition::LEGS(false),
    );

    spawn_button(
        commands,
        &asset_server,
        UiRect {
            right: Val::Px(40.),
            top: Val::Px(460.),
            ..default()
        },
        ">",
        SwapButtonPosition::LEGS(true),
    );
}

fn spawn_button<'a>(
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
