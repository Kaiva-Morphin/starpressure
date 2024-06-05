use bevy::{prelude::*, ui::widget::UiImageSize};
use bevy_egui::{egui::{self, ImageSize}, EguiContexts};
use bevy_file_dialog::{DialogFileLoaded, DialogFileSaved};

use crate::{components::Fonts, consts::TILE_SIZE, editor::{
    components::*,
    systems::{HOVER_COLOR, MAIN_COLOR, SECONDARY_COLOR, TEXT_COLOR}
}, game_core::components::Name2Handle, ragdoll::systems::load_atlas, ShipFileContents};

use super::{components::{SelectionSquare, TilesButton, TilesOrWalls, WallsButton}, TILES, WALLS};

pub fn init_file_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: ResMut<Fonts>
) {
    let font = fonts.data.clone();
    commands
    .spawn(
        NodeBundle {
            style: Style{
                width: Val::Percent(100.),
                height: Val::Percent(3.),
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
    })
    .with_children(|parent| {
        parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(10.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.),
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
                        font,
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
    fonts: ResMut<Fonts>
) {
    for event in file_event.read() {
        let font = fonts.data.clone();
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
                "New File".to_owned(),
                font.clone(),
            );
            commands.entity(child).insert(NewFileButton {});
            commands.entity(top_entity).add_child(child);
            // open ship button
            let child = spawn_file_tab_button(
                &mut commands,
                &asset_server,
                n_actions,
                1.,
                "Open File".to_owned(),
                font.clone(),
            );
            commands.entity(child).insert(OpenFileButton {});
            commands.entity(top_entity).add_child(child);
            // save ship button
            let child = spawn_file_tab_button(
                &mut commands,
                &asset_server,
                n_actions,
                2.,
                "Save File".to_owned(),
                font.clone(),
            );
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
    font: Handle<Font>
) -> Entity {
    commands.spawn(
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100. / n_actions),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
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
                        font,
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

pub fn init_selection_tab(
    mut commands: Commands,
    fonts: ResMut<Fonts>,
    mut asset_server: ResMut<AssetServer>,
    mut name2handle: ResMut<Name2Handle>,
    tilesorwalls: Res<TilesOrWalls>,
    mut prev: Local<Option<Entity>>,
) {
    if tilesorwalls.is_changed() {
    let font = fonts.data.clone();
    let parent = commands.spawn(
        NodeBundle {
            style: Style{
                width: Val::Percent(100.),
                height: Val::Percent(20.),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Row,
                row_gap: Val::Px(20.),
                column_gap: Val::Px(20.),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                ..default()
            },
            background_color: MAIN_COLOR.into(),
            ..default()
    }).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(25.),
                top: Val::Percent(0.),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: SECONDARY_COLOR.into(),
            ..default()
        }).with_children(|parent| {
            // walls button
            parent.spawn((ButtonBundle {
                style: Style {
                    width: Val::Percent(48.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: SECONDARY_COLOR.into(),
                ..default()
            }, WallsButton)).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("Walls", TextStyle {
                            font: font.clone(),
                            font_size: 16.,
                            color: TEXT_COLOR 
                        })],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                });
            });
            // tiles button
            parent.spawn((ButtonBundle {
                style: Style {
                    width: Val::Percent(50.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: SECONDARY_COLOR.into(),
                ..default()
            }, TilesButton)).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("Tiles", TextStyle {
                            font: font.clone(),
                            font_size: 16.,
                            color: TEXT_COLOR 
                        })],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                });
            });
        });
    }).id();
    
    if let Some(entity) = *prev {
        commands.entity(entity).despawn_recursive();
    }
    *prev = Some(parent);
    
    let iterator;
    if tilesorwalls.is_tiles {
        iterator = TILES;
    } else {
        iterator = WALLS;
    }
    for (name, path, rect) in iterator {
        let handle = load_atlas(&path.to_string(), &mut asset_server, &mut name2handle);
        let child = commands.spawn((
            ButtonBundle {
                style: Style{
                    width: Val::Px(100.),
                    height: Val::Px(100.),
                    top: Val::Px(30.),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: SECONDARY_COLOR.into(),
                ..default()
            },
            SelectionSquare {
                handle: handle.clone(),
                rect: rect.clone(),
            },
        )).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(name, TextStyle {
                            font: font.clone(),
                            font_size: 16.,
                            color: TEXT_COLOR 
                        })],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }
            );
            parent.spawn(ImageBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Percent(5.),
                        right: Val::Percent(5.),
                        top: Val::Percent(30.),
                        bottom: Val::Percent(5.)
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: handle.into(),
                ..default()
            });
        }).id();
        
        commands.entity(parent).add_child(child);
    }
    }
}

pub fn new_file(
    mut commands: Commands,
    mut new_event: EventReader<NewFileEvent>,
) {
    for _ in new_event.read() {
        // todo: if there is no current file
    }
}

pub fn save_open_file(
    mut open_event: EventReader<DialogFileLoaded<ShipFileContents>>,
    mut save_event: EventReader<DialogFileSaved<ShipFileContents>>,
) {
    for contents in open_event.read() {
        println!("{:?}", contents.path);
    }
    for contents in save_event.read() {
        println!("saved {:?}", contents.path);
        // this is for frontend
        // todo: close the editor or sth.
    }
}