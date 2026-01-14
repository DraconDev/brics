use crate::containers::FluentPanel;
use crate::theme::*;
use crate::widgets::FluentLabel;
use bevy::prelude::*;

/// A tactical FUI card template.
pub struct FluentCard;

impl FluentCard {
    /// Spawns a standard card with a header, art area, and stats.
    pub fn spawn(
        parent: &mut ChildSpawnerCommands,
        color: Color,
        title: &str,
        energy: u32,
        description: &str,
    ) -> Entity {
        parent
            .spawn((FluentPanel::new()
                .size(Val::Px(180.0), Val::Px(260.0))
                .border(color.with_alpha(0.4), 2.0)
                .bg(COLOR_VOID.with_alpha(0.98)),))
            .with_children(|card| {
                // Brackets
                FluentPanel::new().with_brackets(card, color, 15.0, 2.0);

                // Header
                card.spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|header| {
                    header.spawn(FluentLabel::new(title.to_uppercase()).size(14.0));

                    // Energy
                    header
                        .spawn((
                            Node {
                                margin: UiRect::top(Val::Px(4.0)),
                                padding: UiRect::horizontal(Val::Px(8.0)),
                                border: UiRect::bottom(Val::Px(1.0)),
                                ..default()
                            },
                            BorderColor::all(color.with_alpha(0.5)),
                        ))
                        .with_children(|nrg| {
                            nrg.spawn(
                                FluentLabel::new(format!("NRG: {}", energy))
                                    .size(10.0)
                                    .color(color),
                            );
                        });
                });

                // Art Area
                card.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.05, 0.1, 0.8)),
                    BorderColor::all(color.with_alpha(0.2)),
                ))
                .with_children(|art| {
                    FluentPanel::new().with_scanlines(art, color, 5);
                });

                // Description
                card.spawn(Node {
                    width: Val::Percent(100.0),
                    margin: UiRect::top(Val::Px(8.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    border: UiRect::left(Val::Px(2.0)),
                    ..default()
                })
                .with_children(|desc| {
                    desc.spawn(
                        FluentLabel::new(description)
                            .size(11.0)
                            .color(Color::srgb(0.7, 0.8, 0.9)),
                    );
                });
            })
            .id()
    }
}
