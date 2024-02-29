use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ragdoll::components::*;

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