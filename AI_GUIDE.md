# BRICS AI Guide

**BRICS** (Bevy Rapid Interface Construction System) is designed to make UI development in Bevy 0.18+ fast, readable, and "cheaty". It abstracts away the verbosity of `Node` bundles and CSS-like Flexbox boilerplates into fluent, semantic builders.

## Core Philosophy

1.  **Fluent Builders**: Chain methods to set properties (`.size()`, `.bg()`, `.pad()`).
2.  **Cheaty Defaults**: Components like `AutoList` and `Grid` come with sensible defaults (gaps, alignment) so you don't have to think about them.
3.  **Semantic Components**: Use `FluentButton` instead of `Button` + `Node` + `BorderColor` + `BackgroundColor`.

## Layout

### `Col` (Column)
A vertical flex stack.
```rust
Col::new()
    .fill()         // width: 100%, height: 100%
    .center()       // align-items: center, justify-content: center
    .gap(10.0)      // row-gap: 10px
    .pad(20.0)      // padding: 20px
```

### `Row` (Row)
A horizontal flex stack.
```rust
Row::new()
    .fill()         // width: 100%
    .spread()       // justify-content: space-between
    .gap(20.0)
```

### `AutoList` (The "Just Stack It" Container)
The easiest way to make a menu or list. Defaults to vertical, 10px gap.
```rust
AutoList::new()     // Vertical by default
    .horizontal()   // Switch to horizontal
    .gap(15.0)
```

### `Grid` (CSS Grid)
```rust
Grid::new()
    .cols(3)        // 3 equal columns (1fr 1fr 1fr)
    .gap(10.0)
    // .responsive_cols(100.0) // Auto-fit columns (WIP, use .cols() for now)
```

### `Spacer`
Pushes content apart in Flex containers.
```rust
Spacer::new() // flex-grow: 1.0
```

## Widgets

### `FluentButton`
A button with background, border, and radius pre-configured.
```rust
FluentButton::new()
    .size(200.0, 50.0)
    .bg(COLOR_CYAN)
    .border(Color::WHITE, 2.0)
    .rounded(8.0)
```

### `FluentLabel`
Text with easy styling.
```rust
FluentLabel::new("SYSTEM READY")
    .size(24.0)
    .color(COLOR_TEAL)
    .bold()
    .centered()
```

### `FluentBar`
A progress/health bar.
```rust
FluentBar::new()
    .size(Val::Px(200.0), Val::Px(10.0))
    .color(COLOR_RED)
    // To animate:
    // bar.spawn_fill(parent, COLOR_RED, 50.0);
```

### `Clickable`
Invisible interactive node. Use when you want a custom look (like an image) to be clickable.
```rust
Clickable::new()
```

## Containers (FUI)

### `FluentPanel`
A sci-fi panel with optional brackets and scanlines.
```rust
FluentPanel::new()
    .bg(COLOR_VOID.with_alpha(0.9))
    .border(COLOR_CYAN_DIM, 1.0)
    // Extensions
    .with_brackets(parent, COLOR_CYAN, 20.0, 2.0)
    .with_scanlines(parent, COLOR_TEAL, 5)
```

## Transforms
All widgets and layout containers support `.with_transform(Transform::...)` to easily position them in 3D space (Z-index layering).

---

**Usage Rule**: Always prefer these components over raw `Node` bundles unless you need something extremely specific.
