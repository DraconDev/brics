/// A component for a tactical hexagonal grid.
#[derive(Component, Clone, Debug)]
pub struct HexGrid {
    pub cell_size: f32,
    pub color: Color,
    pub line_width: f32,
}

impl Default for HexGrid {
    fn default() -> Self {
        Self {
            cell_size: 32.0,
            color: Color::srgba(0.0, 1.0, 1.0, 0.1),
            line_width: 1.0,
        }
    }
}

pub struct TechGizmos;

impl TechGizmos {
    pub fn draw_hex(gizmos: &mut Gizmos, position: Vec2, size: f32, color: Color) {
        for k in 0..6 {
            let a1 = k as f32 * std::f32::consts::PI / 3.0 + std::f32::consts::FRAC_PI_2;
            let a2 = (k + 1) as f32 * std::f32::consts::PI / 3.0 + std::f32::consts::FRAC_PI_2;
            let p1 = position + Vec2::new(a1.cos(), a1.sin()) * size;
            let p2 = position + Vec2::new(a2.cos(), a2.sin()) * size;
            gizmos.line_2d(p1, p2, color);
        }
    }

    /// Converts axial coordinates (q, r) to 2D world coordinates.
    pub fn axial_to_world(q: i32, r: i32, size: f32) -> Vec2 {
        let x = size * 3.0f32.sqrt() * (q as f32 + r as f32 / 2.0);
        let y = size * 3.0 / 2.0 * r as f32;
        Vec2::new(x, y)
    }
}

pub fn draw_hex_grids(query: Query<(&HexGrid, &Node, &GlobalTransform)>, mut gizmos: Gizmos) {
    for (grid, node, transform) in query.iter() {
        let size = node.size();
        let center = transform.translation().truncate();
        let half_size = size / 2.0;

        // Calculate grid bounds
        let q_max = (half_size.x / (grid.cell_size * 3.0f32.sqrt())).ceil() as i32 + 1;
        let r_max = (half_size.y / (grid.cell_size * 1.5)).ceil() as i32 + 1;

        for q in -q_max..=q_max {
            for r in -r_max..=r_max {
                let world_pos = TechGizmos::axial_to_world(q, r, grid.cell_size);
                if world_pos.x.abs() < half_size.x && world_pos.y.abs() < half_size.y {
                    TechGizmos::draw_hex(
                        &mut gizmos,
                        center + world_pos,
                        grid.cell_size,
                        grid.color,
                    );
                }
            }
        }
    }
}
