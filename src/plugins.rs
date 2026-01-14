use crate::containers::FluentScroll;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct BricsPlugin;

use crate::containers::FluentPanel;
use crate::theme::COLOR_VOID;
use crate::widgets::{FluentLabel, Tooltip, TooltipUi};

impl Plugin for BricsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_fluent_scroll, animate_fluent_scroll, handle_tooltips),
        );
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

fn handle_tooltips(
    mut commands: Commands,
    interaction_q: Query<(&Interaction, &Tooltip, &GlobalTransform), Changed<Interaction>>,
    tooltip_ui_q: Query<Entity, With<TooltipUi>>,
) {
    for (interaction, tooltip, transform) in interaction_q.iter() {
        match *interaction {
            Interaction::Hovered => {
                // Despawn old tooltip if any
                for entity in tooltip_ui_q.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                let pos = transform.translation();

                // Spawn new tooltip
                commands
                    .spawn((
                        FluentPanel::default()
                            .bg(COLOR_VOID.with_alpha(0.98))
                            .border(Color::WHITE, 1.0)
                            .pad(8.0)
                            .size(Val::Auto, Val::Auto),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(pos.x + 20.0),
                            top: Val::Px(pos.y - 40.0),
                            ..default()
                        },
                        ZIndex(100),
                        TooltipUi,
                    ))
                    .with_children(|p| {
                        p.spawn(FluentLabel::new(&tooltip.0).size(12.0));
                    });
            }
            Interaction::None => {
                for entity in tooltip_ui_q.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            _ => {}
        }
    }
}
