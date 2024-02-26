use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn init_skeleton(
    mut commands: Commands,
) {
    let pos = Vec2::new(1., 0.);
    let origin_halfs = Vec2::new(1.0, 1.0);
    let origin_bone = commands.spawn((
        Name::new("origin"),
        VisibilityBundle::default(),
        KinematicCharacterController {
            translation: Some(pos),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(origin_halfs.x, origin_halfs.y),
    ))
    .insert(Transform::from_xyz(10., 20., 0.),)
    .id();

    let joint = FixedJointBuilder::new()
    .local_anchor1(origin_halfs)
    .local_anchor2(origin_halfs);
    
    //let joint = FixedJointBuilder::new().local_anchor1(Vec2::new(0.0, -2.0));
    commands.spawn(RigidBody::Dynamic)
        .insert((
            ImpulseJoint::new(origin_bone, joint),
            Collider::cuboid(origin_halfs.x, origin_halfs.y),
        ))
        .insert(Transform::from_xyz(5., 10., 0.))
    ;
}

pub fn init_bone(
    commands: &mut Commands,
    offset: Vec2,
) {

}