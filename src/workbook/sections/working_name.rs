use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::editor::Editor;
use crate::workbook::editor::{SPACE_INTERNAL_EDITOR, TITLE_FONT_SIZE};
use crate::workbook::note::{Note, Notes};
use crate::workbook::renderer::Renderer;

use crate::workbook::visuals::style_info_button;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct ProjectTitle {
    pub title: String,

    pub options: Notes, // Alternative versions of project title
    pub visible: bool,
}

impl ProjectTitle {
    pub fn edit(&mut self, ui: &mut Ui) {
        let id = ui.make_persistent_id("collapsing_header_project_name");
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.checkbox(&mut self.visible, "");

                ui.toggle_value(
                    &mut self.visible,
                    RichText::new("Project Title").size(TITLE_FONT_SIZE),
                );

                let tip = ui.add(style_info_button());
                tip.on_hover_text(
                    "\
                Give this project a descriptive working name.\n\
                You may change the project title later.\n\n\
                At this stage, the focus is on clarity, not on crafting the perfect \
                title. Once the project is more developed, you can come back and \
                choose a name that sounds compelling and really captures your vision.\
                ",
                );
            })
            .body(|ui| {
                ui.label("Working name:");
                ui.vertical_centered_justified(|ui| {
                    TextEdit::singleline(&mut self.title)
                        .hint_text("Give this project a descriptive working name")
                        .show(ui);
                });

                ui.add_space(SPACE_INTERNAL_EDITOR);
                ui.horizontal(|ui| {
                    ui.label("Alternative title versions:");
                });

                self.options.edit(ui, false);
                if ui
                    .button("Add a note")
                    .on_hover_text(
                        "You may add new versions of text, which then can be \
                    optionally included in the document.\n\n\
                    \
                    Any note can be included in your document by ticking the box next to it. \
                    If you want to hide some notes, just untick the \u{2611} (checkbox) \
                    next to each one.",
                    )
                    .clicked()
                {
                    self.options.notes.push(Note::new());
                }
            });
    }
}

impl ProjectTitle {
    pub fn preview(&self, ui: &mut Ui) {
        if self.visible {
            ui.heading(&self.title);
            self.options.preview(ui, None);
        }
    }
}
