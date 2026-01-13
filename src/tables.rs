use bevy_egui::egui;
use jitui::schema::{Jitui, JituiType};

pub fn render(ui: &mut egui::Ui, jitui: &Jitui) {
    match jitui.jitui_type {
        JituiType::ProcessList => {
            render_data_grid(
                ui,
                jitui,
                &["PID", "Name", "CPU%", "MEM%"],
                |item| {
                    let pid = item
                        .get("pid")
                        .and_then(|v| v.as_u64())
                        .map(|v| v.to_string())
                        .unwrap_or_default();
                    let name = item
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("?")
                        .to_string();
                    let cpu = item
                        .get("cpu")
                        .and_then(|v| v.as_f64())
                        .map(|v| format!("{:.1}", v))
                        .unwrap_or_default();
                    let mem = item
                        .get("mem")
                        .and_then(|v| v.as_f64())
                        .map(|v| format!("{:.1}", v))
                        .unwrap_or_default();
                    vec![pid, name, cpu, mem]
                },
                "processes",
            );
        }
        JituiType::StatusGrid => {
            if let Some(title) = &jitui.title {
                ui.label(
                    egui::RichText::new(title)
                        .strong()
                        .color(egui::Color32::LIGHT_BLUE),
                );
            }
            ui.horizontal_wrapped(|ui| {
                if let Some(items) = jitui.data.get("items").and_then(|v| v.as_array()) {
                    for item in items {
                        let label = item.get("label").and_then(|s| s.as_str()).unwrap_or("?");
                        let status = item.get("status").and_then(|s| s.as_str()).unwrap_or("ok");
                        let color = match status {
                            "err" | "error" => egui::Color32::from_rgb(255, 100, 100),
                            "warn" => egui::Color32::from_rgb(255, 200, 0),
                            _ => egui::Color32::from_rgb(100, 255, 100),
                        };
                        // Chip style
                        egui::Frame::group(ui.style())
                            .fill(color.gamma_multiply(0.15))
                            .stroke(egui::Stroke::new(1.0, color.gamma_multiply(0.5)))
                            .inner_margin(4.0)
                            .rounding(4.0)
                            .show(ui, |ui| {
                                ui.colored_label(color, format!("● {}", label));
                            });
                    }
                }
            });
        }
        JituiType::Table => {
            let headers: Vec<&str> = jitui
                .data
                .get("headers")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();

            render_data_grid(
                ui,
                jitui,
                &headers,
                |item| {
                    if let Some(row) = item.as_array() {
                        row.iter()
                            .map(|v| v.as_str().unwrap_or("").to_string())
                            .collect()
                    } else {
                        vec![]
                    }
                },
                "rows",
            );
        }
        _ => {}
    }
}

fn render_data_grid<F>(
    ui: &mut egui::Ui,
    jitui: &Jitui,
    headers: &[&str],
    row_extractor: F,
    data_key: &str,
) where
    F: Fn(&serde_json::Value) -> Vec<String>,
{
    if let Some(title) = &jitui.title {
        ui.label(egui::RichText::new(title).strong().size(16.0));
        ui.separator();
    }

    egui::Grid::new(&jitui.id)
        .striped(true)
        .min_col_width(80.0)
        .show(ui, |ui| {
            // Headers
            for header in headers {
                ui.label(egui::RichText::new(*header).strong().underline());
            }
            ui.end_row();

            // Data
            if let Some(items) = jitui.data.get(data_key).and_then(|v| v.as_array()) {
                for item in items {
                    let cells = row_extractor(item);
                    for cell in cells {
                        ui.label(cell);
                    }
                    ui.end_row();
                }
            }
        });
}
