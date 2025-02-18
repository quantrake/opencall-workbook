use egui::Ui;

use super::app::Workbook;
use super::chapter::Variety;

pub const SPACE_INTERNAL_EDITOR: f32 = 10.0;
pub const SPACE_SECTIONS_EDITOR: f32 = 14.0;
pub const TITLE_FONT_SIZE: f32 = 16.0;

pub const BIN_ICON_COLOR: egui::Color32 = egui::Color32::GRAY;

impl Workbook {
    // Reflects the contents of workbook sections.
    pub fn edit(&mut self, ui: &mut Ui) {
        let sections = &mut self.project.index_list;

        for section in sections {
            if section.tier > self.project.resolution {
                continue;
            } else {
                match section.variety {
                    Variety::WorkingName => {
                        self.project.working_name.edit(ui);
                    }
                    Variety::Funding => {
                        self.project.funding.edit(ui, self.edit_section_titles);
                    }
                    Variety::Idea => {
                        self.project.idea.edit(
                            ui,
                            self.edit_section_titles,
                            self.project.resolution,
                        );
                    }
                    Variety::Timeline => {
                        self.project.timeline.edit(
                            ui,
                            self.edit_section_titles,
                            self.project.resolution,
                        );
                    }
                    Variety::Scope => {
                        self.project.scope.edit(
                            ui,
                            self.edit_section_titles,
                            self.project.resolution,
                        );
                    }
                    Variety::PrelimResults => {
                        self.project
                            .prelim_results
                            .edit(ui, self.edit_section_titles);
                    }
                    Variety::Methodology => {
                        self.project.methodology.edit(ui, self.edit_section_titles);
                    }
                    Variety::Team => {
                        self.project.team.edit(
                            ui,
                            self.edit_section_titles,
                            self.project.resolution,
                        );
                    }
                    Variety::Scripting => {
                        self.project.scripting.edit(ui, self.edit_section_titles);
                    }
                    Variety::Literature => {
                        self.project.literature.edit(ui, self.edit_section_titles);
                    }
                    Variety::Outcomes => {
                        self.project.outcomes.edit(
                            ui,
                            self.edit_section_titles,
                            self.project.resolution,
                        );
                    }
                    Variety::Resources => {
                        self.project.resources.edit(ui, self.edit_section_titles);
                    }
                    Variety::Budget => {
                        self.project.budget.edit(
                            ui,
                            self.edit_section_titles,
                            self.project.resolution,
                        );
                    }
                    Variety::Attachments => {
                        // self.project.attachments.edit(ui, self.edit_section_titles);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub trait Editor {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool);
}
