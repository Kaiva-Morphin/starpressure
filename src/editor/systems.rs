use std::f32::consts::PI;

use bevy::{prelude::*, ui::widget::UiImageSize};
use bevy_rapier2d::prelude::*;

use crate::ragdoll::systems::init_skeleton;

use super::components::*;

pub const MAIN_COLOR: Color = Color::rgb(24. / 255., 24. / 255., 24. / 255.);
pub const SECONDARY_COLOR: Color = Color::rgb(41. / 255., 41. / 255., 41. / 255.);
pub const HOVER_COLOR: Color = Color::rgb(82. / 255., 82. / 255., 82. / 255.);
pub const TEXT_COLOR: Color = Color::rgb(200. / 255., 200. / 255., 200. / 255.);

pub fn init_editor_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    /* vertically resizable
    commands
    .spawn((
        NodeBundle {
            style: Style{
                width: Val::Percent(100.),
                height: Val::Percent(20.),
                max_height: Val::Percent(70.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
    }, SelectionTab { size: 20. }))
    .with_children(|parent| {
        parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    position_type: PositionType::Absolute,
                    top: Val::Px(-10.),
                    right: Val::Percent(10.),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            Resizer { was_pressed: false, is_vertical: true, start_pos: 0. },
        ))
        ;
    });
    */
    commands
    .spawn((
        NodeBundle {
            style: Style{
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                max_width: Val::Percent(70.),
                min_width: Val::Percent(10.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                ..default()
            },
            background_color: MAIN_COLOR.into(),
            ..default()
    }, SelectionTab { size: 20. }))
    .with_children(|parent| {
        parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(10.),
                    right: Val::Px(-10.),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            Resizer { was_pressed: false, is_vertical: false, start_pos: 0. },
        ));
    })
    .with_children(|parent| {
        parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(4.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Percent(0.),
                    ..default()
                },
                background_color: SECONDARY_COLOR.into(),
                ..default()
            },
            FileButton { is_opened: false },
        )).with_children(|parent| {
            parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new("File", TextStyle {
                        font: asset_server.load("fonts/minecraft_font.ttf"),
                        font_size: 16.,
                        color: TEXT_COLOR 
                    })],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            });
        });
    })
    ;
}

pub fn manage_file_window(
    mut commands: Commands,
    mut file_event: EventReader<FileOpenWindowEvent>,
    asset_server: Res<AssetServer>,
    file_tab_q: Query<&FileTab>,
) {
    for event in file_event.read() {
        if event.to_open {
            let n_actions = 3.;
            let top_entity = commands.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(n_actions * 100.),
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(-100. * n_actions),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: HOVER_COLOR.into(),
                z_index: ZIndex::Global(100),
                ..default()
            }).id();
            // new file button
            let child = spawn_file_tab_button(
                &mut commands,
                &asset_server,
                n_actions,
                0.,
                "New File".to_owned());
            commands.entity(child).insert(NewFileButton {});
            commands.entity(top_entity).add_child(child);
            // open file button
            let child = spawn_file_tab_button(
                &mut commands,
                &asset_server,
                n_actions,
                1.,
                "Open File".to_owned());
            commands.entity(child).insert(OpenFileButton {});
            commands.entity(top_entity).add_child(child);
            // save file button
            let child = spawn_file_tab_button(
                &mut commands,
                &asset_server,
                n_actions,
                2.,
                "Save File".to_owned());
            commands.entity(child).insert(SaveFileButton {});
            commands.entity(top_entity).add_child(child);

            commands.entity(top_entity).insert(FileTab { top_entity });
            commands.entity(event.entity).add_child(top_entity);
        } else {
            if let Ok(tab) = file_tab_q.get_single() {
                commands.entity(tab.top_entity).despawn_recursive();
            }
        }
    }
}

fn spawn_file_tab_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    n_actions: f32,
    order_num: f32,
    text: String,
) -> Entity {
    commands.spawn(
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100. / n_actions),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute, // todo: mb relative?
                top: Val::Percent(100. / n_actions * order_num),
                ..default()
            },
            background_color: SECONDARY_COLOR.into(),
            ..default()
        }
    ).with_children(|parent| {
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(text, TextStyle {
                        font: asset_server.load("fonts/minecraft_font.ttf"),
                        font_size: 16.,
                        color: TEXT_COLOR 
                    })],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            }
        );
    })
    .id()
}

pub fn new_file(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut new_event: EventReader<NewFileEvent>,
    selection_tab_q: Query<Entity, With<SelectionTab>>,
    file_tab_q: Query<&FileTab>,
) {
    for _ in new_event.read() {
        init_skeleton(&mut commands, RigidBody::Fixed);
        // todo: if there is no current file
        commands.entity(file_tab_q.single().top_entity).despawn_recursive();
        let selection_tab_entity = selection_tab_q.single();
        let top_node = spawn_node(&mut commands, &asset_server);
        commands.entity(selection_tab_entity).add_child(top_node);
    }
}

fn spawn_node(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let button_entity = commands.spawn(
        ButtonBundle {
            style: Style {
                width: Val::Percent(90.),
                height: Val::Percent(4.),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Percent(5.),
                right: Val::Percent(5.),
                top: Val::Percent(5.),
                ..default()
            },
            background_color: SECONDARY_COLOR.into(),
            ..default()
        }
    ).id();
    let text_entity = commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection::new("New Ragdoll", TextStyle {
                    font: asset_server.load("fonts/minecraft_font.ttf"),
                    font_size: 16.,
                    color: TEXT_COLOR
                })],
                justify: JustifyText::Left,
                ..default()
            },
            style: Style {
                height: Val::Percent(95.),
                width: Val::Percent(100.),
                left: Val::Percent(0.),
                top: Val::Percent(0.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            background_color: MAIN_COLOR.into(),
            ..default()
        },
    )).id();
    let image_entity = commands.spawn((
        ImageBundle {
            style: Style {
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_z(PI / 2.)),
            image: asset_server.load("arrow.png").into(),
            ..default()
        }
    )).id();
    commands.entity(button_entity)
    .add_child(text_entity)
    .add_child(image_entity)
    .insert(FileTabNode {text_entity: text_entity, image_entity: image_entity});
    button_entity
}