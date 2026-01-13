
use bevy_egui::egui;
use jitui::schema::Jitui;

pub fn render(ui: &mut egui::Ui, jitui: &Jitui) {
    if let Some(title) = &jitui.title {
        ui.label(egui::RichText::new(title).strong().size(16.0));
        ui.separator();
    }
    if let Some(content) = jitui.data.get("content").and_then(|v| v.as_str()) {
        ui.label(content);
    }
}
