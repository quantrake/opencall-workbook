use egui::ScrollArea;
use std::future;
use std::sync::mpsc::{channel, Receiver, Sender};

use super::project::Project;
use super::renderer::Renderer;
use super::store::v_b0004::Store;

// The version of data format used for saving workbook on disk
// for further opening stored files by the desktop app.
pub const DATA_FORMAT_STORAGE: &str = crate::workbook::store::save::DATA_FORMAT_VERSION;

pub struct Workbook {
    pub stored_projects: Vec<Project>,

    pub project: Project,

    pub show_preview: bool,
    pub show_controls: bool,
    pub show_side_panel: PanelSide,

    pub edit_section_titles: bool,

    pub windows: Windows,

    pub file_channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    pub file_buffer: Vec<u8>,
    pub import_state: ImportState,
}

#[derive(PartialEq)]
pub enum PanelSide {
    Left,
    Right,
}

#[derive(Default)]
pub struct Windows {
    pub about: crate::about::AboutOpenCall,
    pub about_open: bool,
}

impl Default for Workbook {
    fn default() -> Self {
        Self {
            stored_projects: Vec::new(),
            project: Project::default(),
            show_preview: false,
            show_controls: true,
            show_side_panel: PanelSide::Left,
            edit_section_titles: false,

            windows: Windows::default(),

            file_channel: channel(),
            file_buffer: Vec::new(),
            import_state: ImportState::Free,
        }
    }
}

impl Store {
    pub fn encode_bincode(&self) -> Vec<u8> {
        let bytes: Vec<u8> = bincode::serialize(&self).unwrap();
        bytes
    }

    pub fn decode_bincode(encoded: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(encoded)
    }
}

#[derive(PartialEq)]
pub enum ImportState {
    Free,
    Busy,
}

impl eframe::App for Workbook {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Save/open workbook
        if let Ok(encoded) = self.file_channel.1.try_recv() {
            self.file_buffer = encoded;
        }

        if !self.file_buffer.is_empty() && self.import_state != ImportState::Free {
            if self.import_state == ImportState::Busy {
                match Store::decode_bincode(&self.file_buffer) {
                    Ok(work) => {
                        self.project = work.into(); // data format version is in the storage struct
                    }
                    Err(e) => {
                        self.convert_or_err(e);
                    }
                }
            }
            self.file_buffer.clear();
        }

        let show_preview = &mut self.show_preview;
        let edit_section_titles = &mut self.edit_section_titles;

        egui::TopBottomPanel::top("control_panel_top_bottom")
            .min_height(0.)
            .show_animated(ctx, true, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {
                            ui.close_menu();
                            self.project.resolution = 1;
                            *show_preview = false;
                            self.project = Project::default();
                        }

                        if ui.button("Open…").clicked() {
                            ui.close_menu();
                            let sender = self.file_channel.0.clone();
                            let task = rfd::AsyncFileDialog::new()
                                .add_filter("Workbook Files", &["w"])
                                .add_filter("All Files", &["*"])
                                .pick_file();
                            execute(async move {
                                let file = task.await;
                                if let Some(file) = file {
                                    let contents = file.read().await;
                                    let _ = sender.send(contents);
                                }
                            });
                            self.import_state = ImportState::Busy;
                        }

                        ui.separator();

                        if ui.button("Make a snapshot").clicked() {
                            use chrono::{Datelike, Timelike};
                            let timestamp = chrono::offset::Local::now().naive_local();
                            // Record the revision by naming it according to the timestamp.
                            let record = format!(
                                "v{:02}{:02}{:02} {:}-{:02}-{:02}",
                                timestamp.hour(),
                                timestamp.minute(),
                                timestamp.second(),
                                timestamp.year(),
                                timestamp.month(),
                                timestamp.day()
                            );
                            let mut proj = self.project.clone();
                            proj.record = record;
                            self.stored_projects.append(&mut vec![proj]);
                        }

                        ui.menu_button("Snapshots", |ui| {
                            egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {
                                if self.stored_projects.is_empty() {
                                    ui.label("…no snapshots found");
                                    return;
                                }

                                ui.spacing_mut().item_spacing.x = 3.;
                                let mut proj_to_delete = None;
                                for i in (0..self.stored_projects.len()).rev() {
                                    ui.horizontal(|ui| {
                                        if ui.button("↺").clicked() {
                                            self.project = self.stored_projects[i].clone();
                                            ui.close_menu();
                                        }
                                        ui.add(
                                            egui::TextEdit::singleline(
                                                &mut self.stored_projects[i].record,
                                            )
                                            .desired_width(120.),
                                        );
                                        // ⬆
                                        if ui.button("⬆").clicked()
                                            && i + 1 < self.stored_projects.len()
                                        {
                                            self.stored_projects.swap(i, i + 1);
                                        }
                                        if ui.button("🗑").clicked() {
                                            proj_to_delete = Some(i);
                                        }
                                    });
                                }
                                if let Some(i) = proj_to_delete {
                                    self.stored_projects.remove(i);
                                }
                            });
                        });

