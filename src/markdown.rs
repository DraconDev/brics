use bevy_egui::egui;
use jitui::schema::Jitui;

pub fn render(ui: &mut egui::Ui, jitui: &Jitui) {
    // If title is present, show it as an H1 equivalent
    if let Some(title) = &jitui.title {
        ui.label(egui::RichText::new(title).strong().size(18.0).underline());
        ui.add_space(5.0);
    }

    if let Some(content) = jitui.data.get("content").and_then(|v| v.as_str()) {
        render_markdown(ui, content);
    }
}

fn render_markdown(ui: &mut egui::Ui, content: &str) {
    let mut in_code_block = false;

    for line in content.lines() {
        if line.trim().starts_with("```") {
            in_code_block = !in_code_block;
            if in_code_block {
                ui.add_space(5.0);
            } else {
                ui.add_space(5.0);
            }
            continue;
        }

        if in_code_block {
            ui.label(
                egui::RichText::new(line)
                    .monospace()
                    .color(egui::Color32::from_rgb(180, 200, 180)) // Terminal green/white
                    .background_color(egui::Color32::from_rgb(30, 30, 35)),
            );
            continue;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            ui.add_space(5.0);
            continue;
        }

        if line.starts_with("# ") {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(line.trim_start_matches("# "))
                    .strong()
                    .size(20.0),
            );
            ui.separator();
        } else if line.starts_with("## ") {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(line.trim_start_matches("## "))
                    .strong()
                    .size(16.0),
            );
        } else if line.starts_with("### ") {
            ui.add_space(5.0);
            ui.label(
                egui::RichText::new(line.trim_start_matches("### "))
                    .strong()
                    .size(14.0),
            );
        } else if line.starts_with("- ") || line.starts_with("* ") {
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.label("•");
                ui.label(line[2..].trim());
            });
        } else if let Some(idx) = line.find(". ") {
            // Basic numbered list detection "1. "
            if line[..idx].chars().all(char::is_numeric) {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(line);
                });
            } else {
                ui.label(line);
            }
        } else {
            ui.label(line);
        }
    }
}
