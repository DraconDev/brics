use bevy::prelude::*;

/// A tactical list container that simplifies stacking items.
/// Supports vertical and horizontal layouts with consistent spacing.
#[derive(Bundle, Clone, Debug)]
pub struct FluentList {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for FluentList {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch, // Default to stretch for list items
                justify_content: JustifyContent::Start,
                row_gap: Val::Px(4.0), // Tight tactical gap
                ..default()
            },
            background_color: Color::NONE.into(),
            border_color: Color::NONE.into(),
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            transform: default(),
            global_transform: default(),
        }
    }
}

impl FluentList {
    pub fn new() -> Self {
        Self::default()
    }

    /// Switches to horizontal layout.
    pub fn horizontal(mut self) -> Self {
        self.node.flex_direction = FlexDirection::Row;
        self.node.column_gap = self.node.row_gap;
        self.node.row_gap = Val::Px(0.0);
        self.node.align_items = AlignItems::Center;
        self
    }

    /// Sets the gap between items.
    pub fn gap(mut self, px: f32) -> Self {
        if self.node.flex_direction == FlexDirection::Row {
            self.node.column_gap = Val::Px(px);
        } else {
            self.node.row_gap = Val::Px(px);
        }
        self
    }

    /// Sets the padding of the list.
    pub fn pad(mut self, px: f32) -> Self {
        self.node.padding = UiRect::all(Val::Px(px));
        self
    }

    /// Center items in the list.
    pub fn center(mut self) -> Self {
        self.node.align_items = AlignItems::Center;
        self.node.justify_content = JustifyContent::Center;
        self
    }

    /// Sets a background color for the list container.
    pub fn bg(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }
}
