use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::chapter::{Segment, Variety};
use crate::workbook::editor::Editor;
use crate::workbook::editor::{SPACE_INTERNAL_EDITOR, SPACE_SECTIONS_EDITOR, TITLE_FONT_SIZE};
use crate::workbook::note::{Note, Notes};
use crate::workbook::renderer::Renderer;
use crate::workbook::renderer::{SPACE_INTERNAL_PREVIEW, SPACE_SECTIONS_PREVIEW};
use crate::workbook::sections::references::References;
use crate::workbook::visuals::style_info_button;

// SectionIdea reflects the essence (character, nature) of the contents
// of paragraphs (subsections) of the Idea section, i.e. the structured
// bits of text which, being combined, make up the contents of this section.
// Chapters or paragraphs (subsections), i.e. the bits of text
// which, being combined, make up the contents of the section.
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum SectionIdea {
    Abstract,
    Hypothesis,
    KeyReferences,
    Problem,
    ProjectDescription,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Idea {
    pub title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    pub index_list: Vec<Segment>,

    pub problem: Notes,         // Introduction – what is the problem to be solved
    pub hypothesis: Notes,      // Main idea, research hypothesis
    pub summary: Notes, // Abstract. Would be useful to add on later stages as general information about the project
    pub description: Notes, // Short description of the project
    pub references: References, // The 'Key References' section is moved here
    pub visible: bool,
}

impl Default for Idea {
    fn default() -> Self {
        Self {
            title: String::from("Idea"),
            index_list: vec![
                Segment {
                    variety: Variety::SectionIdea(SectionIdea::Problem),
                    tier: 1,
                },
                Segment {
                    variety: Variety::SectionIdea(SectionIdea::Hypothesis),
                    tier: 1,
                },
                Segment {
                    variety: Variety::SectionIdea(SectionIdea::Abstract),
                    tier: 3,
                },
                Segment {
                    variety: Variety::SectionIdea(SectionIdea::ProjectDescription),
                    tier: 3,
                },
                Segment {
                    variety: Variety::SectionIdea(SectionIdea::KeyReferences),
                    tier: 1,
                },
            ],
            problem: Notes::default(),
            hypothesis: Notes::default(),
            summary: Notes::default(),
            description: Notes::default(),
            references: References::default(),
            visible: false,
        }
    }
}

impl Idea {
    pub fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool, resolution: usize) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_idea");

        ui.add_space(SPACE_SECTIONS_EDITOR);
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.checkbox(&mut self.visible, "");

                ui.toggle_value(
                    &mut self.visible,
                    RichText::new(title)
                        .size(TITLE_FONT_SIZE),
                );

                ui.add(style_info_button())
                    .on_hover_text("\
                    The purpose of this section is to present your idea of the project.\n\n\
                    \
                    If you want to include any simple figures for illustration, feel free \
                    to mention and list them in your notes here.\n\n\
                    \
                    You can add new notes by clicking the 'Add a note' button. Any note can \
                    be included in your document by ticking the box next to it. If you want \
                    to hide some notes, just untick the \u{2611} (checkbox) next to each one.\n\n\
                    \
                    Tip: You can keep different versions of the same idea, too! If you need \
                    to delete a note, just click the \u{1F5D1} (trash) icon—but remember, \
                    this action can’t be undone!\
                    ");
            })
            .body(|ui| {
                if edit_section_titles {
                    ui.label("You can rename this section:");
                    TextEdit::singleline(&mut self.title)
                        .hint_text("Name this section")
                        .show(ui);
                }

                let chapters = &self.index_list;
                for chapter in chapters {
                    if chapter.tier > resolution { continue; } else {
                        match chapter.variety {
                            Variety::SectionIdea(SectionIdea::Problem) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.label("Introduction – what is the problem to be solved:");

                                self.problem.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new pieces of text, which then can be \
                                    optionally included in the document.\n\n\
                                    \
                                    Any note can be included in your document by ticking the box next to it. \
                                    If you want to hide some notes, just untick the \u{2611} (checkbox) \
                                    next to each one.")
                                    .clicked() {
                                        self.problem.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionIdea(SectionIdea::Hypothesis) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Main idea of the project, research hypothesis:");

                                    let tip = ui.add(style_info_button());
                                    tip.clone().on_hover_text("\
                                    Briefly describe your vision of the project in clear \
                                    and natural form.\n\n\
                                    \
                                    Keep it simple and straightforward—this is your chance \
                                    to convey what you're excited about!\
                                    ");
                                    ui.end_row();
                                });

                                self.hypothesis.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new pieces of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.hypothesis.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionIdea(SectionIdea::Abstract) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.label("Abstract:");

                                self.summary.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new pieces of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.summary.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionIdea(SectionIdea::ProjectDescription) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.label("Short description of the project (general information about the project):");

                                self.description.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new pieces of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.description.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionIdea(SectionIdea::KeyReferences) => {
                                self.references.edit(ui, edit_section_titles);
                            }
                            _ => {}
                        }
                    }
                }
            });
    }
}

impl Idea {
    pub fn preview(&self, ui: &mut egui::Ui, resolution: usize) {
        if self.visible {
            ui.add_space(SPACE_SECTIONS_PREVIEW);
            if self.title.is_empty() {
                ui.heading(Self::default().title);
            } else {
                ui.heading(&self.title);
            }

            let chapters = &self.index_list;
            for chapter in chapters {
                if chapter.tier > resolution {
                    continue;
                } else {
                    match chapter.variety {
                        Variety::SectionIdea(SectionIdea::Problem) => {
                            self.problem.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionIdea(SectionIdea::Hypothesis) => {
                            self.hypothesis.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionIdea(SectionIdea::Abstract) => {
                            self.summary.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionIdea(SectionIdea::ProjectDescription) => {
                            self.description.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionIdea(SectionIdea::KeyReferences) => {
                            self.references.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
