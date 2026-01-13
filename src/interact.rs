use crate::events::InteractionEvent;

use bevy::prelude::EventWriter;
use bevy_egui::egui;
use jitui::schema::{Jitui, JituiType};

pub fn render(
    ui: &mut egui::Ui,
    jitui: &Jitui,
    interaction_events: &mut EventWriter<InteractionEvent>,
) {
    match jitui.jitui_type {
        JituiType::Button => {
            let label = jitui
                .data
                .get("label")
                .and_then(|v| v.as_str())
                .unwrap_or("Button");
            ui.vertical_centered(|ui| {
                if ui.button(label).clicked() {
                    interaction_events.send(InteractionEvent {
                        id: jitui.id.clone(),
                        payload: jitui.data.clone(),
                    });
                }
            });
        }
        JituiType::ActionPanel => {
            if let Some(title) = &jitui.title {
                ui.label(egui::RichText::new(title).strong());
            }
            ui.horizontal_wrapped(|ui| {
                if let Some(actions) = jitui.data.get("actions").and_then(|v| v.as_array()) {
                    for action in actions {
                        if let Some(label) = action.get("label").and_then(|s| s.as_str()) {
                            if ui.button(label).clicked() {
                                println!("Action Panel: {:?}", action);
                            }
                        }
                    }
                }
            });
        }
        _ => {}
    }
}
