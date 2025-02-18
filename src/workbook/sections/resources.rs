use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::editor::Editor;
use crate::workbook::editor::{SPACE_INTERNAL_EDITOR, SPACE_SECTIONS_EDITOR, TITLE_FONT_SIZE};
use crate::workbook::note::{Note, Notes};
use crate::workbook::renderer::Renderer;
use crate::workbook::renderer::{SPACE_INTERNAL_PREVIEW, SPACE_SECTIONS_PREVIEW};

#[derive(Deserialize, Serialize, Clone)]
pub struct Resources {
    pub title: String,
    pub existing: Notes, // Resources, available
    pub further: Notes,  // Resources, required (extra)
    pub visible: bool,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            title: String::from("Resources"),
            existing: Notes::default(),
            further: Notes::default(),
            visible: false,
        }
    }
}

impl Editor for Resources {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_resources");

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

                ui.add_space(SPACE_INTERNAL_EDITOR);
                ui.horizontal(|ui| {
                    ui.label("Existing resources, e.g. equipment, computation clusters, etc.:");
                });

                self.existing.edit(ui, edit_section_titles);
                if ui
                    .button("Add a note")
                    .on_hover_text(
                        "You may add new versions of text, which then can be \
                        optionally included in the document",
                    )
                    .clicked()
                {
                    self.existing.notes.push(Note::new());
                }

                ui.add_space(SPACE_INTERNAL_EDITOR);
                ui.horizontal(|ui| {
                    ui.label("Additional resources, e.g. facilities, equipment, etc.:");
                });

                self.further.edit(ui, edit_section_titles);
                if ui
                    .button("Add a note")
                    .on_hover_text(
                        "You may add new versions of text, which then can be \
                        optionally included in the document",
                    )
                    .clicked()
                {
                    self.further.notes.push(Note::new());
                }
            });
    }
}

impl Renderer for Resources {
    fn preview(&self, ui: &mut egui::Ui, _leading_space: Option<f32>) {
        if self.visible {
            ui.add_space(SPACE_SECTIONS_PREVIEW);
            if self.title.is_empty() {
                ui.heading(Self::default().title);
            } else {
                ui.heading(&self.title);
            }

            self.existing.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
            self.further.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
        }
    }
}
