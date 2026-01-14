use crate::containers::FluentScroll;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct BricsPlugin;

impl Plugin for BricsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_fluent_scroll, animate_fluent_scroll));
    }
}

fn handle_fluent_scroll(
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut scroll_q: Query<&mut FluentScroll>,
) {
    for ev in mouse_wheel.read() {
        for mut scroll in scroll_q.iter_mut() {
            scroll.target = (scroll.target + ev.y * 50.0).clamp(scroll.min, scroll.max);
        }
    }
}

fn animate_fluent_scroll(time: Res<Time>, mut query: Query<(&mut FluentScroll, &mut Node)>) {
    let dt = time.delta_secs();
    for (mut scroll, mut node) in query.iter_mut() {
        scroll.current += (scroll.target - scroll.current) * 10.0 * dt;
        node.top = Val::Px(scroll.current);
    }
}
