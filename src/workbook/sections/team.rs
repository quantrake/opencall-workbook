use egui::collapsing_header::CollapsingState;
use egui::{RichText, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::workbook::chapter::{Segment, Variety};
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

// SectionTeam reflects the essence (character, nature) of the contents
// of paragraphs (subsections) of the Team section, i.e. the structured
// bits of text which, being combined, make up the contents of this section.
// Chapters or paragraphs (subsections), i.e. the bits of text
// which, being combined, make up the contents of the section.
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum SectionTeam {
    IndustrialPartners,
    Participants,
    ProjectLeader,
    Proponents,
    ProposedPartners,

    // Extension for subsections.
    // Note: It is in a separate enum (extension).
    ModuleParticipant(ModuleParticipant),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Team {
    pub title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    pub index_list: Vec<Segment>,

    pub proposed_partners: Notes, // Suggestion of the project team and possible partners
    pub project_leader: Notes,    // Suggestion of the project leader
    pub industrial_partners: Notes, // Possible industrial partners (if necessary)
    pub proponents: Notes,        // Suggestion for cooperation with non-participants of the project

    pub participants: Vec<Participant>, // Full information about participants, institutions, teams

    pub visible: bool,
}

impl Default for Team {
    fn default() -> Self {
        Self {
            title: String::from("Team"),
            index_list: vec![
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::ProposedPartners),
                    tier: 2,
                },
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::ProjectLeader),
                    tier: 2,
                },
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::IndustrialPartners),
                    tier: 2,
                },
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::Proponents),
                    tier: 2,
                },
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::Participants),
                    tier: 3,
                },
            ],
            proposed_partners: Notes::default(),
            project_leader: Notes::default(),
            industrial_partners: Notes::default(),
            proponents: Notes::default(),

            participants: Vec::new(),

            visible: false,
        }
    }
}

// ModuleParticipant reflects the essence (character, nature) of the contents
// of paragraphs (subsections) of the Participant module, i.e. the structured
// bits of text which, being combined, make up the contents of this subsection.
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum ModuleParticipant {
    // Name, // The Participant's full name is the title of subsection

    // // Note: The following `Details` cannot be moved inside the subsection.
    // Affiliation,
    // Country,
    // Expertise,
    // Hyperlink,
    // Role,
    Budget,
    Contribution,
    CV,
    Resources,
    Team,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Participant {
    // The ordered list of chapters (paragraphs).
    // This vector allows to order the paragraphs of this section as required.
    pub index_list: Vec<Segment>,
    pub name: String,
    pub role: String,
    pub affiliation: String,
    pub hyperlink: String,
    pub country: String,
    pub expertise: String,
    pub contribution: Notes,
    pub team: Notes,
    pub cv: Notes, // The short CV of the Participant (Partner's team leader)
    pub resources: Notes,
    pub budget: Notes,
    pub visible: bool,
}

impl Default for Participant {
    fn default() -> Self {
        Self {
            index_list: vec![
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Contribution,
                    )),
                    tier: 3,
                },
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Team,
                    )),
                    tier: 3,
                },
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::CV,
                    )),
                    tier: 3,
                },
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Resources,
                    )),
                    tier: 3,
                },
                Segment {
                    variety: Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Budget,
                    )),
                    tier: 3,
                },
            ],

            name: String::new(),
            role: String::new(),
            affiliation: String::new(),
            hyperlink: String::new(),
            country: String::new(),
            expertise: String::new(),
            contribution: Notes::default(),
            team: Notes::default(),
            cv: Notes::default(), // The short CV of the Participant (Partner's team leader)
            resources: Notes::default(),
            budget: Notes::default(),
            visible: false,
        }
    }
}

