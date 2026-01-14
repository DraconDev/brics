use crate::theme::*;
use bevy::prelude::*;

/// A tactical tooltip.
#[derive(Component, Clone, Debug)]
pub struct Tooltip(pub String);

/// Marker for the spawned tooltip UI.
#[derive(Component)]
pub struct TooltipUi;

/// A fluent builder for creating Buttons descriptively.
#[derive(Bundle, Clone, Debug)]
pub struct FluentButton {
    pub button: Button,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
}

impl Default for FluentButton {
    fn default() -> Self {
        Self {
            button: Button,
            node: Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)), // Default comfy padding
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            background_color: Color::NONE.into(),
            border_color: Color::WHITE.into(),
            global_transform: default(),
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            transform: default(),
        }
    }
}

impl FluentButton {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the precise size of the button in pixels.
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.node.width = Val::Px(width);
        self.node.height = Val::Px(height);
        self
    }

    /// Sets the button's background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }

    /// Sets the button's border color and width.
    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = color.into();
        self.node.border = UiRect::all(Val::Px(width));
        self
    }

    /// Sets the border radius.
    pub fn rounded(mut self, px: f32) -> Self {
        self.node.border_radius = BorderRadius::all(Val::Px(px));
        self
    }

    /// Sets the padding.
    pub fn pad(mut self, px: f32) -> Self {
        self.node.padding = UiRect::all(Val::Px(px));
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn column(mut self) -> Self {
        self.node.flex_direction = FlexDirection::Column;
        self
    }

    pub fn spread(mut self) -> Self {
        self.node.justify_content = JustifyContent::SpaceBetween;
        self
    }

    pub fn with_tooltip(self, parent: &mut ChildSpawnerCommands, text: impl Into<String>) -> Self {
        let parent_entity = parent.target_entity();
        parent
            .commands()
            .entity(parent_entity)
            .insert(Tooltip(text.into()));
        self
    }
}

/// A fluent builder for creating Text.
#[derive(Bundle, Clone, Debug)]
pub struct FluentLabel {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
    pub layout: TextLayout,
    pub node: Node,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for FluentLabel {
    fn default() -> Self {
        Self {
            text: Text::new(""),
            font: TextFont {
                font_size: 14.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
            layout: TextLayout::default(),
            node: Node { ..default() },
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            transform: default(),
            global_transform: default(),
        }
    }
}

impl FluentLabel {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            text: Text::new(content),
            ..default()
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.font.font_size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = TextColor(color);
        self
    }

    pub fn bold(mut self) -> Self {
        // Simple mock for bold if we don't have multiple fonts loaded yet
        // In a real app this would switch the handle
        self.font.font_size *= 1.1;
        self
    }

    pub fn centered(mut self) -> Self {
        self.layout.justify = Justify::Center;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

/// A progress bar / telemetry indicator.
#[derive(Component, Clone, Debug)]
pub struct SegmentedBar {
    pub chunks: usize,
    pub current_chunk: usize,
    pub pulsing: bool,
}

#[derive(Bundle, Clone, Debug)]
pub struct FluentBar {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
}

impl Default for FluentBar {
    fn default() -> Self {
        Self {
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Px(8.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: Color::NONE.into(),
            border_color: COLOR_CYAN_DIM.into(),
            global_transform: default(),
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            transform: default(),
        }
    }
}

impl FluentBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.node.width = width;
        self.node.height = height;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.border_color = color.with_alpha(0.3).into();
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    /// Spawns the inner fill of the bar.
    pub fn spawn_fill(
        self,
        parent: &mut ChildSpawnerCommands,
        color: Color,
        percent: f32,
    ) -> Entity {
        parent
            .spawn((
                Node {
                    width: Val::Percent(percent.clamp(0.0, 100.0)),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor::from(color),
            ))
            .id()
    }

    /// Spawns a segmented bar.
    pub fn spawn_segmented(
        self,
        parent: &mut ChildSpawnerCommands,
        color: Color,
        chunks: usize,
        current: usize,
    ) -> Entity {
        let mut entity = parent.spawn((
            self.node.clone(),
            self.background_color,
            self.border_color,
            SegmentedBar {
                chunks,
                current_chunk: current,
                pulsing: false,
            },
        ));

        entity.with_children(|p| {
            for i in 0..chunks {
                p.spawn((
                    Node {
                        width: Val::Percent(100.0 / chunks as f32 - 1.0),
                        height: Val::Percent(100.0),
                        margin: UiRect::horizontal(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor::from(if i < current {
                        color
                    } else {
                        color.with_alpha(0.1)
                    }),
                ));
            }
        });

        entity.id()
    }
}

/// A simple clickable region without default styling.
/// Use this for custom UI elements that need interaction.
#[derive(Bundle, Clone, Debug, Default)]
pub struct Clickable {
    pub button: Button,
    pub node: Node,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

impl Clickable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

pub fn update_segmented_bars(
    time: Res<Time>,
    mut query: Query<(&SegmentedBar, &mut Children)>,
    mut bg_query: Query<&mut BackgroundColor>,
) {
    let t = time.elapsed_secs();
    for (bar, children) in query.iter_mut() {
        if bar.pulsing {
            let alpha = 0.5 + 0.5 * (t * 5.0).sin();
            for (i, child) in children.iter().enumerate() {
                if i < bar.current_chunk {
                    if let Ok(mut bg) = bg_query.get_mut(child) {
                        bg.0 = bg.0.with_alpha(alpha);
                    }
                }
            }
        }
    }
}