                        ui.separator();

                        if ui.button("Save…").clicked() {
                            ui.close_menu();
                            let task = rfd::AsyncFileDialog::new()
                                .add_filter("Workbook Files", &["w"])
                                .add_filter("All Files", &["*"])
                                .set_file_name(format!("{}.w", self.project.working_name.title))
                                .save_file();
                            let work: Store = self.project.clone().into();
                            let contents = work.encode_bincode();
                            execute(async move {
                                let file = task.await;
                                if let Some(file) = file {
                                    _ = file.write(&contents).await;
                                }
                            });
                        }
                    });

                    ui.menu_button("View", |ui| {
                        if ui.checkbox(show_preview, "Preview").changed() {
                            ui.close_menu()
                        };

                        ui.horizontal(|ui| {
                            ui.radio_value(&mut self.show_side_panel, PanelSide::Left, "Left");
                            ui.radio_value(&mut self.show_side_panel, PanelSide::Right, "Right");
                        });

                        ui.separator();

                        if ui
                            .checkbox(&mut self.show_controls, "Show Controls")
                            .changed()
                        {
                            // ui.close_menu()
                        };
                        if ui
                            .checkbox(edit_section_titles, "Edit Section Titles")
                            .changed()
                        {
                            // ui.close_menu()
                        };
                    });

                    ui.menu_button("About", |ui| {
                        ui.menu_button("Version", |ui| {
                            ui.label(crate::open_call::OPENCALL_VERSION.to_string());
                            if ui.button("Show More").clicked() {
                                ui.close_menu();
                                self.windows.about_open = true;
                            }
                        });
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.show_controls {
                            ui.label("+");
                            let slider_detail_level =
                                egui::Slider::new(&mut self.project.resolution, 1..=4);
                            let _ = ui.add(slider_detail_level.show_value(false).text("–"));
                            ui.label("Details:");
                            ui.toggle_value(show_preview, "\u{1F441} Preview"); // 👁️ EYE
                        }
                    });
                });
            });

        self.editor_gui(ctx);
        self.show_windows(ctx);
    }
}

impl Workbook {
    fn editor_gui(&mut self, ctx: &egui::Context) {
        match self.show_side_panel {
            PanelSide::Left => self.show_preview_left(ctx),
            PanelSide::Right => self.show_preview_right(ctx),
        }
    }

    // *******************************************************
    // *** Preview - Left SidePanel
    // *** Editor  - CentralPanel (must be always added last)
    // *******************************************************
    fn show_preview_left(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("workbook_preview_panel_left").show_animated(
            ctx,
            self.show_preview,
            |ui| {
                self.preview_panel(ui);
            },
        );
        // CentralPanel must be always added last.
        self.central_panel_edit(ctx);
    }

    // *******************************************************
    // *** Preview - Right SidePanel
    // *** Editor  - CentralPanel (must be always added last)
    // *******************************************************
    fn show_preview_right(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("workbook_preview_panel_right")
            .max_width(600.)
            .show_animated(ctx, self.show_preview, |ui| {
                self.preview_panel(ui);
            });
        // CentralPanel must be always added last.
        self.central_panel_edit(ctx);
    }

    fn preview_panel(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical()
            .id_source("preview_scroll")
            .show(ui, |ui| {
                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::remainder())
                    .vertical(|mut strip| {
                        strip.strip(|builder| {
                            builder
                                .size(egui_extras::Size::remainder())
                                .size(
                                    egui_extras::Size::relative(0.85)
                                        .at_least(360.0)
                                        .at_most(750.0),
                                )
                                .size(egui_extras::Size::remainder())
                                .horizontal(|mut strip| {
                                    strip.empty();
                                    strip.cell(|ui| {
                                        ui.vertical(|ui| {
                                            self.preview(ui, None);
                                        });
                                    });
                                    strip.empty();
                                });
                        });
                    });
            });
    }

    fn central_panel_edit(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .id_source("outline_scroll")
                .show(ui, |ui| {
                    egui_extras::StripBuilder::new(ui)
                        .size(egui_extras::Size::remainder())
                        .vertical(|mut strip| {
                            strip.strip(|builder| {
                                builder
                                    .size(egui_extras::Size::relative(0.9))
                                    .size(egui_extras::Size::remainder())
                                    .horizontal(|mut strip| {
                                        strip.cell(|ui| {
                                            ui.vertical(|ui| {
                                                self.edit(ui);
                                            });
                                        });
                                        strip.empty();
                                    });
                            });
                        });
                });
        });
    }

    // Show open windows.
    fn show_windows(&mut self, ctx: &egui::Context) {
        self.windows.about.show(ctx, &mut self.windows.about_open);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: future::Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
