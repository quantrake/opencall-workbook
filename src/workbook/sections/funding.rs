use chrono::Datelike;
use chrono::TimeZone;
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
use crate::workbook::sections::timeline::Date;
use crate::workbook::visuals::{style_bin_button, style_info_button, style_move_button};

#[derive(Deserialize, Serialize, Clone)]
pub struct FundingOptions {
    pub title: String,
    pub funding_options: Vec<Programme>,
    pub visible: bool,
}

impl Default for FundingOptions {
    fn default() -> Self {
        Self {
            title: String::from("Funding Options"),
            funding_options: Vec::new(),
            visible: false,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Programme {
    pub title: String,
    pub hyperlink: String,

    pub deadline: Date,

    pub annotation: Notes,
    pub visible: bool,
}

impl Editor for FundingOptions {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_funding");

        ui.add_space(SPACE_SECTIONS_EDITOR);
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.checkbox(&mut self.visible, "");

                ui.toggle_value(
                    &mut self.visible,
                    RichText::new(title).size(TITLE_FONT_SIZE),
                );
                let tip = ui.add(style_info_button());
                tip.clone().on_hover_text(
                    "\
                This section’s all about giving your partners a quick idea of \
                what this project call is about—and when it’s happening!\n\n\
                \
                Program Calls usually come with big, detailed docs covering everything: \
                goals, requirements, deadlines, participation rules, you name it. \
                But here, it’s all about the essentials—what partners really need \
                to know to decide if it’s worth exploring further.\n\n\
                \
                Keep in mind, most partners won’t have time to dig through the full \
                docs. A simple, clear summary here can help them see if this call \
                could be a great fit for your team.\
                ",
                );

                if ui
                    .button("+")
                    .on_hover_text("Add a funding program")
                    .clicked()
                {
                    self.funding_options.push(Programme::default());
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
                    ui.label(
                        "Short information about available funding programs, suitable Open Calls:",
                    );

                    ui.add(style_info_button()).on_hover_text(
                        "\
                        List available funding options.\n\n\
                        Add options by clicking on the '+' ('Add a funding program') button \
                        in the section header.\
                        ",
                    );
                });

                let mut option_to_delete: Option<usize> = None;
                let mut option_to_move: Option<usize> = None;
                for (i, option) in self.funding_options.iter_mut().enumerate() {
                    let id_src = format!("funding_collapsing_header{}", i + 1);
                    let id = ui.make_persistent_id(id_src);

                    ui.add_space(SPACE_INTERNAL_EDITOR);
                    CollapsingState::load_with_default_open(ui.ctx(), id, true)
                        .show_header(ui, |ui| {
                            ui.checkbox(&mut option.visible, "").on_hover_text(
                                "\
                                Check to show the funding program.\n\
                                Uncheck to hide it in the document.\
                                ",
                            );

                            TextEdit::singleline(&mut option.title)
                                .hint_text("Funding program name")
                                .show(ui);

                            let icon_color = BIN_ICON_COLOR;
                            if i > 0 {
                                // ⬆ Move up in the list
                                if ui
                                    .add(style_move_button(icon_color))
                                    .on_hover_text("Move up in the list")
                                    .clicked()
                                {
                                    option_to_move = Some(i)
                                }
                            }

                            // Remove from the list
                            if ui
                                .add(style_bin_button(icon_color))
                                .on_hover_text(
                                    "\
                                Click to delete this option entirely. \n\
                                ALERT: You cannot undo this action!\
                                ",
                                )
                                .clicked()
                            {
                                option_to_delete = Some(i)
                            }
                        })
                        .body(|ui| {
                            //

                            option.edit(ui, edit_section_titles);
                        });
                }

                if let Some(i) = option_to_move {
                    self.funding_options.swap(i, i - 1);
                }
                if let Some(i) = option_to_delete {
                    self.funding_options.remove(i);
                }
            });
    }
}

