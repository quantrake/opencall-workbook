use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::editor::Editor;
use crate::workbook::editor::{SPACE_INTERNAL_EDITOR, SPACE_SECTIONS_EDITOR, TITLE_FONT_SIZE};
use crate::workbook::note::{Note, Notes};
use crate::workbook::renderer::Renderer;
use crate::workbook::renderer::{SPACE_INTERNAL_PREVIEW, SPACE_SECTIONS_PREVIEW};
use crate::workbook::visuals::style_info_button;

#[derive(Deserialize, Serialize, Clone)]
pub struct PrelimResults {
    pub title: String,
    pub prelim_results: Notes,
    pub visible: bool,
}

impl Default for PrelimResults {
    fn default() -> Self {
        Self {
            title: String::from("Preliminary Results"),
            prelim_results: Notes::default(),
            visible: false,
        }
    }
}

impl Editor for PrelimResults {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_results");

        ui.add_space(SPACE_SECTIONS_EDITOR);
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.checkbox(&mut self.visible, "");

                ui.toggle_value(
                    &mut self.visible,
                    RichText::new(title).size(TITLE_FONT_SIZE),
                );

                ui.add(style_info_button()).on_hover_text(
                    "\
                    The purpose of this section is to present preliminary results\n\
                    which make you believe in the success of this project.\
                    ",
                );
            })
            .body(|ui| {
                if edit_section_titles {
                    ui.label("You can rename this section:");
                    TextEdit::singleline(&mut self.title)
                        .hint_text("Name this section")
                        .show(ui);
                }

                ui.add_space(SPACE_INTERNAL_EDITOR);
                ui.horizontal(|ui| {
                    ui.label(
                        "Refer to some preliminary results which may make the project viable:",
                    );

                    ui.add(style_info_button()).on_hover_text(
                        "\
                        Describe preliminary results, e.g. published/unpublished works, \
                        unpublished finished/unfinished works, preliminary calculations \
                        and estimations, etc.\
                        ",
                    );
                });

                self.prelim_results.edit(ui, edit_section_titles);
                if ui
                    .button("Add a note")
                    .on_hover_text(
                        "You may add new pieces of text, which then can be \
                        optionally included in the document",
                    )
                    .clicked()
                {
                    self.prelim_results.notes.push(Note::new());
                }
            });
    }
}

impl Renderer for PrelimResults {
    fn preview(&self, ui: &mut egui::Ui, _leading_space: Option<f32>) {
        if self.visible {
            ui.add_space(SPACE_SECTIONS_PREVIEW);
            if self.title.is_empty() {
                ui.heading(Self::default().title);
            } else {
                ui.heading(&self.title);
            }
            self.prelim_results
                .preview(ui, Some(SPACE_INTERNAL_PREVIEW));
        }
    }
}
