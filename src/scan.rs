use bevy::prelude::*;

/// A component for a tactical scanline overlay.
#[derive(Component, Clone, Debug)]
pub struct Scanner {
    pub speed: f32,
    pub color: Color,
    pub line_count: usize,
    pub spacing: f32,
    pub vertical: bool,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            speed: 1.0,
            color: Color::srgba(0.0, 1.0, 1.0, 0.2),
            line_count: 5,
            spacing: 20.0,
            vertical: false,
        }
    }
}

/// A fluent builder for creating a Scanner overlay.
#[derive(Bundle, Clone, Debug, Default)]
pub struct FluentScanner {
    pub scanner: Scanner,
    pub node: Node,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl FluentScanner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.scanner.speed = speed;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.scanner.color = color;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.scanner.vertical = true;
        self
    }
}

pub fn animate_scanner(
    time: Res<Time>,
    query: Query<(&Scanner, &ComputedNode, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    let t = time.elapsed_secs();
    for (scanner, computed_node, transform) in query.iter() {
        let size = computed_node.size();
        let center = transform.translation().truncate();
        let half_size = size / 2.0;

        // Calculate line offsets based on time and speed
        let offset = (t * scanner.speed * 50.0) % scanner.spacing;

        for i in 0..scanner.line_count {
            let pos = -half_size + offset + (i as f32 * scanner.spacing);

            if scanner.vertical {
                if pos < half_size.x && pos > -half_size.x {
                    let p1 = center + Vec2::new(pos, -half_size.y);
                    let p2 = center + Vec2::new(pos, half_size.y);
                    gizmos.line_2d(p1, p2, scanner.color);
                }
            } else {
                if pos < half_size.y && pos > -half_size.y {
                    let p1 = center + Vec2::new(-half_size.x, pos);
                    let p2 = center + Vec2::new(half_size.x, pos);
                    gizmos.line_2d(p1, p2, scanner.color);
                }
            }
        }
    }
}
