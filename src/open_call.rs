/* egui_demo_app egui.rs v0.25.0 | MIT License | https://github.com/emilk/egui */
// https://github.com/emilk/egui/blob/master/crates/egui_demo_app/src/wrap_app.rs

use serde::Deserialize;

#[cfg(target_arch = "wasm32")]
use core::any::Any;

use crate::guide::GuideReader;
use crate::workbook::app::Workbook;

pub const OPENCALL_VERSION: &str = "0.8.2";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Anchor {
    // Applications in the top bar
    GuideApp,
    WorkbookApp,
}

impl Anchor {
    #[cfg(target_arch = "wasm32")]
    fn all() -> Vec<Self> {
        vec![
            // Applications.
            Self::GuideApp,
            Self::WorkbookApp,
        ]
    }
}

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<Anchor> for egui::WidgetText {
    fn from(value: Anchor) -> Self {
        Self::RichText(egui::RichText::new(value.to_string()))
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Self::WorkbookApp
    }
}

// The state that we persist (serialize).
#[derive(Default)]
pub struct State {
    guide: GuideReader,
    workbook: Workbook,
    selected_anchor: Anchor,
}

#[derive(Default)]
pub struct Settings {
    pub customer: CustomerAccount,
}

#[derive(Deserialize, Default)]
pub struct CustomerAccount {
    pub email: String,
    pub id: String,
}

pub struct OpenCall {
    pub state: State,
}

impl OpenCall {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set the zoom factor of the UI when the app starts.
        // Variable by the user with `Cmd +/-` and sets to 1.0 with `Cmd 0`.
        cc.egui_ctx.set_zoom_factor(1.2);

        Self {
            state: State::default(),
        }
    }
}

impl eframe::App for OpenCall {
    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        visuals.panel_fill.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        if let Some(anchor) = frame.info().web_info.location.hash.strip_prefix('#') {
            let anchor = Anchor::all().into_iter().find(|x| x.to_string() == anchor);
            if let Some(v) = anchor {
                self.state.selected_anchor = v;
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            let fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
        }

        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });
        });

        self.show_selected_app(ctx, frame);
    }

    #[cfg(target_arch = "wasm32")]
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(&mut *self)
    }
}

impl OpenCall {
    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, Anchor, &mut dyn eframe::App)> {
        let vec = vec![
            (
                "\u{2139} Guide", // ℹ︎ INFORMATION SOURCE
                Anchor::GuideApp,
                &mut self.state.guide as &mut dyn eframe::App,
            ),
            (
                "📝 Workbook",
                Anchor::WorkbookApp,
                &mut self.state.workbook as &mut dyn eframe::App,
            ),
        ];

        vec.into_iter()
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected_anchor = self.state.selected_anchor;
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory(|mem| mem.everything_is_visible()) {
                app.update(ctx, frame);
            }
        }
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut selected_anchor = self.state.selected_anchor;
        for (name, anchor, _app) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor;
                // // For web demo
                // if frame.is_web() {
                //     ui.ctx()
                //         .open_url(egui::OpenUrl::same_tab(format!("#{anchor}")));
                // }
            }
        }
        self.state.selected_anchor = selected_anchor;

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
            egui::warn_if_debug_build(ui);
        });
    }
}
