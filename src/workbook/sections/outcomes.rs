use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::chapter::{Segment, Variety};
use crate::workbook::editor::Editor;
use crate::workbook::editor::{SPACE_INTERNAL_EDITOR, SPACE_SECTIONS_EDITOR, TITLE_FONT_SIZE};
use crate::workbook::note::{Note, Notes};
use crate::workbook::renderer::Renderer;
use crate::workbook::renderer::{SPACE_INTERNAL_PREVIEW, SPACE_SECTIONS_PREVIEW};

// SectionOutcomes reflects the essence (character, nature) of the contents
// of paragraphs (subsections) of the Outcomes section, i.e. the structured
// bits of text which, being combined, make up the contents of this section.
// Chapters or paragraphs (subsections), i.e. the bits of text
// which, being combined, make up the contents of the section.
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum SectionOutcomes {
    ExpectedResults,
    Impact,
    Propagation,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Outcomes {
    pub title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    pub index_list: Vec<Segment>,

    pub results: Notes,
    pub impact: Notes,
    pub propagation: Notes,
    pub visible: bool,
}

impl Default for Outcomes {
    fn default() -> Self {
        Self {
            title: String::from("Expected Results, Impact and Dissemination"),
            index_list: vec![
                Segment {
                    variety: Variety::SectionOutcomes(SectionOutcomes::ExpectedResults),
                    tier: 1,
                },
                Segment {
                    variety: Variety::SectionOutcomes(SectionOutcomes::Impact),
                    tier: 1,
                },
                Segment {
                    variety: Variety::SectionOutcomes(SectionOutcomes::Propagation),
                    tier: 4,
                },
            ],
            results: Notes::default(),
            impact: Notes::default(),
            propagation: Notes::default(),
            visible: false,
        }
    }
}

impl Outcomes {
    pub fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool, resolution: usize) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_outcome");

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

                let chapters = &self.index_list;
                for chapter in chapters {
                    if chapter.tier > resolution {
                        continue;
                    } else {
                        match chapter.variety {
                            Variety::SectionOutcomes(SectionOutcomes::ExpectedResults) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Draft expected results and outcomes of the project:");
                                });

                                self.results.edit(ui, edit_section_titles);
                                if ui
                                    .button("Add a note")
                                    .on_hover_text(
                                        "You may add new pieces of text, which then can be \
                                    optionally included in the document",
                                    )
                                    .clicked()
                                {
                                    self.results.notes.push(Note::new());
                                }
                            }
                            Variety::SectionOutcomes(SectionOutcomes::Impact) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("What is the potential impact of the project?");
                                });

                                self.impact.edit(ui, edit_section_titles);
                                if ui
                                    .button("Add a note")
                                    .on_hover_text(
                                        "You may add new pieces of text, which then can be \
                                    optionally included in the document",
                                    )
                                    .clicked()
                                {
                                    self.impact.notes.push(Note::new());
                                }
                            }
                            Variety::SectionOutcomes(SectionOutcomes::Propagation) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("How the project results will be disseminated?");
                                });

                                self.propagation.edit(ui, edit_section_titles);
                                if ui
                                    .button("Add a note")
                                    .on_hover_text(
                                        "You may add new pieces of text, which then can be \
                                    optionally included in the document",
                                    )
                                    .clicked()
                                {
                                    self.propagation.notes.push(Note::new());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            });
    }
}

impl Outcomes {
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
                        Variety::SectionOutcomes(SectionOutcomes::ExpectedResults) => {
                            self.results.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionOutcomes(SectionOutcomes::Impact) => {
                            self.impact.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionOutcomes(SectionOutcomes::Propagation) => {
                            self.propagation.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
