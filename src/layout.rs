use bevy::prelude::*;

/// A helper bundle for a flexbox column (vertical stack).
#[derive(Bundle, Clone, Debug)]
pub struct Col {
    pub node: Node,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for Col {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            transform: default(),
            global_transform: default(),
        }
    }
}

impl Col {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn center(mut self) -> Self {
        self.node.align_items = AlignItems::Center;
        self.node.justify_content = JustifyContent::Center;
        self
    }

    pub fn fill(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self.node.height = Val::Percent(100.0);
        self
    }

    pub fn pad(mut self, px: f32) -> Self {
        self.node.padding = UiRect::all(Val::Px(px));
        self
    }

    pub fn gap(mut self, px: f32) -> Self {
        self.node.row_gap = Val::Px(px);
        self
    }

    pub fn grow(mut self, factor: f32) -> Self {
        self.node.flex_grow = factor;
        self
    }

    pub fn size(mut self, w: Val, h: Val) -> Self {
        self.node.width = w;
        self.node.height = h;
        self
    }

    pub fn align_end(mut self) -> Self {
        self.node.align_items = AlignItems::FlexEnd;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

/// A helper bundle for a flexbox row (horizontal stack).
#[derive(Bundle, Clone, Debug)]
pub struct Row {
    pub node: Node,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for Row {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                ..default()
            },
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            transform: default(),
            global_transform: default(),
        }
    }
}

impl Row {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn center(mut self) -> Self {
        self.node.justify_content = JustifyContent::Center;
        self
    }

    pub fn spread(mut self) -> Self {
        self.node.justify_content = JustifyContent::SpaceBetween;
        self
    }

    pub fn fill(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self
    }

    pub fn pad(mut self, px: f32) -> Self {
        self.node.padding = UiRect::all(Val::Px(px));
        self
    }

    pub fn gap(mut self, px: f32) -> Self {
        self.node.column_gap = Val::Px(px);
        self
    }

    pub fn grow(mut self, factor: f32) -> Self {
        self.node.flex_grow = factor;
        self
    }

    pub fn size(mut self, w: Val, h: Val) -> Self {
        self.node.width = w;
        self.node.height = h;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

/// A spacer entity that expands to fill available space.
#[derive(Bundle, Clone, Debug, Default)]
pub struct Spacer {
    pub node: Node,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            node: Node {
                flex_grow: 1.0,
                ..default()
            },
            ..default()
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

/// A helper bundle for a CSS Grid container.
#[derive(Bundle, Clone, Debug)]
pub struct Grid {
    pub node: Node,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![GridTrack::auto()],
                ..default()
            },
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            transform: default(),
            global_transform: default(),
        }
    }
}

impl Grid {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of columns (equal width flexible tracks).
    pub fn cols(mut self, count: u16) -> Self {
        self.node.grid_template_columns = vec![GridTrack::flex(1.0); count as usize];
        self
    }

    /// Sets up a responsive grid where columns automatically fit based on a minimum width.
    /// This is the "Cheaty Grid" mode - just pour items in and they flow.
    pub fn responsive_cols(mut self, min_width_px: f32) -> Self {
        self.node.grid_template_columns = vec![GridTrack::minmax(
            MinTrackSizingFunction::Px(min_width_px),
            MaxTrackSizingFunction::Fraction(1.0),
        )];
        self.node.grid_auto_rows = vec![GridTrack::auto()];
        self
    }

    /// Set the number of rows (equal height flexible tracks).
    pub fn rows(mut self, count: u16) -> Self {
        self.node.grid_template_rows = vec![GridTrack::flex(1.0); count as usize];
        self
    }

    pub fn grow(mut self, factor: f32) -> Self {
        self.node.flex_grow = factor;
        self
    }

    pub fn width(mut self, w: Val) -> Self {
        self.node.width = w;
        self
    }

    pub fn height(mut self, h: Val) -> Self {
        self.node.height = h;
        self
    }

    pub fn fill(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self.node.height = Val::Percent(100.0);
        self
    }

    pub fn gap(mut self, px: f32) -> Self {
        self.node.row_gap = Val::Px(px);
        self.node.column_gap = Val::Px(px);
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

/// A helper bundle for placing an item in a specific Grid slot.
#[derive(Bundle, Clone, Debug)]
pub struct GridCell {
    pub node: Node,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for GridCell {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                ..default()
            },
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            transform: default(),
            global_transform: default(),
        }
    }
}

impl GridCell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn row(mut self, start: i16, span: u16) -> Self {
        self.node.grid_row = GridPlacement::start_span(start, span);
        self
    }

    pub fn col(mut self, start: i16, span: u16) -> Self {
        self.node.grid_column = GridPlacement::start_span(start, span);
        self
    }
}
