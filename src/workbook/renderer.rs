// use chrono::Datelike;
use egui::{Color32, RichText, Ui};

use super::{app::Workbook, chapter::Variety};

pub const SPACE_INTERNAL_PREVIEW: f32 = 10.0;
pub const SPACE_SECTIONS_PREVIEW: f32 = 14.0;
pub const SUBSECTION_FONT_SIZE: f32 = 14.0;
pub const SUBSECTION_FONT_COLOR: Color32 = Color32::DARK_GRAY;

pub trait Renderer {
    fn preview(&self, ui: &mut egui::Ui, leading_space: Option<f32>);
}

impl Renderer for Workbook {
    fn preview(&self, ui: &mut Ui, _leading_space: Option<f32>) {
        let sections = &self.project.index_list;

        let working_name = &self.project.working_name;
        let funding_options = &self.project.funding;
        let idea = &self.project.idea;
        let timeline = &self.project.timeline;
        let scope = &self.project.scope;
        let preliminary_results = &self.project.prelim_results;
        let methodology = &self.project.methodology;
        let team = &self.project.team;
        let scripting = &self.project.scripting;
        let literature = &self.project.literature;
        let expected_results = &self.project.outcomes;
        let resources = &self.project.resources;
        let budget = &self.project.budget;
        // let todo_attachments = &self.project.attachments;

        self.preview_version(ui);
        for section in sections {
            if section.tier > self.project.resolution {
                continue;
            } else {
                match section.variety {
                    Variety::WorkingName => {
                        working_name.preview(ui);
                    }
                    Variety::Funding => {
                        funding_options.preview(ui, None);
                    }
                    Variety::Idea => {
                        idea.preview(ui, self.project.resolution);
                    }
                    Variety::Timeline => {
                        timeline.preview(ui, self.project.resolution);
                    }
                    Variety::Scope => {
                        scope.preview(ui, self.project.resolution);
                    }
                    Variety::PrelimResults => {
                        preliminary_results.preview(ui, None);
                    }
                    Variety::Methodology => {
                        methodology.preview(ui, None);
                    }
                    Variety::Team => {
                        team.preview(ui, self.project.resolution);
                    }
                    Variety::Scripting => {
                        scripting.preview(ui, None);
                    }
                    Variety::Literature => {
                        literature.preview(ui, None);
                    }
                    Variety::Outcomes => {
                        expected_results.preview(ui, self.project.resolution);
                    }
                    Variety::Resources => {
                        resources.preview(ui, None);
                    }
                    Variety::Budget => {
                        budget.preview(ui, self.project.resolution);
                    }
                    Variety::Attachments => {
                        // todo_attachments.preview(ui, None);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Workbook {
    // Project Version
    fn preview_version(&self, ui: &mut Ui) {
        if !self.project.record.is_empty() {
            ui.label(
                RichText::new(&self.project.record)
                    .color(Color32::GRAY)
                    .small(),
            );
            // ui.colored_label(egui::Color32::GRAY, &self.project.record);
        }
    }
}
