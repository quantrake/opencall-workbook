use egui::{TextEdit, Ui};
use serde::{Deserialize, Serialize};

use super::editor::{Editor, BIN_ICON_COLOR};
use super::renderer::Renderer;
use super::visuals::{style_bin_button, style_move_button};

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Notes {
    pub notes: Vec<Note>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Note {
    pub note: String,
    #[serde(skip)]
    pub hint: String,
    pub visible: bool,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            note: Note::new().note,
            hint: String::from("…"),
            visible: true,
        }
    }
}

impl Note {
    pub fn new() -> Self {
        Self {
            note: String::new(),
            hint: String::from("…"),
            visible: true,
        }
    }
}

impl Editor for Notes {
    fn edit(&mut self, ui: &mut Ui, _edit_section_titles: bool) {
        let mut note_to_delete: Option<usize> = None;
        let mut note_to_move: Option<usize> = None;
        for (i, item) in self.notes.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut item.visible, "").on_hover_text(
                        "Any note can be optionally shown/hidden \
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
                            "Click to delete this note entirely. \n\
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
            self.notes.swap(i, i - 1);
        }
        if let Some(i) = note_to_delete {
            self.notes.remove(i);
        }
    }
}

impl Renderer for Notes {
    fn preview(&self, ui: &mut Ui, leading_space: Option<f32>) {
        self.notes.iter().for_each(|note| {
            if note.visible {
                if leading_space.is_some() {
                    ui.add_space(Option::expect(
                        leading_space,
                        "the leading space should be set to Option<f32>, \
                            e.g. Some(10.0) or None",
                    ));
                }
                ui.label(&note.note);
            }
        });
    }
}
