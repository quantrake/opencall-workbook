use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::chapter::{Segment, Variety};
use crate::workbook::editor::Editor;
use crate::workbook::editor::{SPACE_INTERNAL_EDITOR, SPACE_SECTIONS_EDITOR, TITLE_FONT_SIZE};
use crate::workbook::note::{Note, Notes};
use crate::workbook::renderer::Renderer;
use crate::workbook::renderer::{SPACE_INTERNAL_PREVIEW, SPACE_SECTIONS_PREVIEW};
use crate::workbook::visuals::style_info_button;

// SectionBudget reflects the essence (character, nature) of the contents
// of paragraphs (subsections) of the Budget section, i.e. the structured
// bits of text which, being combined, make up the contents of this section.
// Chapters or paragraphs (subsections), i.e. the bits of text
// which, being combined, make up the contents of the section.
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum SectionBudget {
    Facilities,
    Materials,
    Miscellaneous,
    Overheads,
    Personnel,
    Workshops,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Budget {
    pub title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    pub index_list: Vec<Segment>,

    pub personnel: Notes, // Personnel (incl. stipends for PhD students and postdocs)
    pub facilities: Notes, // Facilities, Equipment
    pub materials: Notes, // Materials, software, publications, conference fee and travel expenses
    pub workshops: Notes, // Organization of meetings and workshops
    pub overheads: Notes, // Overheads
    pub misc: Notes,      // Miscellaneous
    pub visible: bool,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            title: String::from("Budget Estimates"),
            index_list: vec![
                Segment {
                    variety: Variety::SectionBudget(SectionBudget::Personnel),
                    tier: 4,
                },
                Segment {
                    variety: Variety::SectionBudget(SectionBudget::Facilities),
                    tier: 4,
                },
                Segment {
                    variety: Variety::SectionBudget(SectionBudget::Materials),
                    tier: 4,
                },
                Segment {
                    variety: Variety::SectionBudget(SectionBudget::Workshops),
                    tier: 4,
                },
                Segment {
                    variety: Variety::SectionBudget(SectionBudget::Overheads),
                    tier: 4,
                },
                Segment {
                    variety: Variety::SectionBudget(SectionBudget::Miscellaneous),
                    tier: 4,
                },
            ],
            personnel: Notes::default(),
            facilities: Notes::default(),
            materials: Notes::default(),
            workshops: Notes::default(),
            overheads: Notes::default(),
            misc: Notes::default(),
            visible: false,
        }
    }
}

impl Budget {
    pub fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool, resolution: usize) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_budget");

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
                    .on_hover_text("Describe budget estimates, for each partner team and for the whole project");
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
                            Variety::SectionBudget(SectionBudget::Personnel) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Personnel expenses, incl. stipends for PhD students and postdocs:");
                                });

                                self.personnel.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.personnel.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionBudget(SectionBudget::Facilities) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Facilities and Equipment:");
                                });

                                self.facilities.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.facilities.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionBudget(SectionBudget::Materials) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Materials, software, publications, conference fee and travel expenses:");
                                });

                                self.materials.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.materials.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionBudget(SectionBudget::Workshops) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Organization of meetings and workshops:");
                                });

                                self.workshops.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.workshops.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionBudget(SectionBudget::Overheads) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Overheads:");
                                });

                                self.overheads.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.overheads.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionBudget(SectionBudget::Miscellaneous) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Miscellaneous:");
                                });

                                self.misc.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.misc.notes.push(Note::new());
                                    }
                            }
                            _ => {}
                        }
                    }
                }
            });
    }
}

impl Budget {
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
                        Variety::SectionBudget(SectionBudget::Personnel) => {
                            self.personnel.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionBudget(SectionBudget::Facilities) => {
                            self.facilities.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionBudget(SectionBudget::Materials) => {
                            self.materials.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionBudget(SectionBudget::Workshops) => {
                            self.workshops.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionBudget(SectionBudget::Overheads) => {
                            self.overheads.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionBudget(SectionBudget::Miscellaneous) => {
                            self.misc.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
