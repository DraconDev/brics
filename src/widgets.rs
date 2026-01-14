use crate::theme::*;
use bevy::prelude::*;

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
            background_color: BackgroundColor(Color::NONE),
            border_color: BorderColor::all(Color::WHITE),
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
        self.background_color = BackgroundColor(color);
        self
    }

    /// Sets the button's border color and width.
    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = BorderColor::all(color);
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
            background_color: BackgroundColor(Color::NONE),
            border_color: BorderColor::all(COLOR_CYAN_DIM),
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
        self.border_color = BorderColor::all(color.with_alpha(0.3));
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
                BackgroundColor(color),
            ))
            .id()
    }
}
