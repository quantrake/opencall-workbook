use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::editor::Editor;
use crate::workbook::editor::{
    BIN_ICON_COLOR, SPACE_INTERNAL_EDITOR, SPACE_SECTIONS_EDITOR, TITLE_FONT_SIZE,
};
use crate::workbook::note::{Note, Notes};
use crate::workbook::renderer::Renderer;
use crate::workbook::renderer::{
    SPACE_INTERNAL_PREVIEW, SPACE_SECTIONS_PREVIEW, SUBSECTION_FONT_COLOR, SUBSECTION_FONT_SIZE,
};
use crate::workbook::sections::references::Reference;
use crate::workbook::visuals::{style_bin_button, style_move_button};

#[derive(Deserialize, Serialize, Clone)]
pub struct Literature {
    pub title: String,
    pub literature_survey: Notes,
    pub references: Vec<Reference>,
    pub visible: bool,
}

impl Default for Literature {
    fn default() -> Self {
        Self {
            title: String::from("Literature Survey"),
            literature_survey: Notes::default(),
            references: Vec::new(),
            visible: false,
        }
    }
}

impl Editor for Literature {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_survey");

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
                    ui.label("Draft a short survey of existing literature (try to be critical):");
                });

                self.literature_survey.edit(ui, edit_section_titles);
                if ui
                    .button("Add a note")
                    .on_hover_text(
                        "You may add new pieces of text, which then can be \
                optionally included in the document",
                    )
                    .clicked()
                {
                    self.literature_survey.notes.push(Note::new());
                }

                ui.add_space(SPACE_INTERNAL_EDITOR);
                ui.horizontal(|ui| {
                    ui.label("Add references to the cited papers:");
                    if ui.button("+").on_hover_text("Add a reference").clicked() {
                        self.references.push(Reference::default());
                    }
                });

                // Cited papers
                let mut source_to_delete: Option<usize> = None;
                let mut source_to_move: Option<usize> = None;
                for (i, reference) in self.references.iter_mut().enumerate() {
                    let id_src = format!("my_collapsing_source{}", i + 1);
                    let id = ui.make_persistent_id(id_src);

                    ui.add_space(SPACE_INTERNAL_EDITOR);
                    CollapsingState::load_with_default_open(ui.ctx(), id, true)
                        .show_header(ui, |ui| {
                            ui.checkbox(&mut reference.visible, "").on_hover_text(
                                "Check to show the reference to this source.\n\
                                Uncheck to hide it in the document.",
                            );

                            TextEdit::singleline(&mut reference.title)
                                .hint_text("Reference title")
                                .show(ui);

                            let icon_color = BIN_ICON_COLOR;
                            if i > 0 {
                                // ⬆ Move up in the list
                                if ui
                                    .add(style_move_button(icon_color))
                                    .on_hover_text("Move up in the list")
                                    .clicked()
                                {
                                    source_to_move = Some(i)
                                }
                            }

                            // Remove from the list
                            if ui
                                .add(style_bin_button(icon_color))
                                .on_hover_text(
                                    "Click to delete this reference entirely. \n\
                                ALERT: You cannot undo this action!",
                                )
                                .clicked()
                            {
                                source_to_delete = Some(i)
                            }
                        })
                        .body(|ui| {
                            ui.label("Link to reference/source:");
                            ui.vertical_centered_justified(|ui| {
                                TextEdit::singleline(&mut reference.hyperlink)
                                    .hint_text("https://links.example.com/source_example")
                                    .show(ui);
                            });

                            reference.edit(ui, edit_section_titles);
                        });
                }

                if let Some(i) = source_to_move {
                    self.references.swap(i, i - 1);
                }
                if let Some(i) = source_to_delete {
                    self.references.remove(i);
                }
            });
    }
}

impl Renderer for Literature {
    fn preview(&self, ui: &mut egui::Ui, _leading_space: Option<f32>) {
        if self.visible {
            ui.add_space(SPACE_SECTIONS_PREVIEW);
            if self.title.is_empty() {
                ui.heading(Self::default().title);
            } else {
                ui.heading(&self.title);
            }

            self.literature_survey
                .preview(ui, Some(SPACE_INTERNAL_PREVIEW));

            if !self.references.is_empty() {
                ui.add_space(SPACE_INTERNAL_PREVIEW);
            }

            for reference in self.references.iter() {
                if reference.visible {
                    ui.add_space(SPACE_INTERNAL_PREVIEW);
                    ui.label(
                        RichText::new(&reference.title)
                            .size(SUBSECTION_FONT_SIZE)
                            .color(SUBSECTION_FONT_COLOR),
                    );
                    if !&reference.hyperlink.is_empty() {
                        ui.hyperlink(&reference.hyperlink);
                    }
                    reference.preview(ui, None);
                }
            }
        }
    }
}
