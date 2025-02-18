use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::editor::Editor;
use crate::workbook::editor::{SPACE_SECTIONS_EDITOR, TITLE_FONT_SIZE};
use crate::workbook::note::{Note, Notes};
use crate::workbook::renderer::Renderer;
use crate::workbook::renderer::{SPACE_INTERNAL_PREVIEW, SPACE_SECTIONS_PREVIEW};

#[derive(Deserialize, Serialize, Clone)]
pub struct Methodology {
    pub title: String,
    pub methodology: Notes, // Methodology: research design, methodology and techniques
    pub visible: bool,
}

impl Default for Methodology {
    fn default() -> Self {
        Self {
            title: String::from("Research Methodology"),
            methodology: Notes::default(),
            visible: false,
        }
    }
}

impl Editor for Methodology {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_methodology");

        ui.add_space(SPACE_SECTIONS_EDITOR);
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.checkbox(&mut self.visible, "");

                ui.toggle_value(
                    &mut self.visible,
                    RichText::new(title).size(TITLE_FONT_SIZE),
                );
            })
            .body(|ui| {
                if edit_section_titles {
                    ui.label("You can rename this section:");
                    TextEdit::singleline(&mut self.title)
                        .hint_text("Name this section")
                        .show(ui);
                }

                self.methodology.edit(ui, edit_section_titles);
                if ui
                    .button("Add a note")
                    .on_hover_text(
                        "You may add new versions of text, which then can be \
                    optionally included in the document",
                    )
                    .clicked()
                {
                    self.methodology.notes.push(Note::new());
                }
            });
    }
}

impl Renderer for Methodology {
    fn preview(&self, ui: &mut egui::Ui, _leading_space: Option<f32>) {
        if self.visible {
            ui.add_space(SPACE_SECTIONS_PREVIEW);
            if self.title.is_empty() {
                ui.heading(Self::default().title);
            } else {
                ui.heading(&self.title);
            }

            self.methodology.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
        }
    }
}
