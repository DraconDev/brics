use crate::theme::*;
use bevy::prelude::*;

/// A tactical FUI panel with optional brackets and scanlines.
#[derive(Bundle, Clone, Debug)]
pub struct FluentPanel {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for FluentPanel {
    fn default() -> Self {
        Self {
            node: Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                border_radius: BorderRadius::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(COLOR_VOID.with_alpha(0.95)),
            border_color: BorderColor::default(),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
        }
    }
}

impl FluentPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.background_color = BackgroundColor(color);
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = BorderColor(color);
        self.node.border = UiRect::all(Val::Px(width));
        self
    }

    pub fn pad(mut self, px: f32) -> Self {
        self.node.padding = UiRect::all(Val::Px(px));
        self
    }

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.node.width = width;
        self.node.height = height;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    /// Spawns tactical brackets at the corners of this panel.
    pub fn with_brackets(
        self,
        parent: &mut ChildSpawnerCommands,
        color: Color,
        size: f32,
        thickness: f32,
    ) -> Self {
        // Top Left
        spawn_bracket(
            parent,
            color,
            size,
            thickness,
            Some(0.0),
            Some(0.0),
            None,
            None,
        );
        // Top Right
        spawn_bracket(
            parent,
            color,
            size,
            thickness,
            Some(0.0),
            None,
            None,
            Some(0.0),
        );
        // Bottom Left
        spawn_bracket(
            parent,
            color,
            size,
            thickness,
            None,
            Some(0.0),
            Some(0.0),
            None,
        );
        // Bottom Right
        spawn_bracket(
            parent,
            color,
            size,
            thickness,
            None,
            None,
            Some(0.0),
            Some(0.0),
        );
        self
    }

    /// Spawns horizontal scanlines over the panel area.
    pub fn with_scanlines(
        self,
        parent: &mut ChildSpawnerCommands,
        color: Color,
        count: usize,
    ) -> Self {
        for i in 0..count {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    top: Val::Percent(i as f32 * (100.0 / count as f32)),
                    ..default()
                },
                BackgroundColor(color.with_alpha(0.05)),
            ));
        }
        self
    }
}

fn spawn_bracket(
    parent: &mut ChildSpawnerCommands,
    color: Color,
    size: f32,
    thickness: f32,
    top: Option<f32>,
    left: Option<f32>,
    bottom: Option<f32>,
    right: Option<f32>,
) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: top.map(Val::Px).unwrap_or(Val::Auto),
            left: left.map(Val::Px).unwrap_or(Val::Auto),
            bottom: bottom.map(Val::Px).unwrap_or(Val::Auto),
            right: right.map(Val::Px).unwrap_or(Val::Auto),
            width: Val::Px(size),
            height: Val::Px(size),
            border: UiRect {
                top: if top.is_some() {
                    Val::Px(thickness)
                } else {
                    Val::Px(0.0)
                },
                left: if left.is_some() {
                    Val::Px(thickness)
                } else {
                    Val::Px(0.0)
                },
                bottom: if bottom.is_some() {
                    Val::Px(thickness)
                } else {
                    Val::Px(0.0)
                },
                right: if right.is_some() {
                    Val::Px(thickness)
                } else {
                    Val::Px(0.0)
                },
            },
            ..default()
        },
        BorderColor(color),
    ));
}

/// A fluent builder for scrollable areas.
#[derive(Component, Default, Clone, Debug)]
pub struct FluentScroll {
    pub current: f32,
    pub target: f32,
    pub min: f32,
    pub max: f32,
}

impl FluentScroll {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            ..default()
        }
    }
}