impl Team {
    pub fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool, resolution: usize) {
        let mut title = self.title.clone();
        if title.is_empty() {
            title = Self::default().title;
        }
        let id = ui.make_persistent_id("collapsing_header_team");

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
                    The purpose of this section is to tell and show\n\
                    how the suggested contributors to the project\n\
                    can compose a balanced and effective team\n\
                    to succeed in the project.\
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
                            Variety::SectionTeam(SectionTeam::ProposedPartners) => {
                                // Suggestion of the project team and possible partners
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Suggestion of the project team and possible partners:");
                                });
                                self.proposed_partners.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.proposed_partners.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionTeam(SectionTeam::ProjectLeader) => {
                                // Suggestion of the project leader
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Suggestion of the project leader:");
                                });
                                self.project_leader.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.project_leader.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionTeam(SectionTeam::IndustrialPartners) => {
                                // Possible Industrial Partners
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Possible industrial partners:");

                                    ui.add(style_info_button())
                                        .on_hover_text("Possible industrial partners (if necessary)");
                                });
                                self.industrial_partners.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.industrial_partners.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionTeam(SectionTeam::Proponents) => {
                                // Possible Cooperation (proponents who can support and cooperate but not participate)
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Suggestion for cooperation with non-participants:");

                                    ui.add(style_info_button())
                                        .on_hover_text("Suggestion for cooperation with non-participants \
                                        of the project who can support and cooperate but won’t participate \
                                        in the project");
                                });
                                self.proponents.edit(ui, edit_section_titles);
                                if ui.button("Add a note")
                                    .on_hover_text("You may add new versions of text, which then can be \
                                    optionally included in the document")
                                    .clicked() {
                                        self.proponents.notes.push(Note::new());
                                    }
                            }
                            Variety::SectionTeam(SectionTeam::Participants) => {
                                // Full information about participants, institutions, teams
                                ui.add_space(SPACE_INTERNAL_EDITOR);
                                ui.horizontal(|ui| {
                                    ui.label("Full information about participants, institutions and their teams:");

                                    ui.add(style_info_button())
                                        .on_hover_text("\
                                        Suggest a list of project participants\n\
                                        and mention the contribution expected from each participant.\n\n\
                                        Add participants by clicking on '+' ('Add a member') in the header.\
                                        ");

                                    if ui.button("+").on_hover_text("Add a member").clicked() {
                                        self.participants.push(Participant::default());
                                    }
                                });

                                // Edit participants in the project.
                                let mut member_to_delete: Option<usize> = None;
                                let mut member_to_move: Option<usize> = None;
                                for (i, participant) in self
                                    .participants
                                    .iter_mut()
                                    .enumerate()
                                {
                                    let id_src = format!("my_collapsing_header{}", i + 1);
                                    let id = ui.make_persistent_id(id_src);

                                    ui.add_space(SPACE_INTERNAL_EDITOR);
                                    CollapsingState::load_with_default_open(ui.ctx(), id, true)
                                        .show_header(ui, |ui| {
                                            ui.checkbox(&mut participant.visible, "")
                                                .on_hover_text("\
                                                Any participant can be optionally shown/hidden in the document \
                                                by adding/removing a tick mark in this field\
                                                ");

                                            TextEdit::singleline(&mut participant.name)
                                                .hint_text("The name of the leader of partner team")
                                                .show(ui);

                                            if i > 0 {
                                                let icon_color = BIN_ICON_COLOR;

                                                // ⬆ Move up in the list
                                                if ui.add(style_move_button(icon_color))
                                                    .on_hover_text("Move up in the list")
                                                    .clicked() { member_to_move = Some(i) }

                                                // Remove from the list
                                                if ui.add(style_bin_button(icon_color))
                                                    .on_hover_text("\
                                                    Click to remove this participant entirely. \n\
                                                    ALERT: You cannot undo this action!\
                                                    ")
                                                    .clicked() { member_to_delete = Some(i) }
                                            }
                                        })
                                        .body(|ui| {
                                            participant.edit(ui, edit_section_titles, resolution);
                                        });
                                }

                                if let Some(i) = member_to_move {
                                    self.participants.swap(i, i - 1);
                                }
                                if let Some(i) = member_to_delete {
                                    self.participants.remove(i);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            });
    }
}

