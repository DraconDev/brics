use bevy::prelude::*;

// --- NEURAL TERMINAL PALETTE ---
pub const COLOR_VOID: Color = Color::BLACK; // Pure terminal void background
pub const COLOR_CYAN: Color = Color::srgba(0.0, 0.9, 1.0, 1.0); // High-energy core
pub const COLOR_TEAL: Color = Color::srgba(0.0, 0.4, 0.6, 1.0); // Primary technical
pub const COLOR_VOID_TEAL: Color = Color::srgba(0.0, 0.01, 0.03, 1.0); // Deep cinematic shadow
pub const COLOR_TEAL_BRIGHT: Color = Color::srgba(0.1, 0.8, 0.9, 1.0); // Highlight data
pub const COLOR_CYAN_DIM: Color = Color::srgba(0.0, 0.3, 0.4, 0.2); // Background data
pub const COLOR_ORANGE: Color = Color::srgba(1.0, 0.5, 0.0, 1.0); // Warnings / Interactive elements
pub const COLOR_RED: Color = Color::srgba(1.0, 0.1, 0.1, 1.0); // Critical / Combat damage / Enemy logic

pub const BTN_NORMAL: Color = Color::srgba(0.0, 0.1, 0.15, 0.8);
pub const BTN_HOVERED: Color = Color::srgba(0.0, 0.3, 0.4, 0.9);
pub const BTN_PRESSED: Color = Color::srgba(0.0, 0.8, 1.0, 1.0);