impl Editor for Programme {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool) {
        ui.add_space(SPACE_INTERNAL_EDITOR);
        ui.horizontal(|ui| {
            ui.label("Short annotation of the funding program:");

            let tip = ui.add(style_info_button());
            tip.clone().on_hover_text(
                "\
            Add a short informal annotation of the funding program.\n\n\
            \
            This is where you add a short, informal summary of the \
            funding program. This helps potential collaborators quickly understand \
            why this program might be a good fit for them.\n\n\
            \
            It’s a key step! Start by reviewing the available info on potential \
            funding programs. Some programs might look promising but can be tricky \
            to get grants from. If you’re unsure, reach out to the contacts listed \
            in the program docs—they’re there to help answer questions specific to \
            your country.\n\n\
            \
            Providing a clear explanation here can be really valuable. \
            For instance, you could write: \
            “New program launched this year—could be a good opportunity,” \
            or: “Continuation of the big European program that started in 2022, \
            but needs an industrial partner,” \
            or even: “This Call aligns really well with what our team can do.” \
            Remember, this section is just for you and your partners, so keep it \
            informal and helpful.\n\n\
            \
            You can add more notes with the 'Add a note' button. Any note can be \
            included in your final document by ticking the box to the left of it. \
            To hide notes, simply untick the \u{2611} (checkbox) next to each one.\n\n\
            \
            Want to save different versions? Feel free! You can delete a note \
            permanently by clicking the \u{1F5D1} (trash) icon—just keep in mind \
            that this action can’t be undone!\
            ",
            );
        });

        // Program annotation
        self.annotation.edit(ui, edit_section_titles);
        if ui
            .button("Add a note")
            .on_hover_text(
                "You may add new pieces of text, which then can be \
                optionally included in the document",
            )
            .clicked()
        {
            self.annotation.notes.push(Note::new());
        }

        ui.add_space(SPACE_INTERNAL_EDITOR);
        ui.label("Link to the website with all documentation:");
        ui.vertical_centered_justified(|ui| {
            TextEdit::singleline(&mut self.hyperlink)
                .hint_text("https://docs.example.com/program_name")
                .show(ui);
        });
        ui.add_space(SPACE_INTERNAL_EDITOR);
        egui::Grid::new("program_deadline_grid")
            .num_columns(2)
            .spacing([40.0, 14.0])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Application deadline:");
                    ui.add(style_info_button())
                        .on_hover_text("The date is in the Program guide");
                });
                {
                    let year = &mut self.deadline.year;
                    let month = &mut self.deadline.month;
                    let day = &mut self.deadline.day;
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

                    if date == chrono::MappedLocalTime::None {
                        self.deadline.date = chrono::Local
                            .with_ymd_and_hms(*year, *month, *day - 1, 0, 1, 1)
                            .unwrap();
                    } else {
                        self.deadline.date = date.unwrap();
                    }
                }
                ui.end_row();
            });
    }
}

impl Renderer for FundingOptions {
    fn preview(&self, ui: &mut egui::Ui, _leading_space: Option<f32>) {
        if self.visible {
            ui.add_space(SPACE_SECTIONS_PREVIEW);
            if self.title.is_empty() {
                ui.heading(Self::default().title);
            } else {
                ui.heading(&self.title);
            }

            self.funding_options.iter().for_each(|option| {
                if option.visible {
                    ui.add_space(SPACE_SECTIONS_PREVIEW);
                    ui.label(
                        RichText::new(&option.title)
                            .size(SUBSECTION_FONT_SIZE)
                            .color(SUBSECTION_FONT_COLOR),
                    );
                    option.preview(ui, None);
                }
            });
        };
    }
}

impl Renderer for Programme {
    fn preview(&self, ui: &mut egui::Ui, _leading_space: Option<f32>) {
        if self.visible {
            self.annotation.preview(ui, Some(SPACE_INTERNAL_PREVIEW));

            ui.add_space(SPACE_INTERNAL_PREVIEW);
            ui.hyperlink(&self.hyperlink);

            ui.add_space(SPACE_INTERNAL_PREVIEW);
            ui.label(format!(
                "Application deadline: {:}-{:02}-{:02}",
                self.deadline.date.year(),
                self.deadline.date.month(),
                self.deadline.date.day()
            ));
        }
    }
}
