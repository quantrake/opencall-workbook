use serde::Deserialize;

use crate::workbook::app::Workbook;
use crate::workbook::note::{Note, Notes};
use crate::workbook::project::{Project, ProjectOwner};
use crate::workbook::sections::attachments::Attachments;
use crate::workbook::sections::funding::{FundingOptions, Programme};
use crate::workbook::sections::idea::Idea;
use crate::workbook::sections::literature::Literature;
use crate::workbook::sections::methodology::Methodology;
use crate::workbook::sections::outcomes::Outcomes;
use crate::workbook::sections::prelim_results::PrelimResults;
use crate::workbook::sections::references::{Reference, References};
use crate::workbook::sections::scope::Scope;
use crate::workbook::sections::scripting::{Scripting, ScriptingOption};
use crate::workbook::sections::team::{Participant, Team};
use crate::workbook::sections::timeline::{Date, Timeline};
use crate::workbook::sections::working_name::ProjectTitle;

// ############################################################################
// ### Legacy data formats for conversion.
// ############################################################################

#[derive(Deserialize)]
pub struct WorkVersionB0002(String, ProjectVersion0002, Vec<ProjectVersion0002>);

impl WorkVersionB0002 {
    pub fn decode_bincode(encoded: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(encoded)
    }
}

// Note: The order of fields is important, as the `Project` is serialized
// as an array without field names!
#[derive(Deserialize)]
struct ProjectVersion0002 {
    record: String,

    owner: ProjectOwner0002,

    working_name: String,
    funding_program: Programme0002,
    idea: Idea0002,
    references: References0002,
    prelim_results: PrelimResults0002,
    contribution: Contribution0002,
    offer: Offer0002,
    team: Team0002,
    scripting: Scripting0002,
    survey: Literature0002,
    expected_results: ExpectedResults0002,
    attachments: Attachments0002,
}

#[derive(Deserialize)]
struct ProjectOwner0002 {
    email: String,
    account: String,
}

#[derive(Deserialize)]
struct Attachments0002 {
    title: String,
    tables: Notes0002,
    figures: Notes0002,
    other: Notes0002,
    visible: bool,
}

impl From<Attachments0002> for Attachments {
    fn from(val: Attachments0002) -> Self {
        Attachments {
            title: val.title,
            tables: val.tables.into(),
            figures: val.figures.into(),
            other: val.other.into(),
            visible: val.visible,
        }
    }
}

#[derive(Deserialize)]
struct Idea0002 {
    title: String,
    idea: Notes0002,
    visible: bool,
}

#[derive(Deserialize)]
struct PrelimResults0002 {
    title: String,
    prelim_results: Notes0002,
    visible: bool,
}

#[derive(Deserialize)]
struct Scripting0002 {
    title: String,
    answer: ScriptingOption0002,
    scripting: Notes0002,
    visible: bool,
}

#[derive(Deserialize)]
enum ScriptingOption0002 {
    Myself,
    Jointly,
    Other,
}

impl From<ScriptingOption0002> for ScriptingOption {
    fn from(val: ScriptingOption0002) -> Self {
        match val {
            ScriptingOption0002::Myself => ScriptingOption::Myself,
            ScriptingOption0002::Jointly => ScriptingOption::Jointly,
            ScriptingOption0002::Other => ScriptingOption::Other,
        }
    }
}

#[derive(Deserialize)]
struct Programme0002 {
    title: String,
    name: String,
    hyperlink: String,

    deadline: Date,
    project_start: Date,
    duration_years: f32,

    annotation: Notes0002,
    visible: bool,
}

#[derive(Deserialize)]
struct Team0002 {
    title: String,
    partners: Vec<Partner0002>,
    industrial_partners: Notes0002,
    proponents: Notes0002,
    visible: bool,
}

// We need it if we add CV to the `Partner` struct.
#[derive(Deserialize)]
struct Partner0002 {
    name: String,
    role: String,
    affiliation: String,
    hyperlink: String,
    country: String,
    expertise: String,
    contribution: Notes0002,
    team: Notes0002,
    resources: Notes0002,
    budget: Notes0002,
    visible: bool,
}

