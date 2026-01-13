use crate::events::InteractionEvent;

use bevy::prelude::EventWriter;
use bevy_egui::egui;
use jitui::schema::Jitui;

pub fn render(
    ui: &mut egui::Ui,
    jitui: &Jitui,
    interaction_events: &mut EventWriter<InteractionEvent>,
) {
    if let Some(title) = &jitui.title {
        ui.label(egui::RichText::new(title).strong().size(16.0));
        ui.separator();
    }
    ui.style_mut().spacing.item_spacing.y = 10.0;

    ui.vertical(|ui| {
        // "Hack": Use a simple single-field assumption for prototype 'Research'
        // Real form needs a HashMap of values.
        let mut input_val = String::new();
        let input_id = ui.make_persistent_id(&jitui.id).with("input");

        // Retrieve state
        if let Some(val) = ui.memory_mut(|mem| mem.data.get_temp::<String>(input_id)) {
            input_val = val;
        }

        if let Some(fields) = jitui.data.get("fields").and_then(|v| v.as_array()) {
            for field in fields {
                let label = field
                    .get("label")
                    .and_then(|s| s.as_str())
                    .unwrap_or("Field");
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(label).strong());
                    // Bind input
                    if ui
                        .add(egui::TextEdit::singleline(&mut input_val).hint_text("Enter URL..."))
                        .changed()
                    {
                        ui.memory_mut(|mem| mem.data.insert_temp(input_id, input_val.clone()));
                    }
                });
            }
        }

        let submit = jitui
            .data
            .get("submit_label")
            .and_then(|s| s.as_str())
            .unwrap_or("Submit");
        let btn = ui.add(
            egui::Button::new(egui::RichText::new(submit).size(14.0))
                .min_size(egui::vec2(100.0, 30.0)),
        );

        if btn.clicked() {
            // Send Event
            interaction_events.send(InteractionEvent {
                id: jitui.id.clone(),
                payload: serde_json::json!({
                    "action": "submit",
                    "value": input_val // Sending the raw value
                }),
            });
            ui.memory_mut(|mem| mem.data.remove::<String>(input_id)); // Clear on submit
        }
    });
}
