use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::editor::Editor;
use crate::workbook::editor::{SPACE_INTERNAL_EDITOR, SPACE_SECTIONS_EDITOR, TITLE_FONT_SIZE};
use crate::workbook::note::{Note, Notes};
use crate::workbook::renderer::Renderer;
use crate::workbook::renderer::{SPACE_INTERNAL_PREVIEW, SPACE_SECTIONS_PREVIEW};
use crate::workbook::visuals::style_info_button;

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub enum ScriptingOption {
    #[default]
    Myself,
    Jointly,
    Other,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Scripting {
    pub title: String,
    pub answer: ScriptingOption,
    pub scripting: Notes,
    pub visible: bool,
}

impl Default for Scripting {
    fn default() -> Self {
        Self {
            title: String::from("Who Will Write The Project Proposal"),
            answer: ScriptingOption::default(),
            scripting: Notes::default(),
            visible: false,
        }
    }
}

impl Editor for Scripting {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_scripting");

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
                    The purpose of this section is to give your partners a clue\n\
                    who will write the Project Proposal for the application.\
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
                ui.label("Who will write the Project Proposal?");

                ui.radio_value(
                    &mut self.answer,
                    ScriptingOption::Myself,
                    "I can write the Project Proposal myself",
                );
                ui.radio_value(
                    &mut self.answer,
                    ScriptingOption::Jointly,
                    "I offer to write the Project Proposal jointly with someone else",
                );
                ui.radio_value(
                    &mut self.answer,
                    ScriptingOption::Other,
                    "I suggest another person for writing the Project Proposal",
                );

                ui.add_space(SPACE_INTERNAL_EDITOR);
                ui.label("If needed, provide details, e.g. who will write and/or assist:");
                self.scripting.edit(ui, edit_section_titles);
                if ui
                    .button("Add a note")
                    .on_hover_text(
                        "You may add new versions of text, which then can be \
                        optionally included in the document",
                    )
                    .clicked()
                {
                    self.scripting.notes.push(Note::new());
                }
            });
    }
}

impl Renderer for Scripting {
    fn preview(&self, ui: &mut egui::Ui, _leading_space: Option<f32>) {
        if self.visible {
            ui.add_space(SPACE_SECTIONS_PREVIEW);
            if self.title.is_empty() {
                ui.heading(Self::default().title);
            } else {
                ui.heading(&self.title);
            }

            ui.add_space(SPACE_INTERNAL_PREVIEW);
            match self.answer {
                ScriptingOption::Myself => ui.label("I can write the Project Proposal myself."),
                ScriptingOption::Jointly => {
                    ui.label("I offer to write the Project Proposal jointly with someone else.")
                }
                ScriptingOption::Other => {
                    ui.label("I suggest another person for writing the Project Proposal.")
                }
            };

            self.scripting.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
        }
    }
}
