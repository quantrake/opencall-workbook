#[derive(Default)]
pub struct AboutOpenCall {}

impl AboutOpenCall {
    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("OpenCall")
            .open(open)
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(crate::open_call::OPENCALL_VERSION.to_string());
                    ui.label(format!("Guide: {}", crate::guide::GUIDE_VERSION));
                    ui.label(format!(
                        "Workbook data: {}",
                        crate::workbook::app::DATA_FORMAT_STORAGE
                    ));

                    ui.add_space(14.0);
                    ui.label("Narrative editor for research funding applications");

                    ui.add_space(7.0);
                    ui.hyperlink_to("https://opencallworks.com", "https://opencallworks.com")
                        .on_hover_text("https://opencallworks.com");
                });
            });
    }
}
