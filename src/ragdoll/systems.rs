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
        //KinematicCharacterController {
        //    translation: Some(pos),
        //    ..default()
        //},
        RigidBody::Dynamic,
        Collider::cuboid(origin_halfs.x, origin_halfs.y),
        TransformBundle::from(Transform::from_xyz(10., 20., 0.)),
        Velocity::zero(),
        Sleeping::disabled(),
    ))
    .id();

    let joint = RevoluteJointBuilder::new()
    .local_anchor1(Vec2::Y * 3.)
    .local_anchor2(Vec2::Y * -3.);
    
    //let joint = FixedJointBuilder::new().local_anchor1(Vec2::new(0.0, -2.0));
    commands.spawn(RigidBody::Dynamic)
        .insert((
            ImpulseJoint::new(origin_bone, joint),
            Collider::cuboid(origin_halfs.x, origin_halfs.y * 2.),
            TransformBundle::from(Transform::from_xyz(5., 10., 0.)),
            Velocity::zero(),
            Sleeping::disabled(),
        ))
        //.insert(CollisionGroups::new(0b1101.into(), 0b0100.into()))
        .insert(SolverGroups::new(default(), Group::from_bits(0b0100).unwrap()))
        // membership - то чем является
        // filter - то с чем коллизирует
    ;
}

pub fn init_bone(
    commands: &mut Commands,
    offset: Vec2,
) {

}