impl Participant {
    fn edit(&mut self, ui: &mut Ui, edit_section_titles: bool, resolution: usize) {
        let chapters = &self.index_list;
        TextEdit::singleline(&mut self.role)
            .hint_text("Role, e.g. Principal Investigator or Project Leader")
            .show(ui);

        TextEdit::singleline(&mut self.affiliation)
            .hint_text("Affiliation")
            .show(ui);
        TextEdit::singleline(&mut self.hyperlink)
            .hint_text("Web address")
            .show(ui);
        TextEdit::singleline(&mut self.country)
            .hint_text("Country")
            .show(ui);
        TextEdit::singleline(&mut self.expertise)
            .hint_text("Main expertise")
            .show(ui);

        for chapter in chapters {
            if chapter.tier > resolution {
                continue;
            } else {
                match chapter.variety {
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Budget,
                    )) => {
                        ui.add_space(SPACE_INTERNAL_EDITOR);
                        ui.horizontal(|ui| {
                            ui.label("Partner's budget requirements and estimates:");

                            ui.add(style_info_button()).on_hover_text(
                                "Some rough estimations of budget \
                                for the partner's team: \
                                stipends, equipment, computers, travel, materials, \
                                publications, organization of meetings and workshops, etc.",
                            );
                        });
                        self.budget.edit(ui, edit_section_titles);
                        if ui
                            .button("Add a note")
                            .on_hover_text(
                                "You may add new versions of text, which then can be \
                            optionally included in the document",
                            )
                            .clicked()
                        {
                            self.budget.notes.push(Note::new());
                        }
                    }
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Contribution,
                    )) => {
                        ui.add_space(SPACE_INTERNAL_EDITOR);
                        ui.horizontal(|ui| {
                            ui.label("Partner's proposed contribution to the project:");

                            ui.add(style_info_button()).on_hover_text(
                                "Provide a short summary of the expected contribution \
                                from the participant and the tasks for the participant's team",
                            );
                        });
                        self.contribution.edit(ui, edit_section_titles);
                        if ui
                            .button("Add a note")
                            .on_hover_text(
                                "You may add new versions of text, which then can be \
                            optionally included in the document",
                            )
                            .clicked()
                        {
                            self.contribution.notes.push(Note::new());
                        }
                    }
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::CV)) => {
                        ui.add_space(SPACE_INTERNAL_EDITOR);
                        ui.horizontal(|ui| {
                            ui.label("The short CV of the Participant (Partner's team leader):");
                        });
                        self.cv.edit(ui, edit_section_titles);
                        if ui
                            .button("Add a note")
                            .on_hover_text(
                                "You may add new versions of text, which then can be \
                            optionally included in the document",
                            )
                            .clicked()
                        {
                            self.cv.notes.push(Note::new());
                        }
                    }
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Resources,
                    )) => {
                        ui.add_space(SPACE_INTERNAL_EDITOR);
                        ui.horizontal(|ui| {
                            ui.label("Partner's existing resources:");
                            ui.add(style_info_button()).on_hover_text(
                                "Other resources – \
                                equipment, facilities, software, etc.",
                            );
                        });
                        self.resources.edit(ui, edit_section_titles);
                        if ui
                            .button("Add a note")
                            .on_hover_text(
                                "You may add new versions of text, which then can be \
                            optionally included in the document",
                            )
                            .clicked()
                        {
                            self.resources.notes.push(Note::new());
                        }
                    }
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Team,
                    )) => {
                        ui.add_space(SPACE_INTERNAL_EDITOR);
                        ui.horizontal(|ui| {
                            ui.label("Partner's team:");
                            ui.add(style_info_button())
                                .on_hover_text("Human resources – who, \
                                including PhD students, postdocs and students, \
                                will participate in the project? How many people for each category?");
                        });
                        self.team.edit(ui, edit_section_titles);
                        if ui
                            .button("Add a note")
                            .on_hover_text(
                                "You may add new versions of text, which then can be \
                            optionally included in the document",
                            )
                            .clicked()
                        {
                            self.team.notes.push(Note::new());
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Team {
    pub fn preview(&self, ui: &mut Ui, resolution: usize) {
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
                        Variety::SectionTeam(SectionTeam::ProposedPartners) => {
                            self.proposed_partners
                                .preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionTeam(SectionTeam::ProjectLeader) => {
                            self.project_leader
                                .preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionTeam(SectionTeam::IndustrialPartners) => {
                            self.industrial_partners
                                .preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionTeam(SectionTeam::Proponents) => {
                            self.proponents.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                        }
                        Variety::SectionTeam(SectionTeam::Participants) => {
                            self.participants.iter().for_each(|partner| {
                                if partner.visible {
                                    ui.add_space(SPACE_INTERNAL_PREVIEW);
                                    partner.preview(ui, resolution);
                                }
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

impl Participant {
    fn preview(&self, ui: &mut egui::Ui, resolution: usize) {
        let chapters = &self.index_list;
        ui.label(
            RichText::new(&self.name)
                .size(SUBSECTION_FONT_SIZE)
                .color(SUBSECTION_FONT_COLOR),
        );

        if !&self.role.is_empty() {
            ui.label(&self.role);
        }
        if !&self.affiliation.is_empty() {
            ui.label(&self.affiliation);
        }
        if !&self.hyperlink.is_empty() {
            ui.hyperlink(&self.hyperlink);
        }
        if !&self.country.is_empty() {
            ui.label(&self.country);
        }
        if !&self.expertise.is_empty() {
            ui.label(&self.expertise);
        }

        for chapter in chapters {
            if chapter.tier > resolution {
                continue;
            } else {
                match chapter.variety {
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Budget,
                    )) => {
                        self.budget.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                    }
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Contribution,
                    )) => {
                        self.contribution.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                    }
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::CV)) => {
                        self.cv.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                    }
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Resources,
                    )) => {
                        self.resources.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                    }
                    Variety::SectionTeam(SectionTeam::ModuleParticipant(
                        ModuleParticipant::Team,
                    )) => {
                        self.team.preview(ui, Some(SPACE_INTERNAL_PREVIEW));
                    }
                    _ => {}
                }
            }
        }
    }
}
