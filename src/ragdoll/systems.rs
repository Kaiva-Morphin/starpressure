use std::{collections::{HashMap, HashSet}, fs::File, io::Read};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ragdoll::components::*};

pub fn init_skeleton(
    commands: &mut Commands,
    rigid_body_type: RigidBody,
) {
    let origin_halfs = Vec2::new(1.0, 1.0);
    let origin_bone = commands.spawn((
        Name::new("origin"),
        VisibilityBundle::default(),
        rigid_body_type,
        Collider::cuboid(origin_halfs.x, origin_halfs.y),
        TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
        Velocity::zero(),
        Sleeping::disabled(),
        Bone {},
    ))
    .id();
    add_bone(commands, origin_bone, Vec2::ZERO);
}

pub fn add_bone(
    commands: &mut Commands,
    parent: Entity,
    offset: Vec2,
) {
    let joint = RevoluteJointBuilder::new()
    .local_anchor1(Vec2::Y * 3.)
    .local_anchor2(Vec2::Y * -3.);

    commands.spawn(RigidBody::Dynamic)
    .insert((
        ImpulseJoint::new(parent, joint),
        Collider::cuboid(2., 2.),
        TransformBundle::from(Transform::from_xyz(5., 10., 0.)),
        Velocity::zero(),
        Sleeping::disabled(),
        Bone {},
    ));
}

pub fn ph(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ragdoll_save_event: EventWriter<RagdollSave>,
) {
    if keyboard_input.just_released(KeyCode::KeyL) {
        ragdoll_save_event.send(load_ragdoll_save());
    }
}

pub fn load_ragdoll(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut ragdoll_save_event: EventReader<RagdollSave>,
    mut name2handle: ResMut<Name2Handle>,
) {
    for ragdoll_save in ragdoll_save_event.read() {
        let handle = load_atlas(&ragdoll_save.name, &mut asset_server, &mut name2handle);
        println!("{:?}", ragdoll_save.name);
        for (i, j) in ragdoll_save.saves.iter().enumerate() {
            println!("sel {}: {:?}", i, j)
        }
        let mut map = HashMap::new();
        for save in ragdoll_save.saves.iter() {
            // todo: can old entity duplicate new?
            // future child entity processing
            let parent_entity;
            if let Some(new_child_entity) = map.get(&save.entity) {
                let hs = save.ulrect.half_size();
                parent_entity = commands.entity(*new_child_entity).insert((
                    Collider::cuboid(hs.x, hs.y),
                    TransformBundle::from(Transform::from_translation(save.lpos.extend(0.))),   
                )).id();
            } else {
                // init first parent
                let parent_hs = save.ulrect.half_size();
                parent_entity = commands.spawn((
                    RigidBody::Dynamic,
                    SpriteBundle {
                        texture: handle.clone(),
                        transform: Transform::from_translation(save.lpos.extend(0.)),
                        sprite: Sprite {
                            rect: Some(save.ulrect),
                            ..default()
                        },
                        ..default()
                    },
                    Collider::cuboid(parent_hs.x, parent_hs.y),
                )).id();
            }
            // init children from joints:
            for (child_entity, joint) in save.joints.iter() {
                let joint = RevoluteJointBuilder::new()
                .local_anchor1(joint.origin1)
                .local_anchor2(joint.origin2);
                let new_child_entity = commands.spawn(RigidBody::Dynamic)
                .insert((
                    SpriteBundle {
                        texture: handle.clone(),
                        transform: Transform::from_translation(save.lpos.extend(0.)),
                        sprite: Sprite {
                            rect: Some(save.ulrect),
                            ..default()
                        },
                        ..default()
                    },
                    ImpulseJoint::new(parent_entity, joint),
                    Bone {},
                )).id();
                map.insert(child_entity, new_child_entity);
            }
        }
    }
}


pub fn load_atlas(
    name: &String,
    asset_server: &mut ResMut<AssetServer>,
    name2handle: &mut ResMut<Name2Handle>,
) -> Handle<Image> {
    if let Some(handle) = name2handle.get(name) {
        return handle.clone();
    } else {
        // todo: add a check if a file exists
        let handle = asset_server.load(name);
        name2handle.insert(name.clone(), handle.clone());
        return handle;
    }
}

pub fn load_ragdoll_save(

) -> RagdollSave {
    let path = "C:/Users/yaro4/Downloads/ragdoll.bin";
    let mut file = File::open(path).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let save = bincode::deserialize(&buf).unwrap();
    save
}