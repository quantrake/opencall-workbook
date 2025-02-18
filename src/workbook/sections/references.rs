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
use crate::workbook::visuals::{style_bin_button, style_info_button, style_move_button};

#[derive(Deserialize, Serialize, Clone)]
pub struct References {
    pub title: String,
    pub references: Vec<Reference>,
    pub visible: bool,
}

impl Default for References {
    fn default() -> Self {
        Self {
            title: String::from("Key References"),
            references: Vec::new(),
            visible: false,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Reference {
    pub title: String,
    pub hyperlink: String,
    pub source_details: Notes,
    pub visible: bool,
}

impl Editor for References {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_references");

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
                    The purpose of this section is to suggest a short list of\n\
                    key sources supporting your application.\
                    ",
                );

                if ui.button("+").on_hover_text("Add a reference").clicked() {
                    self.references.push(Reference::default());
                }
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
                    ui.label("Suggest a very short list of key references:");

                    ui.add(style_info_button()).on_hover_text(
                        "\
                        Offer 2-3 key references.\n\n\
                        Add sources by clicking on the '+' ('Add a reference') button \
                        in the section header.\
                        ",
                    );
                });

                let mut source_to_delete: Option<usize> = None;
                let mut source_to_move: Option<usize> = None;
                for (i, reference) in self.references.iter_mut().enumerate() {
                    let id_src = format!("my_collapsing_header{}", i + 1);
                    let id = ui.make_persistent_id(id_src);

                    ui.add_space(SPACE_INTERNAL_EDITOR);
                    CollapsingState::load_with_default_open(ui.ctx(), id, true)
                        .show_header(ui, |ui| {
                            ui.checkbox(&mut reference.visible, "").on_hover_text(
                                "\
                                Check to show the reference to this source.\n\
                                Uncheck to hide it in the document.\
                                ",
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
                                    "\
                                Click to delete this reference entirely. \n\
                                ALERT: You cannot undo this action!\
                                ",
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

impl Editor for Reference {
    fn edit(&mut self, ui: &mut Ui, _edit_section_titles: bool) {
        let mut note_to_delete: Option<usize> = None;
        let mut note_to_move: Option<usize> = None;
        for (i, item) in self.source_details.notes.iter_mut().enumerate() {
            if i < 1 {
                ui.horizontal(|ui| {
                    ui.label("Source details:");

                    ui.add(style_info_button()).on_hover_text(
                        "Provide details for the reference \
                        to this source",
                    );
                });
            }

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut item.visible, "").on_hover_text(
                        "Any details can be optionally shown/hidden \
                        in the document by adding/removing a tick mark in this \
                        field",
                    );

                    let icon_color = BIN_ICON_COLOR;
                    if i > 0 {
                        // ⬆ Move up in the list
                        if ui
                            .add(style_move_button(icon_color))
                            .on_hover_text("Move up in the list")
                            .clicked()
                        {
                            note_to_move = Some(i)
                        }
                    }

                    // Remove from the list
                    if ui
                        .add(style_bin_button(icon_color))
                        .on_hover_text(
                            "Click to delete this piece of information \
                        entirely. \n\
                        ALERT: You cannot undo this action!",
                        )
                        .clicked()
                    {
                        note_to_delete = Some(i)
                    }
                });
                ui.vertical_centered_justified(|ui| {
                    TextEdit::multiline(&mut item.note)
                        .hint_text(&item.hint)
                        .show(ui);
                });
            });
        }

        if let Some(i) = note_to_move {
            self.source_details.notes.swap(i, i - 1);
        }
        if let Some(i) = note_to_delete {
            self.source_details.notes.remove(i);
        }

        if ui
            .button("Add a note")
            .on_hover_text(
                "You may add new pieces of information, which then can \
                be optionally included in the document",
            )
            .clicked()
        {
            self.source_details.notes.push(Note::new());
        }
    }
}

impl Renderer for References {
    fn preview(&self, ui: &mut egui::Ui, _leading_space: Option<f32>) {
        if self.visible {
            ui.add_space(SPACE_SECTIONS_PREVIEW);
            if self.title.is_empty() {
                ui.heading(Self::default().title);
            } else {
                ui.heading(&self.title);
            }

            self.references.iter().for_each(|reference| {
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
            });
        }
    }
}

impl Renderer for Reference {
    fn preview(&self, ui: &mut egui::Ui, _leading_space: Option<f32>) {
        self.source_details.notes.iter().for_each(|note| {
            if note.visible {
                ui.label(&note.note);
            }
        });
    }
}