impl From<Partner0002> for Participant {
    fn from(val: Partner0002) -> Self {
        Participant {
            name: val.name,
            role: val.role,
            affiliation: val.affiliation,
            hyperlink: val.hyperlink,
            country: val.country,
            expertise: val.expertise,
            contribution: val.contribution.into(),
            team: val.team.into(),
            resources: val.resources.into(),
            budget: val.budget.into(),
            visible: val.visible,

            ..Default::default()
        }
    }
}

#[derive(Deserialize)]
struct Contribution0002 {
    title: String,
    contribution: Notes0002,
    visible: bool,
}

#[derive(Deserialize)]
struct Offer0002 {
    title: String,
    offer: Notes0002,
    visible: bool,
}

#[derive(Deserialize)]
struct Literature0002 {
    title: String,
    literature_survey: Notes0002,
    references: Vec<Reference0002>,
    visible: bool,
}

#[derive(Deserialize)]
struct Reference0002 {
    title: String,
    hyperlink: String,
    source_details: Notes0002,
    visible: bool,
}

#[derive(Deserialize)]
struct References0002 {
    title: String,
    references: Vec<Reference0002>,
    visible: bool,
}

impl From<Reference0002> for Reference {
    fn from(val: Reference0002) -> Self {
        Reference {
            title: val.title,
            hyperlink: val.hyperlink,
            source_details: val.source_details.into(),
            visible: val.visible,
        }
    }
}

