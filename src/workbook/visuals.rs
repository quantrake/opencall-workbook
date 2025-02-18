use egui::{Button, Color32, RichText};

pub fn style_info_button() -> Button<'static> {
    Button::new(
        RichText::new("\u{2139}") // ℹ︎ INFORMATION SOURCE
            .color(Color32::GRAY),
    )
    .frame(false)
}

pub fn style_bin_button(color: Color32) -> Button<'static> {
    Button::new(
        egui::RichText::new("\u{1F5D1}") // 🗑️ WASTEBASKET
            .color(color),
    )
    .frame(false)
}

pub fn style_move_button(color: Color32) -> Button<'static> {
    Button::new(egui::RichText::new("⬆").color(color))
}
