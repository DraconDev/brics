use bevy::prelude::*;

/// A reusable component for drawing technical hex visuals using Gizmos.
pub struct TechGizmos;

impl TechGizmos {
    pub fn draw_hex(gizmos: &mut Gizmos, position: Vec2, size: f32, color: Color, _segments: u32) {
        for k in 0..6 {
            let a1 = k as f32 * std::f32::consts::PI / 3.0 + std::f32::consts::FRAC_PI_2;
            let a2 = (k + 1) as f32 * std::f32::consts::PI / 3.0 + std::f32::consts::FRAC_PI_2;
            let p1 = position + Vec2::new(a1.cos(), a1.sin()) * size;
            let p2 = position + Vec2::new(a2.cos(), a2.sin()) * size;
            gizmos.line_2d(p1, p2, color);
        }
    }
}
