# Brics 🧩

> [!WARNING] > **Work In Progress**: This crate is currently in early development and is part of the [Demon](https://github.com/DraconDev/demon) ecosystem. APIs are subject to change.

**Brics** (Bevy Reusable Interactive Component System) provides a flexible, modular system for building interactive UIs in [Bevy](https://bevyengine.org/) using [egui](https://github.com/emilk/egui). It is designed to render dynamic UI components based on the `jitui` schema.

## Features

- 📊 **Dynamic Charts**: Integrated time-series and status visualizations via `egui_plot`.
- 📝 **Markdown Rendering**: Built-in support for rendering markdown content directly within the UI.
- 🐚 **Terminal & Logs**: Specialized components for terminal emulation and real-time log streaming.
- 📑 **Data Grids**: Flexible table and process list components with striped layouts.
- ⚡ **Interactive Forms**: Support for buttons, action panels, and event-driven interactions.

## Core Modules

- `charts`: Time-series and progress visualizations.
- `markdown`: Rich text and document rendering.
- `terminal`: Monospaced terminal views and log streams.
- `tables`: Data grids, process lists, and status chips.
- `form`: Buttons and input handling.
- `interact`: Glue logic for UI-driven events.

## Integration

Brics is built to work seamlessly with `jitui` schemas, allowing you to define UI structures in data and render them with standardized Bevy components.
