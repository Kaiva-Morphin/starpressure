use bevy::{input::mouse::MouseWheel, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_egui::egui::Shape;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{components::{Box, CursorEntity, CursorPosition, WindowSize}, ship::{ tiles::systems::{TILE_SIZE, TILE_SIZE_U32, TILE_SIZE_USIZE}}};

pub fn raycast(
    rapier_context: Res<RapierContext>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<&Camera>,
    mut cursor_pos_res: ResMut<CursorPosition>,
    mut cursor_entity_res: ResMut<CursorEntity>
) {
    let window = window_q.single();
    if let Some(cursor_pos) = window.physical_cursor_position() {
        let camera = camera_q.single();
        let ndc = Vec4::new(
            (cursor_pos.x / window.physical_width() as f32) * 2. - 1.,
            (cursor_pos.y / window.physical_height() as f32) * 2. - 1.,
            0.,
            1.,
        );
        
        let ray_pos = camera.projection_matrix().inverse() * ndc;
        let mut ray_pos = ray_pos.xy();
        ray_pos.y = -ray_pos.y;
        let ray_dir = Vec2::new(0.0, 1.0);
        let max_toi = 4.0;
        let solid = true;
        let filter = QueryFilter::default();
        cursor_pos_res.pos = ray_pos;
        if let Some((entity, _)) = rapier_context.cast_ray(
            ray_pos, ray_dir, max_toi, solid, filter
        ) {
            cursor_entity_res.entity = Some(entity);
        } else {
            cursor_entity_res.entity = None;
        }
    }
}

pub fn set_window_size(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut window_size: ResMut<WindowSize>
) {
    let window = window_q.single();
    window_size.width = window.physical_width();
    window_size.height = window.physical_height();
}

pub fn draw_penis(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ponos ebychii
    println!("hui");
    commands.spawn(
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(111., 1000.))),
            material: materials.add(ColorMaterial { color: Color::WHITE, texture: None }),
            ..default()
        }
    );
}

pub fn draw_net(
    mut gizmos: Gizmos,
    window_size: Res<WindowSize>,
    camera_q: Query<&Camera>,
) {
    let camera = camera_q.single();
    
    let xe = window_size.width as f32;
    let ye = window_size.height as f32;

    let ndc = Vec4::new(- 1.,- 1.,0.,1.,);
    let corner = (camera.projection_matrix().inverse() * ndc).xy();

    for (x, _) in (0..window_size.width).step_by(TILE_SIZE_USIZE).enumerate() {
        let xs = x as f32 * TILE_SIZE;
        gizmos.line_2d(Vec2::new(xs, 0.,) + corner, Vec2::new(xs, ye) + corner, Color::WHITE);
    }

    for (y, _) in (0..window_size.height).step_by(TILE_SIZE_USIZE).enumerate() {
        let ys = y as f32 * TILE_SIZE;
        gizmos.line_2d(Vec2::new(0., ys,) + corner, Vec2::new(xe, ys) + corner, Color::WHITE);
    }
}

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(3., 3., 0.,)),
        ..default()
    });
}

pub fn free_camera_controller(
    mut camera_q: Query<&mut Transform, With<Camera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scroll_ev: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let mut camera_transform = camera_q.single_mut();
    let mut direction = Vec3::ZERO;
    let mut ms = 50.;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1., 0., 0.);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1., 0., 0.);
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0., 1., 0.);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0., -1., 0.);
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        ms *= 2.;
    }

    camera_transform.translation.x += direction.x * ms * dt;
    camera_transform.translation.y += direction.y * ms * dt;

    for ev in scroll_ev.read() {
        camera_transform.scale.x = (camera_transform.scale.x - ev.y * 0.2).clamp(0.1, 1.);
        camera_transform.scale.y = (camera_transform.scale.y - ev.y * 0.2).clamp(0.1, 1.);
    }
}

pub fn spawn_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("box.png"),
            ..default()
        },
        Box,
        RigidBody::Dynamic,
        Velocity::default(),
        Collider::cuboid(2., 2.),
    ));
}

pub fn spawn_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Collider::cuboid(200., 2.),
        Transform::from_xyz(0., -90., 0.),
        Name::new("floor"),
    ));
}

