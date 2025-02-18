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

// SectionScope reflects the essence (character, nature) of the contents
// of paragraphs (subsections) of the Scope section, i.e. the structured
// bits of text which, being combined, make up the contents of this section.
// Chapters or paragraphs (subsections), i.e. the bits of text
// which, being combined, make up the contents of the section.
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum SectionScope {
    Activities,
    Objectives,
    SuggestedTasks,
    Tasks,
    WorkPlan,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Scope {
    // `Scope` - Describe 'The Scope of Research' ('Subject Matter')
    pub title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    pub index_list: Vec<Segment>,

    pub suggested_tasks: Notes, // Tasks for the partners as suggested by the participants
    pub objectives: Notes,      // 'Objectives'
    pub activities: Notes,      // 'Activities' – General description of the planned work
    pub work_plan: Notes,
    pub tasks: Notes,
    pub visible: bool,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            title: String::from("Project Scope: Objectives and Planned Activities"),
            index_list: vec![
                Segment {
                    variety: Variety::SectionScope(SectionScope::SuggestedTasks),
                    tier: 2,
                },
                Segment {
                    variety: Variety::SectionScope(SectionScope::Objectives),
                    tier: 3,
                },
                Segment {
                    variety: Variety::SectionScope(SectionScope::Activities),
                    tier: 3,
                },
                Segment {
                    variety: Variety::SectionScope(SectionScope::WorkPlan),
                    tier: 3,
                },
                Segment {
                    variety: Variety::SectionScope(SectionScope::Tasks),
                    tier: 3,
                },
            ],
            suggested_tasks: Notes::default(),
            objectives: Notes::default(),
            activities: Notes::default(),
            work_plan: Notes::default(),
            tasks: Notes::default(),
            visible: false,
        }
    }
}

impl Scope {
    pub fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool, resolution: usize) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_scope");

        ui.add_space(SPACE_SECTIONS_EDITOR);
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.checkbox(&mut self.visible, "");
                ui.toggle_value(
                    &mut self.visible,
                    RichText::new(title).size(TITLE_FONT_SIZE),
                );
                ui.add(style_info_button()).on_hover_text(
                    "Draft supposed project stages or work plan, \
                    e.g. task list, mention existing and required equipment, \
                    facilities, software, etc.",
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
                            Variety::SectionScope(SectionScope::SuggestedTasks) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.label(
                                    "Tasks for the partners as suggested by the participants:",
                                );

                                self.suggested_tasks.edit(ui, edit_section_titles);
                                if ui
                                    .button("Add a note")
                                    .on_hover_text(
                                        "You may add new pieces of text, which then can be \
                                        optionally included in the document",
                                    )
                                    .clicked()
                                {
                                    self.suggested_tasks.notes.push(Note::new());
                                }
                            }
                            Variety::SectionScope(SectionScope::Objectives) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.label("Objectives:");

                                self.objectives.edit(ui, edit_section_titles);
                                if ui
                                    .button("Add a note")
                                    .on_hover_text(
                                        "You may add new pieces of text, which then can be \
                                        optionally included in the document",
                                    )
                                    .clicked()
                                {
                                    self.objectives.notes.push(Note::new());
                                }
                            }
                            Variety::SectionScope(SectionScope::Activities) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.label("Activities – general description of the planned work:");

                                self.activities.edit(ui, edit_section_titles);
                                if ui
                                    .button("Add a note")
                                    .on_hover_text(
                                        "You may add new pieces of text, which then can be \
                                        optionally included in the document",
                                    )
                                    .clicked()
                                {
                                    self.activities.notes.push(Note::new());
                                }
                            }
                            Variety::SectionScope(SectionScope::WorkPlan) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.label("Work Plan:");

                                self.work_plan.edit(ui, edit_section_titles);
                                if ui
                                    .button("Add a note")
                                    .on_hover_text(
                                        "You may add new pieces of text, which then can be \
                                        optionally included in the document",
                                    )
                                    .clicked()
                                {
                                    self.work_plan.notes.push(Note::new());
                                }
                            }
                            Variety::SectionScope(SectionScope::Tasks) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.label("Tasks:");

                                self.tasks.edit(ui, edit_section_titles);
                                if ui
                                    .button("Add a note")
                                    .on_hover_text(
                                        "You may add new pieces of text, which then can be \
                                        optionally included in the document",
                                    )
                                    .clicked()
                                {
                                    self.tasks.notes.push(Note::new());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            });
    }
}

impl Scope {
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
                        Variety::SectionScope(SectionScope::SuggestedTasks) => {
                            self.suggested_tasks
                                .preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionScope(SectionScope::Objectives) => {
                            self.objectives.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionScope(SectionScope::Activities) => {
                            self.activities.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionScope(SectionScope::WorkPlan) => {
                            self.work_plan.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionScope(SectionScope::Tasks) => {
                            self.tasks.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
