use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{CursorEntity, CursorPosition};

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