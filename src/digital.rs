use crate::widgets::FluentLabel;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum DigitalEffect {
    Typewriter,
    Glitch { intensity: f32 },
}

#[derive(Component, Clone, Debug)]
pub struct DigitalLabel {
    pub full_text: String,
    pub effect: DigitalEffect,
    pub timer: Timer,
    pub char_index: usize,
    pub finished: bool,
}

impl DigitalLabel {
    pub fn typewriter(text: impl Into<String>, chars_per_sec: f32) -> Self {
        Self {
            full_text: text.into(),
            effect: DigitalEffect::Typewriter,
            timer: Timer::from_seconds(1.0 / chars_per_sec, TimerMode::Repeating),
            char_index: 0,
            finished: false,
        }
    }

    pub fn glitch(text: impl Into<String>, duration: f32) -> Self {
        Self {
            full_text: text.into(),
            effect: DigitalEffect::Glitch { intensity: 0.5 },
            timer: Timer::from_seconds(duration, TimerMode::Once),
            char_index: 0,
            finished: false,
        }
    }
}

/// A fluent builder for creating an animated Digital Label.
#[derive(Bundle, Clone, Debug)]
pub struct FluentDigitalLabel {
    pub digital: DigitalLabel,
    pub label: FluentLabel,
}

impl FluentDigitalLabel {
    pub fn typewriter(text: impl Into<String>, cps: f32) -> Self {
        let text_s = text.into();
        Self {
            digital: DigitalLabel::typewriter(text_s.clone(), cps),
            label: FluentLabel::new(""), // Start empty
        }
    }
}

pub fn animate_scanner(
    time: Res<Time>,
    query: Query<(&Scanner, &ComputedNode, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    let t = time.elapsed_secs();
    for (scanner, computed_node, transform) in query.iter() {
        let size = computed_node.size();
            continue;
        }

        digital.timer.tick(time.delta());

pub fn update_digital_labels(time: Res<Time>, mut query: Query<(&mut DigitalLabel, &mut Text)>) {
    for (mut digital, mut text) in query.iter_mut() {
        if digital.finished {
            continue;
        }

        digital.timer.tick(time.delta());

        match digital.effect {
            DigitalEffect::Typewriter => {
                if digital.timer.just_finished() {
                    digital.char_index += 1;
                    if digital.char_index <= digital.full_text.len() {
                        // Correctly handle multi-byte characters by finding the byte index
                        let end = digital
                            .full_text
                            .char_indices()
                            .map(|(i, _)| i)
                            .nth(digital.char_index)
                            .unwrap_or(digital.full_text.len());

                        text.0 = digital.full_text[..end].to_string();
                    } else {
                        digital.finished = true;
                    }
                }
            }
            DigitalEffect::Glitch { intensity } => {
                if digital.timer.finished() {
                    text.0 = digital.full_text.clone();
                    digital.finished = true;
                } else {
                    // Randomize characters for glitch effect
                    let mut glitched = String::new();
                    for _ in 0..digital.full_text.chars().count() {
                        if rand::random::<f32>() < intensity {
                            let random_char = (rand::random::<u8>() % 94 + 33) as char;
                            glitched.push(random_char);
                        } else {
                            glitched.push(' ');
                        }
                    }
                    text.0 = glitched;
                }
            }
        }
    }
}