#[derive(Deserialize)]
struct ExpectedResults0002 {
    title: String,
    results: Notes0002,
    visible: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Note0002 {
    note: String,
    #[serde(skip)]
    hint: String,
    visible: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Notes0002 {
    notes: Vec<Note0002>,
}

impl From<Note0002> for Note {
    fn from(val: Note0002) -> Self {
        Note {
            note: val.note,
            hint: val.hint,
            visible: val.visible,
        }
    }
}

impl From<Notes0002> for Notes {
    fn from(val: Notes0002) -> Self {
        Notes {
            notes: val.notes.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl Workbook {
    pub fn apply_format_b0002(&mut self, work: WorkVersionB0002) {
        let _old_version = work.0;
        let old_project = work.1;

        // Map ProjectVersion0002 to Project.
        self.project = Project {
            record: old_project.record,

            owner: ProjectOwner {
                email: old_project.owner.email,
                account: old_project.owner.account,
            },

            index_list: Project::default().index_list,

            attachments: old_project.attachments.into(),
            funding: FundingOptions {
                title: old_project.funding_program.title,
                funding_options: vec![Programme {
                    title: old_project.funding_program.name,
                    hyperlink: old_project.funding_program.hyperlink,
                    deadline: old_project.funding_program.deadline,
                    annotation: old_project.funding_program.annotation.into(),
                    visible: old_project.funding_program.visible,
                }],
                visible: old_project.funding_program.visible,
            },
            idea: Idea {
                title: old_project.idea.title,
                hypothesis: old_project.idea.idea.into(),
                references: References {
                    title: old_project.references.title,
                    references: old_project
                        .references
                        .references
                        .into_iter()
                        .map(|x| x.into())
                        .collect(),
                    visible: old_project.references.visible,
                },
                visible: old_project.idea.visible,

                ..Default::default()
            },
            literature: Literature {
                title: old_project.survey.title,
                literature_survey: old_project.survey.literature_survey.into(),
                references: old_project
                    .survey
                    .references
                    .into_iter()
                    .map(|x| x.into())
                    .collect(),
                visible: old_project.survey.visible,
            },
            methodology: Methodology {
                title: old_project.offer.title,
                methodology: old_project.offer.offer.into(),
                visible: old_project.offer.visible,
            },
            outcomes: Outcomes {
                title: old_project.expected_results.title,
                results: old_project.expected_results.results.into(),
                visible: old_project.expected_results.visible,

                ..Default::default()
            },
            prelim_results: PrelimResults {
                title: old_project.prelim_results.title,
                prelim_results: old_project.prelim_results.prelim_results.into(),
                visible: old_project.prelim_results.visible,
            },
            scope: Scope {
                title: old_project.contribution.title,
                activities: old_project.contribution.contribution.into(),
                visible: old_project.contribution.visible,

                ..Default::default()
            },
            scripting: Scripting {
                title: old_project.scripting.title,
                scripting: old_project.scripting.scripting.into(),
                visible: old_project.scripting.visible,

                ..Default::default()
            },
            team: Team {
                title: old_project.team.title,
                participants: old_project
                    .team
                    .partners
                    .into_iter()
                    .map(|x| x.into())
                    .collect(),
                industrial_partners: old_project.team.industrial_partners.into(),
                proponents: old_project.team.proponents.into(),
                visible: old_project.team.visible,

                ..Default::default()
            },
            timeline: Timeline {
                project_start: old_project.funding_program.project_start,
                duration_years: old_project.funding_program.duration_years,

                ..Default::default()
            },
            working_name: ProjectTitle {
                title: old_project.working_name,
                visible: true,
                ..Default::default()
            },

            ..Default::default()
        };

        self.stored_projects = Vec::new();
        let old_stored_projects = work.2;
        for old_project in old_stored_projects {
            self.stored_projects.push(Project {
                record: old_project.record,
                owner: ProjectOwner {
                    email: old_project.owner.email,
                    account: old_project.owner.account,
                },

                index_list: Project::default().index_list,

                attachments: old_project.attachments.into(),
                funding: FundingOptions {
                    title: old_project.funding_program.title,
                    funding_options: vec![Programme {
                        title: old_project.funding_program.name,
                        hyperlink: old_project.funding_program.hyperlink,
                        deadline: old_project.funding_program.deadline,
                        annotation: old_project.funding_program.annotation.into(),
                        visible: old_project.funding_program.visible,
                    }],
                    visible: old_project.funding_program.visible,
                },
                idea: Idea {
                    title: old_project.idea.title,
                    hypothesis: old_project.idea.idea.into(),
                    references: References {
                        title: old_project.references.title,
                        references: old_project
                            .references
                            .references
                            .into_iter()
                            .map(|x| x.into())
                            .collect(),
                        visible: old_project.references.visible,
                    },
                    visible: old_project.idea.visible,

                    ..Default::default()
                },
                literature: Literature {
                    title: old_project.survey.title,
                    literature_survey: old_project.survey.literature_survey.into(),
                    references: old_project
                        .survey
                        .references
                        .into_iter()
                        .map(|x| x.into())
                        .collect(),
                    visible: old_project.survey.visible,
                },
                methodology: Methodology {
                    title: old_project.offer.title,
                    methodology: old_project.offer.offer.into(),
                    visible: old_project.offer.visible,
                },
                outcomes: Outcomes {
                    title: old_project.expected_results.title,
                    results: old_project.expected_results.results.into(),
                    visible: old_project.expected_results.visible,

                    ..Default::default()
                },
                prelim_results: PrelimResults {
                    title: old_project.prelim_results.title,
                    prelim_results: old_project.prelim_results.prelim_results.into(),
                    visible: old_project.prelim_results.visible,
                },
                scope: Scope {
                    title: old_project.contribution.title,
                    activities: old_project.contribution.contribution.into(),
                    visible: old_project.contribution.visible,

                    ..Default::default()
                },
                scripting: Scripting {
                    title: old_project.scripting.title,
                    answer: old_project.scripting.answer.into(),
                    scripting: old_project.scripting.scripting.into(),
                    visible: old_project.scripting.visible,
                },
                team: Team {
                    title: old_project.team.title,
                    participants: old_project
                        .team
                        .partners
                        .into_iter()
                        .map(|x| x.into())
                        .collect(),
                    industrial_partners: old_project.team.industrial_partners.into(),
                    proponents: old_project.team.proponents.into(),
                    visible: old_project.team.visible,

                    ..Default::default()
                },
                timeline: Timeline {
                    project_start: old_project.funding_program.project_start,
                    duration_years: old_project.funding_program.duration_years,

                    ..Default::default()
                },
                working_name: ProjectTitle {
                    title: old_project.working_name,
                    visible: true,
                    ..Default::default()
                },

                ..Default::default()
            });
        }
    }
}
