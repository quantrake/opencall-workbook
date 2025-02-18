use chrono::Datelike;
use chrono::TimeZone;
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

// SectionTimeline reflects the essence (character, nature) of the contents
// of paragraphs (subsections) of the Timeline section, i.e. the structured
// bits of text which, being combined, make up the contents of this section.
// Chapters or paragraphs (subsections), i.e. the bits of text
// which, being combined, make up the contents of the section.
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum SectionTimeline {
    Milestones,
    ProjectTiming,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Timeline {
    pub title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    pub index_list: Vec<Segment>,

    pub project_start: Date,
    pub duration_years: f32,
    pub milestones: Notes,
    pub visible: bool,
}

impl Default for Timeline {
    fn default() -> Self {
        Self {
            title: String::from("Timeline"),
            index_list: vec![
                Segment {
                    variety: Variety::SectionTimeline(SectionTimeline::ProjectTiming),
                    tier: 1,
                },
                Segment {
                    variety: Variety::SectionTimeline(SectionTimeline::Milestones),
                    tier: 4,
                },
            ],
            project_start: Date::default(),
            duration_years: 0.0,
            milestones: Notes::default(),
            visible: false,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub date: chrono::DateTime<chrono::Local>,
}

impl Default for Date {
    fn default() -> Self {
        Self {
            year: chrono::Local::now().year(),
            month: chrono::Local::now().month(),
            day: chrono::Local::now().day(),
            date: chrono::Local::now(),
        }
    }
}

impl Timeline {
    pub fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool, resolution: usize) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_timeline");

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
                    .on_hover_text("Assess the time required for performing \
                    each part of the project. Develop a realistic timeline \
                    that outlines the major milestones and activities \
                    of the project");
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
                            Variety::SectionTimeline(SectionTimeline::ProjectTiming) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                egui::Grid::new("timeline_dates_grid")
                                    .num_columns(2)
                                    .spacing([40.0, 14.0])
                                    .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("Proposed project start:");
                                        ui.add(style_info_button())
                                            .on_hover_text("\
                                            Take a moment to estimate when you and your partners could \
                                            realistically begin. Not everyone needs to start on the same \
                                            day, but the start date for some partners may be crucial for \
                                            the project’s timeline.");
                                    });

                                    {
                                        let year = &mut self.project_start.year;
                                        let month = &mut self.project_start.month;
                                        let day = &mut self.project_start.day;
                                        let date = chrono::Local.with_ymd_and_hms(*year, *month, *day, 0, 1, 1);
                                        let mut max_day: u32 = 31;

                                        ui.horizontal(|ui| {
                                            ui.add(egui::DragValue::new(year).clamp_range(2024..=2050));
                                            ui.add(egui::DragValue::new(month).clamp_range(1..=12));
                                            if *month == 2 {
                                                if date == chrono::MappedLocalTime::None {
                                                    max_day = 28;
                                                } else {
                                                    max_day = 29;
                                                }
                                            } else if date == chrono::MappedLocalTime::None {
                                                max_day = 30;
                                            }
                                            ui.add(egui::DragValue::new(day).clamp_range(1..=max_day));
                                        });

                                        let date = chrono::Local.with_ymd_and_hms(*year, *month, *day, 0, 1, 1);
                                        if date == chrono::MappedLocalTime::None {
                                            self.project_start.date = chrono::Local::now();
                                        } else {
                                            self.project_start.date = date.unwrap();
                                        }
                                    }
                                    ui.end_row();

                                    ui.horizontal(|ui| {
                                        ui.label("Proposed project duration:");
                                        ui.add(style_info_button())
                                            .on_hover_text("\
                                            Enter the proposed project duration in years. If you're planning \
                                            to involve PhD students, consider that their work typically \
                                            spans over 2 years, so a minimum duration of 3 years might be \
                                            ideal.");
                                    });
                                    ui.horizontal(|ui| {
                                        let field_project_duration = egui::DragValue::new(&mut self.duration_years)
                                            .clamp_range(0.0..=10.0);
                                        let _ = ui.add(field_project_duration) | if self.duration_years == 1. {
                                            ui.label("year")
                                        } else {
                                            ui.label("years")
                                        };
                                    });
                                    ui.end_row();
                                });
                            }
                            Variety::SectionTimeline(SectionTimeline::Milestones) => {
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Milestones:");
                                });

                                self.milestones.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.milestones.notes.push(Note::new());
                                    }
                            }
                            _ => {}
                        }
                    }
                }
            });
    }
}

impl Timeline {
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
                        Variety::SectionTimeline(SectionTimeline::ProjectTiming) => {
                            ui.label(format!(
                                "Proposed project start: {:}-{:02}-{:02}",
                                self.project_start.date.year(),
                                self.project_start.date.month(),
                                self.project_start.date.day()
                            ));

                            let duration = self.duration_years;
                            if duration == 1. {
                                ui.label(format!("Proposed project duration: {} year", duration))
                            } else {
                                ui.label(format!("Proposed project duration: {} years", duration))
                            };
                        }
                        Variety::SectionTimeline(SectionTimeline::Milestones) => {
                            self.milestones.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
