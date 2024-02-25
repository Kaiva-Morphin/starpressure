use bevy::{asset::Assets, ecs::{entity::Entity, system::{Commands, ResMut}}, math::{Vec2, Vec3}, prelude::default, render::{color::Color, mesh::{Indices, Mesh, PrimitiveTopology}, render_asset::RenderAssetUsages}, sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle}, transform::components::Transform};
use bevy_rapier2d::{dynamics::{GravityScale, RigidBody, Velocity}, geometry::Collider};













pub fn spawn_plyer_puppet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
) -> Entity{
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        Vec3::from([100., 100., 0.]),
        Vec3::from([100., -100., 0.]),
        Vec3::from([-100., -100., 0.]),
        Vec3::from([-100., 100., 0.]),
    ]);        
    mesh.insert_indices(Indices::U32(vec![0, 1, 1, 2, 2, 3, 3, 0]));
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![color.as_rgba_f32(); 4]);
    commands.spawn((
        RigidBody::Dynamic,
        Velocity::zero(),
        GravityScale(0.),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh)),
            transform: Transform::from_translation(Vec3::ZERO),
            material: materials.add(ColorMaterial::default()),
            ..default()
        },
    )).id()
}