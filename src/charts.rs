use bevy_egui::egui;
use egui_plot::{Line, Plot, PlotPoints};
use jitui::schema::Jitui;

pub fn render(ui: &mut egui::Ui, jitui: &Jitui) {
    if let Some(title) = &jitui.title {
        ui.label(egui::RichText::new(title).small());
    }

    let variant = jitui
        .data
        .get("variant")
        .and_then(|v| v.as_str())
        .unwrap_or("TimeSeries");

    match variant {
        "TimeSeries" => {
            let points: PlotPoints = jitui
                .data
                .get("points")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|p| {
                            let p = p.as_array()?;
                            if p.len() >= 2 {
                                Some([p[0].as_f64()?, p[1].as_f64()?])
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .unwrap_or_else(|| {
                    // Fallback to sine wave for demo
                    (0..1000)
                        .map(|i| {
                            let x = i as f64 * 0.01;
                            [x, x.sin()]
                        })
                        .collect()
                });
            let line = Line::new(points);
            Plot::new(&jitui.id)
                .view_aspect(2.0)
                .show(ui, |plot_ui| plot_ui.line(line));
        }
        "Bar" | "Pie" | _ => {
            ui.vertical(|ui| {
                ui.label(format!("[{}] Visualization Placeholder", variant));
                ui.add(egui::ProgressBar::new(0.7).text("CPU Load"));
                ui.add(egui::ProgressBar::new(0.4).text("Memory"));
            });
        }
    }
}
