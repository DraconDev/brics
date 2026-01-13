
use bevy_egui::egui;
use jitui::schema::{Jitui, JituiType};

pub fn render(ui: &mut egui::Ui, jitui: &Jitui) {
    match jitui.jitui_type {
        JituiType::Terminal => {
            if let Some(title) = &jitui.title {
                ui.label(egui::RichText::new(title).monospace().strong());
                ui.separator();
            }
            ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_rgb(10, 10, 15);

            let id = ui.make_persistent_id(&jitui.id).with("term_scroll");
            egui::ScrollArea::vertical()
                .id_source(id)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut "User@Demon:~$ System ready.\n> Waiting for stream...\n> [LOG] Connected to deployment stream.")
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .text_color(egui::Color32::from_rgb(0, 255, 0))
                            .lock_focus(true)
                    );
                });
        }
        JituiType::LogStream => {
            if let Some(title) = &jitui.title {
                ui.label(egui::RichText::new(title).monospace().strong());
                ui.separator();
            }
            ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_rgb(5, 5, 10);

            let id = ui.make_persistent_id(&jitui.id).with("log_scroll");
            egui::ScrollArea::vertical()
                .id_source(id)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    if let Some(events) = jitui.data.get("events").and_then(|v| v.as_array()) {
                        for event in events {
                            let ts = event.get("ts").and_then(|s| s.as_str()).unwrap_or("");
                            let level = event
                                .get("level")
                                .and_then(|s| s.as_str())
                                .unwrap_or("info");
                            let msg = event.get("msg").and_then(|s| s.as_str()).unwrap_or("");

                            let color = match level.to_lowercase().as_str() {
                                "err" | "error" => egui::Color32::RED,
                                "warn" => egui::Color32::YELLOW,
                                _ => egui::Color32::LIGHT_GRAY,
                            };

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(ts).size(10.0).weak());
                                ui.label(
                                    egui::RichText::new(format!("[{}]", level.to_uppercase()))
                                        .color(color)
                                        .monospace(),
                                );
                                ui.label(egui::RichText::new(msg).monospace());
                            });
                        }
                    }
                });
        }
        _ => {}
    }
}
