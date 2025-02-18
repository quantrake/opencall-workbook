use chrono::{DateTime, Local, NaiveDate, TimeZone};

use super::v_b0004::{Parent, Rank, YearMonthDay};
use crate::workbook::sections::{funding::Programme, references::Reference, team::Participant};
use crate::workbook::store::v_b0004::Store;
use crate::workbook::{project::Project, sections::timeline::Date};

impl From<YearMonthDay> for Date {
    fn from(val: YearMonthDay) -> Self {
        let year = val.year;
        let month = val.month;
        let day = val.day;
        let date: DateTime<Local> = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(year, month, day)
                    .unwrap()
                    .and_hms_opt(0, 1, 1)
                    .unwrap(),
            )
            .unwrap();
        Date {
            year,
            month,
            day,
            date,
        }
    }
}

impl From<Store> for Project {
    fn from(val: Store) -> Self {
        let chunks = &val.chunks;
        let mut project = Project::default();
        let mut ref_source = Reference::default();
        let mut funding_option = Programme::default();
        let mut participant = Participant::default();
        project.owner = val.owner;
        project.record = val.record;
        project.resolution = val.resolution;
        for chunk in chunks {
            match chunk.parent {
                None => {
                    match chunk.address {
                        // WorkingName
                        0 => {
                            project.working_name.title = chunk.title.clone();
                            project.working_name.visible = chunk.visible;
                        }
                        // Idea
                        1 => {
                            project.idea.title = chunk.title.clone();
                            project.idea.visible = chunk.visible;
                        }
                        // Funding
                        2 => {
                            project.funding.title = chunk.title.clone();
                            project.funding.visible = chunk.visible;
                        }
                        // Scope
                        3 => {
                            project.scope.title = chunk.title.clone();
                            project.scope.visible = chunk.visible;
                        }
                        // Timeline
                        4 => {
                            project.timeline.title = chunk.title.clone();
                            project.timeline.visible = chunk.visible;
                        }
                        // Scripting
                        5 => {
                            project.scripting.title = chunk.title.clone();
                            project.scripting.visible = chunk.visible;
                        }
                        // Team
                        6 => {
                            project.team.title = chunk.title.clone();
                            project.team.visible = chunk.visible;
                        }
                        // PrelimResults
                        7 => {
                            project.prelim_results.title = chunk.title.clone();
                            project.prelim_results.visible = chunk.visible;
                        }
                        // Methodology
                        8 => {
                            project.methodology.title = chunk.title.clone();
                            project.methodology.visible = chunk.visible;
                        }
                        // Outcomes
                        9 => {
                            project.outcomes.title = chunk.title.clone();
                            project.outcomes.visible = chunk.visible;
                        }
                        // Literature
                        10 => {
                            project.literature.title = chunk.title.clone();
                            project.literature.visible = chunk.visible;
                        }
                        // Resources
                        11 => {
                            project.resources.title = chunk.title.clone();
                            project.resources.visible = chunk.visible;
                        }
                        // Budget
                        12 => {
                            project.budget.title = chunk.title.clone();
                            project.budget.visible = chunk.visible;
                        }
                        // Attachments
                        13 => {
                            project.attachments.title = chunk.title.clone();
                            project.attachments.visible = chunk.visible;
                        }
                        _ => unimplemented!("Workbook (Project) has 14 subsections"),
                    }
                }
                // WorkingName - Options
                Some(Parent {
                    address: 0,
                    variety: Rank::Top,
                }) => {
                    if chunk.address == 0 {
                        project.working_name.options = chunk.body.notes.clone();
                    }
                }
                // Idea - Subsections
                Some(Parent {
                    address: 1,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        // IdeaProblem
                        0 => {
                            project.idea.problem = chunk.body.notes.clone();
                        }
                        // IdeaHypothesis
                        1 => {
                            project.idea.hypothesis = chunk.body.notes.clone();
                        }
                        // IdeaSummary 'Abstract'
                        2 => {
                            project.idea.summary = chunk.body.notes.clone();
                        }
                        // IdeaDescription `ProjectDescription`
                        3 => {
                            project.idea.description = chunk.body.notes.clone();
                        }
                        // IdeaReferences - Subsection Title `KeyReferences`
                        4 => {
                            project.idea.references.title = chunk.title.clone();
                            project.idea.references.visible = chunk.visible;
                        }
                        _ => {} // _ => unimplemented!("Idea has 5 subsections")
                    }
                }
                // IdeaReferences - Subsection Title `KeyReferences`
                // Introduced a special `Rank::IdeaReferences`
                // to distinguish its 'children' from 'children' of
                // other branches of `Rank::Mid`.
                Some(Parent {
                    address: 4,
                    variety: Rank::IdeaReferences,
                }) => {
                    let citation = chunk;
                    ref_source.title = citation.title.clone();
                    ref_source.visible = citation.visible;
                    project.idea.references.references.push(ref_source.clone());
                }
                // IdeaReferencesCitation
                // Introduced a special `Rank::IdeaReferencesCitation`
                // to distinguish its 'children' from 'children' of
                // other branches of `Rank::Low`.
                // Each Citation has a different address, so parents are
                // rotated with `source_counter`.
                Some(Parent {
                    address: source_counter,
                    variety: Rank::IdeaReferencesCitation,
                }) => {
                    let citation = chunk;
                    match chunk.address {
                        // IdeaReferencesCitationHyperlink
                        0 => {
                            project.idea.references.references[source_counter].hyperlink =
                                citation.body.phrase.clone();
                        }
                        // IdeaReferencesCitationDetails
                        1 => {
                            project.idea.references.references[source_counter].source_details =
                                citation.body.notes.clone();
                        }
                        _ => {} // _ => unimplemented!("Reference has 2 subsections")
                    }
                }
                // FundingProgram - Subsection Title `FundingProgram`
                // Introduced a special `Rank::FundingProgram`
                // to distinguish its 'children' from 'children' of
                // other branches of `Rank::Mid`.
                Some(Parent {
                    address: 2,
                    variety: Rank::Top,
                }) => {
                    let funding_program = chunk;
                    funding_option.title = funding_program.title.clone();
                    funding_option.visible = funding_program.visible;
                    project.funding.funding_options.push(funding_option.clone());
                }
                // FundingProgram - Funding Program Options
                Some(Parent {
                    address: program_counter,
                    variety: Rank::FundingProgram,
                }) => {
                    let funding_program = chunk;
                    match chunk.address {
                        // FundingProgramAnnotation
                        0 => {
                            project.funding.funding_options[program_counter].annotation =
                                funding_program.body.notes.clone();
                        }
                        // FundingProgramHyperlink
                        1 => {
                            project.funding.funding_options[program_counter].hyperlink =
                                funding_program.body.phrase.clone();
                        }
                        // FundingProgramDeadline
                        2 => {
                            project.funding.funding_options[program_counter].deadline =
                                funding_program.body.date.clone().into();
                        }
                        _ => {} // _ => unimplemented!("Funding Program has 3 subsections")
                    }
                }
                // Scope - Subsections
                Some(Parent {
                    address: 3,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        // ScopeSuggestedTasks
                        0 => {
                            project.scope.suggested_tasks = chunk.body.notes.clone();
                        }
                        // ScopeObjectives
                        1 => {
                            project.scope.objectives = chunk.body.notes.clone();
                        }
                        // ScopeActivities
                        2 => {
                            project.scope.activities = chunk.body.notes.clone();
                        }
                        // ScopeWorkPlan
                        3 => {
                            project.scope.work_plan = chunk.body.notes.clone();
                        }
                        // ScopeTasks
                        4 => {
                            project.scope.tasks = chunk.body.notes.clone();
                        }
                        _ => {} // _ => unimplemented!("Scope has 5 subsections")
                    }
                }
                // Timeline - Subsections
                Some(Parent {
                    address: 4,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        // TimelineProjectTiming - ProjectStart
                        0 => {
                            project.timeline.project_start = chunk.body.date.clone().into();
                        }
                        // TimelineProjectTiming - ProjectDuration
                        1 => {
                            project.timeline.duration_years = chunk.body.numeral;
                        }
                        // TimelineMilestones
                        2 => {
                            project.timeline.milestones = chunk.body.notes.clone();
                        }
                        _ => unimplemented!("Timeline has 3 subsections"),
                    }
                }
                // Scripting - Options
                Some(Parent {
                    address: 5,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        0 => {
                            project.scripting.answer = chunk.body.option.clone();
                        }
                        1 => {
                            project.scripting.scripting = chunk.body.notes.clone();
                        }
                        _ => {} // _ => unimplemented!("Scripting has 2 subsections")
                    }
                }
                // Team - Subsections
                Some(Parent {
                    address: 6,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        // TeamProposedPartners
                        0 => {
                            project.team.proposed_partners = chunk.body.notes.clone();
                        }
                        // TeamProjectLeader
                        1 => {
                            project.team.project_leader = chunk.body.notes.clone();
                        }
                        // TeamIndustrialPartners
                        2 => {
                            project.team.industrial_partners = chunk.body.notes.clone();
                        }
                        // TeamProponents
                        3 => {
                            project.team.proponents = chunk.body.notes.clone();
                        }
                        _ => {} // _ => unimplemented!()
                    }
                }
                // TeamParticipants - Subsection Title `Participants`
                // Introduced a special `Rank::TeamParticipants`
                // to distinguish its 'children' from 'children' of
                // other branches of `Rank::Mid`.
                Some(Parent {
                    address: 4,
                    variety: Rank::TeamParticipants,
                }) => {
                    let partner = chunk;
                    participant.name = partner.title.clone();
                    participant.visible = partner.visible;
                    project.team.participants.push(participant.clone());
                }
                // TeamParticipantsPartner
                Some(Parent {
                    address: member_counter,
                    variety: Rank::TeamParticipantsPartner,
                }) => {
                    let partner = chunk;
                    match chunk.address {
                        // TeamParticipantsPartner - Role
                        0 => {
                            project.team.participants[member_counter].role =
                                partner.body.phrase.clone();
                        }
                        // TeamParticipantsPartner - Affiliation
                        1 => {
                            project.team.participants[member_counter].affiliation =
                                partner.body.phrase.clone();
                        }
                        // TeamParticipantsPartner - Hyperlink
                        2 => {
                            project.team.participants[member_counter].hyperlink =
                                partner.body.phrase.clone();
                        }
                        // TeamParticipantsPartner - Country
                        3 => {
                            project.team.participants[member_counter].country =
                                partner.body.phrase.clone();
                        }
                        // TeamParticipantsPartner - Expertise
                        4 => {
                            project.team.participants[member_counter].expertise =
                                partner.body.phrase.clone();
                        }
                        // TeamParticipantsPartner - Contribution
                        5 => {
                            project.team.participants[member_counter].contribution =
                                partner.body.notes.clone();
                        }
                        // TeamParticipantsPartner - Team
                        6 => {
                            project.team.participants[member_counter].team =
                                partner.body.notes.clone();
                        }
                        // TeamParticipantsPartner - CV
                        7 => {
                            project.team.participants[member_counter].cv =
                                partner.body.notes.clone();
                        }
                        // TeamParticipantsPartner - Resources
                        8 => {
                            project.team.participants[member_counter].resources =
                                partner.body.notes.clone();
                        }
                        // TeamParticipantsPartner - Budget
                        9 => {
                            project.team.participants[member_counter].budget =
                                partner.body.notes.clone();
                        }
                        _ => {} // _ => unimplemented!()
                    }
                }
                // PrelimResults
                Some(Parent {
                    address: 7,
                    variety: Rank::Top,
                }) => {
                    if chunk.address == 0 {
                        project.prelim_results.prelim_results = chunk.body.notes.clone();
                    }
                }
                // Methodology
                Some(Parent {
                    address: 8,
                    variety: Rank::Top,
                }) => {
                    if chunk.address == 0 {
                        project.methodology.methodology = chunk.body.notes.clone();
                    }
                }
                // Outcomes - Subsections
                Some(Parent {
                    address: 9,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        // Outcomes - ExpectedResults
                        0 => {
                            project.outcomes.results = chunk.body.notes.clone();
                        }
                        // Outcomes - Impact
                        1 => {
                            project.outcomes.impact = chunk.body.notes.clone();
                        }
                        // Outcomes - Propagation
                        2 => {
                            project.outcomes.propagation = chunk.body.notes.clone();
                        }
                        _ => {} // _ => unimplemented!("Outcomes has 3 subsections")
                    }
                }
                // Literature - Subsections
                Some(Parent {
                    address: 10,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        // Literature - Literature Survey
                        0 => {
                            project.literature.literature_survey = chunk.body.notes.clone();
                        }
                        1 => {
                            // LiteratureSources
                        }
                        _ => {} // _ => unimplemented!("Literature has 2 subsections")
                    }
                }
                // LiteratureSources - Subsection Title `Literature Sources`
                // Introduced a special `Rank::LiteratureSources`
                // to distinguish its 'children' from 'children' of
                // other branches of `Rank::Mid`.
                Some(Parent {
                    address: 1,
                    variety: Rank::LiteratureSources,
                }) => {
                    let citation = chunk;
                    ref_source.title = citation.title.clone();
                    ref_source.visible = citation.visible;
                    project.literature.references.push(ref_source.clone());
                }
                // LiteratureSourcesCitation
                // Introduced a special `Rank::LiteratureSourcesCitation`
                // to distinguish its 'children' from 'children' of
                // other branches of `Rank::low`.
                // Each Citation has a different address, so parents are
                // rotated with `bib_counter`.
                Some(Parent {
                    address: bib_counter,
                    variety: Rank::LiteratureSourcesCitation,
                }) => {
                    let citation = chunk;
                    match chunk.address {
                        // LiteratureCitation - Hyperlink
                        0 => {
                            project.literature.references[bib_counter].hyperlink =
                                citation.body.phrase.clone();
                        }
                        // LiteratureCitation - Details
                        1 => {
                            project.literature.references[bib_counter].source_details =
                                citation.body.notes.clone();
                        }
                        _ => {} // _ => unimplemented!()
                    }
                }
                // Resources - Subsections
                Some(Parent {
                    address: 11,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        // Resources - Existing
                        0 => {
                            project.resources.existing = chunk.body.notes.clone();
                        }
                        // Resources - Additional
                        1 => {
                            project.resources.further = chunk.body.notes.clone();
                        }
                        _ => {} // _ => unimplemented!("Resources has 2 subsections")
                    }
                }
                // Budget - Subsections
                Some(Parent {
                    address: 12,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        // Budget - Personnel
                        0 => {
                            project.budget.personnel = chunk.body.notes.clone();
                        }
                        // Budget - Facilities
                        1 => {
                            project.budget.facilities = chunk.body.notes.clone();
                        }
                        // Budget - Materials
                        2 => {
                            project.budget.materials = chunk.body.notes.clone();
                        }
                        // Budget - Workshops
                        3 => {
                            project.budget.workshops = chunk.body.notes.clone();
                        }
                        // Budget - Overheads
                        4 => {
                            project.budget.overheads = chunk.body.notes.clone();
                        }
                        // Budget - Miscellaneous
                        5 => {
                            project.budget.misc = chunk.body.notes.clone();
                        }
                        _ => {} // !("Budget has 6 subsections")
                    }
                }
                // Attachments - Subsections
                Some(Parent {
                    address: 13,
                    variety: Rank::Top,
                }) => {
                    match chunk.address {
                        // Attachments - Tables
                        0 => {
                            project.attachments.tables = chunk.body.notes.clone();
                        }
                        // Attachments - Figures
                        1 => {
                            project.attachments.figures = chunk.body.notes.clone();
                        }
                        // Attachments - Other
                        2 => {
                            project.attachments.other = chunk.body.notes.clone();
                        }
                        _ => {} // _ => unimplemented!("Attachments has 3 subsections")
                    }
                }
                _ => {} // _ => todo!()
            }
        }
        project
    }
}

// Test the correctness of the storage format conversion into the `Project`.
#[test]
fn conversion_from_store() {
    use super::v_b0004::{Body, Chunk};
    use crate::workbook::note::{Note, Notes};
    use crate::workbook::sections::scripting::ScriptingOption;
    use crate::workbook::sections::timeline::Date;

    let test = &Store {
        format: "B0004".to_string(),
        owner: crate::workbook::project::ProjectOwner {
            email: "abc@email.tst".to_string(),
            account: "abc123".to_string(),
        },
        record: "test_record".to_string(),
        resolution: 3,
        chunks: vec![
            // 0
            Chunk {
                // WorkingName
                address: 0,
                parent: None,
                title: "Test project name".to_string(),
                visible: true,
                ..Default::default()
            },
            // 1
            Chunk {
                // WorkingNameOptions
                address: 0,
                parent: Some(
                    // Variant::WorkingName,
                    Parent {
                        address: 0,
                        variety: Rank::Top,
                    },
                ),
                // title: "Name options".to_string(), // no `title` for this subsection
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Name option 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 2
            Chunk {
                // Idea
                address: 1,
                parent: None,
                title: "Idea".to_string(),
                visible: true,
                ..Default::default()
            },
            // 3
            Chunk {
                // IdeaProblem
                address: 0,
                parent: Some(
                    // Variant::Idea,
                    Parent {
                        address: 1,
                        variety: Rank::Top,
                    },
                ),
                // title: "Problem".to_string(), // no `title` for this subsection
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Problem note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 4
            Chunk {
                // IdeaHypothesis
                address: 1,
                parent: Some(
                    // Variant::Idea,
                    Parent {
                        address: 1,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Hypothesis note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 5
            Chunk {
                // IdeaSummary
                address: 2,
                parent: Some(
                    // Variant::Idea,
                    Parent {
                        address: 1,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Abstract note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 6
            Chunk {
                // IdeaDescription
                address: 3,
                parent: Some(
                    // Variant::Idea,
                    Parent {
                        address: 1,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Project description note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 7
            Chunk {
                // IdeaReferences
                address: 4,
                parent: Some(
                    // Variant::Idea,
                    Parent {
                        address: 1,
                        variety: Rank::Top,
                    },
                ),
                title: "Key References".to_string(),
                visible: true,
                ..Default::default()
            },
            // 8
            Chunk {
                // IdeaReferencesCitation = IdeaReferencesCitationName
                address: 0,
                parent: Some(
                    // Variant::IdeaReferences,
                    Parent {
                        address: 4,
                        variety: Rank::IdeaReferences,
                    },
                ),
                title: "Key source 1".to_string(),
                visible: true,
                ..Default::default()
            },
            // 9
            Chunk {
                // IdeaReferencesCitationHyperlink
                address: 0,
                parent: Some(
                    // Variant::IdeaReferencesCitation,
                    Parent {
                        address: 0,
                        variety: Rank::IdeaReferencesCitation,
                    },
                ),
                body: Body {
                    phrase: "link source 1".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 10
            Chunk {
                // IdeaReferencesCitationDetails
                address: 1,
                parent: Some(
                    // Variant::IdeaReferencesCitation,
                    Parent {
                        address: 0,
                        variety: Rank::IdeaReferencesCitation,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Source 1 details".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 11
            Chunk {
                // IdeaReferencesCitation = IdeaReferencesCitationName
                address: 1,
                parent: Some(
                    // Variant::IdeaReferences,
                    Parent {
                        address: 4,
                        variety: Rank::IdeaReferences,
                    },
                ),
                title: "Key source 2".to_string(),
                visible: true,
                ..Default::default()
            },
            // 12
            Chunk {
                // IdeaReferencesCitationHyperlink
                address: 0,
                parent: Some(
                    // Variant::IdeaReferencesCitation,
                    Parent {
                        address: 1,
                        variety: Rank::IdeaReferencesCitation,
                    },
                ),
                body: Body {
                    phrase: "link source 2".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 13
            Chunk {
                // IdeaReferencesCitationDetails
                address: 1,
                parent: Some(
                    // Variant::IdeaReferencesCitation,
                    Parent {
                        address: 1,
                        variety: Rank::IdeaReferencesCitation,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Source 2 details".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 14
            Chunk {
                // Funding
                address: 2,
                parent: None,
                title: "Funding".to_string(),
                visible: true,
                ..Default::default()
            },
            // 15
            Chunk {
                // FundingProgram = FundingProgramName
                address: 0,
                parent: Some(
                    // Variant::Funding,
                    Parent {
                        address: 2,
                        variety: Rank::Top,
                    },
                ),
                title: "Funding Option 1".to_string(),
                visible: true,
                ..Default::default()
            },
            // 16
            Chunk {
                // FundingProgramAnnotation
                address: 0,
                parent: Some(
                    // Variant::FundingProgram,
                    Parent {
                        address: 0,
                        variety: Rank::FundingProgram,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Funding Option 1 annotation".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 17
            Chunk {
                // FundingProgramHyperlink
                address: 1,
                parent: Some(
                    // Variant::FundingProgram,
                    Parent {
                        address: 0,
                        variety: Rank::FundingProgram,
                    },
                ),
                body: Body {
                    phrase: "link funding option 1".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 18
            Chunk {
                // FundingProgramDeadline
                address: 2,
                parent: Some(
                    // Variant::FundingProgram,
                    Parent {
                        address: 0,
                        variety: Rank::FundingProgram,
                    },
                ),
                body: Body {
                    date: Date {
                        day: 31,
                        month: 03,
                        year: 2025,
                        ..Default::default() // *** TESTING ONLY!!! ***
                    }
                    .into(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 19
            Chunk {
                // FundingProgram = FundingProgramName
                address: 1,
                parent: Some(
                    // Variant::Funding,
                    Parent {
                        address: 2,
                        variety: Rank::Top,
                    },
                ),
                title: "Funding Option 2".to_string(),
                visible: true,
                ..Default::default()
            },
            /* TODO: Funding Option 2 - Details: annotation, hyperlink and deadline
            */
            // 20
            Chunk {
                // Scope
                address: 3,
                parent: None,
                title: "Project Scope: Objectives and Planned Activities".to_string(),
                visible: true,
                ..Default::default()
            },
            // 21
            Chunk {
                // ScopeSuggestedTasks
                address: 0,
                parent: Some(
                    // Variant::Scope,
                    Parent {
                        address: 3,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Suggested Tasks note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 22
            Chunk {
                // ScopeObjectives
                address: 1,
                parent: Some(
                    // Variant::Scope,
                    Parent {
                        address: 3,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Objectives note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 23
            Chunk {
                // ScopeActivities
                address: 2,
                parent: Some(
                    // Variant::Scope,
                    Parent {
                        address: 3,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Activities note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 24
            Chunk {
                // ScopeWorkPlan
                address: 3,
                parent: Some(
                    // Variant::Scope,
                    Parent {
                        address: 3,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Work Plan note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 25
            Chunk {
                // ScopeTasks
                address: 4,
                parent: Some(
                    // Variant::Scope,
                    Parent {
                        address: 3,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Tasks note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 26
            Chunk {
                // Timeline
                address: 4,
                parent: None,
                title: "Timeline".to_string(),
                visible: true,
                ..Default::default()
            },
            // 27
            Chunk {
                // TimelineProjectTiming - ProjectStart
                address: 0,
                parent: Some(
                    // Variant::Timeline,
                    Parent {
                        address: 4,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    date: Date {
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 28
            Chunk {
                // TimelineProjectTiming - ProjectDuration
                address: 1,
                parent: Some(
                    // Variant::Timeline,
                    Parent {
                        address: 4,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    numeral: 4.5,
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 29
            Chunk {
                // TimelineProjectTiming - Milestones
                address: 2,
                parent: Some(
                    // Variant::Timeline,
                    Parent {
                        address: 4,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Milestones note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 30
            Chunk {
                // Scripting
                address: 5,
                parent: None,
                title: "Who Will Write The Project Proposal".to_string(),
                visible: true,
                ..Default::default()
            },
            // 31
            Chunk {
                // ScriptingOption
                address: 0,
                parent: Some(
                    // Variant::Scripting,
                    Parent {
                        address: 5,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    option: ScriptingOption::Jointly,
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 32
            Chunk {
                // ScriptingOption - Notes
                address: 1,
                parent: Some(
                    // Variant::Scripting,
                    Parent {
                        address: 5,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Scripting Option note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 33
            Chunk {
                // Team
                address: 6,
                parent: None,
                title: "Team".to_string(),
                visible: true,
                ..Default::default()
            },
            // 34
            Chunk {
                // TeamProposedPartners
                address: 0,
                parent: Some(
                    // Variant::Team,
                    Parent {
                        address: 6,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Proposed Partners note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 35
            Chunk {
                // TeamProjectLeader
                address: 1,
                parent: Some(
                    // Variant::Team,
                    Parent {
                        address: 6,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Project Leader note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 36
            Chunk {
                // TeamIndustrialPartners
                address: 2,
                parent: Some(
                    // Variant::Team,
                    Parent {
                        address: 6,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Industrial Partners note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 37
            Chunk {
                // TeamProponents
                address: 3,
                parent: Some(
                    // Variant::Team,
                    Parent {
                        address: 6,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Proponents note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 38
            Chunk {
                // TeamParticipantsPartner
                address: 0,
                parent: Some(
                    // Variant::TeamParticipants,
                    Parent {
                        address: 4,
                        variety: Rank::TeamParticipants,
                    },
                ),
                title: "Partner 1 Name".to_string(),
                visible: true,
                ..Default::default()
            },
            // 39
            Chunk {
                // TeamParticipantsPartner - Role
                address: 0,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 1 role".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 40
            Chunk {
                // TeamParticipantsPartner - Affiliation
                address: 1,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 1 affiliation".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 41
            Chunk {
                // TeamParticipantsPartner - Hyperlink
                address: 2,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 1 hyperlink".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 42
            Chunk {
                // TeamParticipantsPartner - Country
                address: 3,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 1 country".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 43
            Chunk {
                // TeamParticipantsPartner - Expertise
                address: 4,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 1 expertise".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 44
            Chunk {
                // TeamParticipantsPartner - Contribution
                address: 5,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 1 contribution note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 45
            Chunk {
                // TeamParticipantsPartner - Team
                address: 6,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 1 team note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 46
            Chunk {
                // TeamParticipantsPartner - CV
                address: 7,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 1 CV note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 47
            Chunk {
                // TeamParticipantsPartner - Resources
                address: 8,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 1 resources note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 48
            Chunk {
                // TeamParticipantsPartner - Budget
                address: 9,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 0,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 1 budget note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner
                address: 1,
                parent: Some(
                    // Variant::TeamParticipants,
                    Parent {
                        address: 4,
                        variety: Rank::TeamParticipants,
                    },
                ),
                title: "Partner 2 Name".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner - Role
                address: 0,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 2 role".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner - Affiliation
                address: 1,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 2 affiliation".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner - Hyperlink
                address: 2,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 2 hyperlink".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner - Country
                address: 3,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 2 country".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner - Expertise
                address: 4,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    phrase: "Partner 2 expertise".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // 44
            Chunk {
                // TeamParticipantsPartner - Contribution
                address: 5,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 2 contribution note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner - Team
                address: 6,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 2 team note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner - CV
                address: 7,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 2 CV note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner - Resources
                address: 8,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 2 resources note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // TeamParticipantsPartner - Budget
                address: 9,
                parent: Some(
                    // Variant::TeamParticipantsPartner,
                    Parent {
                        address: 1,
                        variety: Rank::TeamParticipantsPartner,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Partner 2 budget note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // PrelimResults
                address: 7,
                parent: None,
                title: "Preliminary Results".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // PrelimResults
                address: 0,
                parent: Some(
                    // Variant::PrelimResults,
                    Parent {
                        address: 7,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Preliminary Results note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Methodology
                address: 8,
                parent: None,
                title: "Research Methodology".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Methodology
                address: 0,
                parent: Some(
                    // Variant::Methodology,
                    Parent {
                        address: 8,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Methodology note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Outcomes
                address: 9,
                parent: None,
                title: "Expected Results, Impact and Dissemination".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Outcomes - Expected Results
                address: 0,
                parent: Some(
                    // Variant::Outcomes,
                    Parent {
                        address: 9,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Expected Results note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Outcomes - Impact
                address: 1,
                parent: Some(
                    // Variant::Outcomes,
                    Parent {
                        address: 9,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Impact note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Outcomes - Propagation
                address: 2,
                parent: Some(
                    // Variant::Outcomes,
                    Parent {
                        address: 9,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Propagation note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Literature
                address: 10,
                parent: None,
                title: "Literature Survey".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Literature Survey
                address: 0,
                parent: Some(
                    // Variant::Literature,
                    Parent {
                        address: 10,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Literature Survey note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // LiteratureSourcesCitation Title = LiteratureSourcesCitationName
                address: 0,
                parent: Some(
                    // Variant::LiteratureSources,
                    Parent {
                        address: 1,
                        variety: Rank::LiteratureSources,
                    },
                ),
                title: "Literature source 1".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // LiteratureSourcesCitationHyperlink
                address: 0,
                parent: Some(
                    // Variant::LiteratureSourcesCitation,
                    Parent {
                        address: 0,
                        variety: Rank::LiteratureSourcesCitation,
                    },
                ),
                body: Body {
                    phrase: "Literature link source 1".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // LiteratureSourcesCitationDetails
                address: 1,
                parent: Some(
                    // Variant::LiteratureSourcesCitation,
                    Parent {
                        address: 0,
                        variety: Rank::LiteratureSourcesCitation,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Literature Source 1 details".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // LiteratureSourcesCitation Title = LiteratureSourcesCitationName
                address: 1,
                parent: Some(
                    // Variant::LiteratureSources,
                    Parent {
                        address: 1,
                        variety: Rank::LiteratureSources,
                    },
                ),
                title: "Literature source 2".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // LiteratureSourcesCitationHyperlink
                address: 0,
                parent: Some(
                    // Variant::LiteratureSourcesCitation,
                    Parent {
                        address: 1,
                        variety: Rank::LiteratureSourcesCitation,
                    },
                ),
                body: Body {
                    phrase: "Literature link source 2".to_string(),
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // LiteratureSourcesCitationDetails
                address: 1,
                parent: Some(
                    // Variant::LiteratureSourcesCitation,
                    Parent {
                        address: 1,
                        variety: Rank::LiteratureSourcesCitation,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Literature Source 2 details".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Resources
                address: 11,
                parent: None,
                title: "Resources".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Resources - Existing
                address: 0,
                parent: Some(
                    // Variant::Resources,
                    Parent {
                        address: 11,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Existing Resources note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Resources - Additional
                address: 1,
                parent: Some(
                    // Variant::Resources,
                    Parent {
                        address: 11,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Additional Resources note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Budget
                address: 12,
                parent: None,
                title: "Budget Estimates".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Budget - Personnel
                address: 0,
                parent: Some(
                    // Variant::Budget,
                    Parent {
                        address: 12,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Budget Personnel note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Budget - Facilities
                address: 1,
                parent: Some(
                    // Variant::Budget,
                    Parent {
                        address: 12,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Budget Facilities note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Budget - Materials
                address: 2,
                parent: Some(
                    // Variant::Budget,
                    Parent {
                        address: 12,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Budget Materials note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Budget - Workshops
                address: 3,
                parent: Some(
                    // Variant::Budget,
                    Parent {
                        address: 12,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Budget Workshops note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Budget - Overheads
                address: 4,
                parent: Some(
                    // Variant::Budget,
                    Parent {
                        address: 12,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Budget Overheads note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Budget - Miscellaneous
                address: 5,
                parent: Some(
                    // Variant::Budget,
                    Parent {
                        address: 12,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Budget Miscellaneous note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Attachments
                address: 13,
                parent: None,
                title: "Attachments".to_string(),
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Attachments - Tables
                address: 0,
                parent: Some(
                    // Variant::Attachments,
                    Parent {
                        address: 13,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Attachments Tables note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Attachments - Figures
                address: 1,
                parent: Some(
                    // Variant::Attachments,
                    Parent {
                        address: 13,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Attachments Figures note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
            // nn
            Chunk {
                // Attachments - Other
                address: 2,
                parent: Some(
                    // Variant::Attachments,
                    Parent {
                        address: 13,
                        variety: Rank::Top,
                    },
                ),
                body: Body {
                    notes: Notes {
                        notes: vec![Note {
                            note: "Attachments Other note 1".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    ..Default::default()
                },
                visible: true,
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    // Chapter counter.
    let mut nn: usize;
    let mut parsed = crate::workbook::app::Workbook::default();

    parsed.project = (*test).clone().into();

    // nn = 0;

    assert_eq!(test.owner, parsed.project.owner);
    assert_eq!(test.record, parsed.project.record);
    assert_eq!(test.resolution, parsed.project.resolution);

    assert_eq!(test.chunks[0].address, 0);
    assert_eq!(test.chunks[0].parent, None);
    assert_eq!(test.chunks[0].visible, parsed.project.working_name.visible);
    assert_eq!(test.chunks[0].title, parsed.project.working_name.title);
    assert_eq!(
        "Test project name".to_string(),
        parsed.project.working_name.title
    );

    assert_eq!(test.chunks[1].address, 0);
    assert_eq!(
        test.chunks[1].parent,
        Some(Parent {
            address: 0,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[1].body.notes.notes.len(),
        parsed.project.working_name.options.notes.len()
    );
    assert_eq!(
        test.chunks[1].body.notes.notes[0].visible,
        parsed.project.working_name.options.notes[0].visible
    );
    assert_eq!(
        test.chunks[1].body.notes.notes[0].note,
        parsed.project.working_name.options.notes[0].note
    );
    assert_eq!(
        "Name option 1".to_string(),
        parsed.project.working_name.options.notes[0].note
    );

    assert_eq!(test.chunks[2].address, 1);
    assert_eq!(test.chunks[2].parent, None);
    assert_eq!(test.chunks[2].visible, parsed.project.idea.visible);
    assert_eq!(test.chunks[2].title, parsed.project.idea.title);
    assert_eq!("Idea".to_string(), parsed.project.idea.title);

    assert_eq!(test.chunks[3].address, 0);
    assert_eq!(
        test.chunks[3].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[3].body.notes.notes.len(),
        parsed.project.idea.problem.notes.len()
    );
    assert_eq!(
        test.chunks[3].body.notes.notes[0].visible,
        parsed.project.idea.problem.notes[0].visible
    );
    assert_eq!(
        test.chunks[3].body.notes.notes[0].note,
        parsed.project.idea.problem.notes[0].note
    );
    assert_eq!(
        "Problem note 1".to_string(),
        parsed.project.idea.problem.notes[0].note
    );

    assert_eq!(test.chunks[4].address, 1);
    assert_eq!(
        test.chunks[4].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[4].body.notes.notes.len(),
        parsed.project.idea.hypothesis.notes.len()
    );
    assert_eq!(
        test.chunks[4].body.notes.notes[0].visible,
        parsed.project.idea.hypothesis.notes[0].visible
    );
    assert_eq!(
        test.chunks[4].body.notes.notes[0].note,
        parsed.project.idea.hypothesis.notes[0].note
    );
    assert_eq!(
        "Hypothesis note 1".to_string(),
        parsed.project.idea.hypothesis.notes[0].note
    );

    assert_eq!(test.chunks[5].address, 2);
    assert_eq!(
        test.chunks[5].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[5].body.notes.notes.len(),
        parsed.project.idea.summary.notes.len()
    );
    assert_eq!(
        test.chunks[5].body.notes.notes[0].visible,
        parsed.project.idea.summary.notes[0].visible
    );
    assert_eq!(
        test.chunks[5].body.notes.notes[0].note,
        parsed.project.idea.summary.notes[0].note
    );
    assert_eq!(
        "Abstract note 1".to_string(),
        parsed.project.idea.summary.notes[0].note
    );

    assert_eq!(test.chunks[6].address, 3);
    assert_eq!(
        test.chunks[6].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[6].body.notes.notes.len(),
        parsed.project.idea.description.notes.len()
    );
    assert_eq!(
        test.chunks[6].body.notes.notes[0].visible,
        parsed.project.idea.description.notes[0].visible
    );
    assert_eq!(
        test.chunks[6].body.notes.notes[0].note,
        parsed.project.idea.description.notes[0].note
    );
    assert_eq!(
        "Project description note 1".to_string(),
        parsed.project.idea.description.notes[0].note
    );

    assert_eq!(test.chunks[7].address, 4);
    assert_eq!(
        test.chunks[7].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[7].visible,
        parsed.project.idea.references.visible
    );
    assert_eq!(test.chunks[7].title, parsed.project.idea.references.title);
    assert_eq!(
        "Key References".to_string(),
        parsed.project.idea.references.title
    );

    assert_eq!(test.chunks[8].address, 0);
    assert_eq!(
        test.chunks[8].parent,
        Some(Parent {
            address: 4,
            variety: Rank::IdeaReferences
        })
    );
    assert_eq!(
        test.chunks[8].visible,
        parsed.project.idea.references.references[0].visible
    );
    assert_eq!(
        test.chunks[8].title,
        parsed.project.idea.references.references[0].title
    );
    assert_eq!(
        "Key source 1".to_string(),
        parsed.project.idea.references.references[0].title
    );

    assert_eq!(test.chunks[9].address, 0);
    assert_eq!(
        test.chunks[9].parent,
        Some(Parent {
            address: 0,
            variety: Rank::IdeaReferencesCitation
        })
    );
    assert_eq!(
        test.chunks[9].body.phrase,
        parsed.project.idea.references.references[0].hyperlink
    );
    assert_eq!(
        "link source 1".to_string(),
        parsed.project.idea.references.references[0].hyperlink
    );

    assert_eq!(test.chunks[10].address, 1);
    assert_eq!(
        test.chunks[10].parent,
        Some(Parent {
            address: 0,
            variety: Rank::IdeaReferencesCitation
        })
    );
    assert_eq!(
        test.chunks[10].body.notes.notes[0].visible,
        parsed.project.idea.references.references[0]
            .source_details
            .notes[0]
            .visible
    );
    assert_eq!(
        test.chunks[10].body.notes.notes[0].note,
        parsed.project.idea.references.references[0]
            .source_details
            .notes[0]
            .note
    );
    assert_eq!(
        "Source 1 details".to_string(),
        parsed.project.idea.references.references[0]
            .source_details
            .notes[0]
            .note
    );

    assert_eq!(test.chunks[11].address, 1);
    assert_eq!(
        test.chunks[11].parent,
        Some(Parent {
            address: 4,
            variety: Rank::IdeaReferences
        })
    );
    assert_eq!(
        test.chunks[11].visible,
        parsed.project.idea.references.references[1].visible
    );
    assert_eq!(
        test.chunks[11].title,
        parsed.project.idea.references.references[1].title
    );
    assert_eq!(
        "Key source 2".to_string(),
        parsed.project.idea.references.references[1].title
    );

    assert_eq!(test.chunks[12].address, 0);
    assert_eq!(
        test.chunks[12].parent,
        Some(Parent {
            address: 1,
            variety: Rank::IdeaReferencesCitation
        })
    );
    assert_eq!(
        test.chunks[12].body.phrase,
        parsed.project.idea.references.references[1].hyperlink
    );
    assert_eq!(
        "link source 2".to_string(),
        parsed.project.idea.references.references[1].hyperlink
    );

    assert_eq!(test.chunks[13].address, 1);
    assert_eq!(
        test.chunks[13].parent,
        Some(Parent {
            address: 1,
            variety: Rank::IdeaReferencesCitation
        })
    );
    assert_eq!(
        test.chunks[13].body.notes.notes[0].visible,
        parsed.project.idea.references.references[1]
            .source_details
            .notes[0]
            .visible
    );
    assert_eq!(
        test.chunks[13].body.notes.notes[0].note,
        parsed.project.idea.references.references[1]
            .source_details
            .notes[0]
            .note
    );
    assert_eq!(
        "Source 2 details".to_string(),
        parsed.project.idea.references.references[1]
            .source_details
            .notes[0]
            .note
    );

    assert_eq!(test.chunks[14].address, 2);
    assert_eq!(test.chunks[14].parent, None);
    assert_eq!(test.chunks[14].visible, parsed.project.funding.visible);
    assert_eq!(test.chunks[14].title, parsed.project.funding.title);
    assert_eq!("Funding".to_string(), parsed.project.funding.title);

    assert_eq!(test.chunks[15].address, 0);
    assert_eq!(
        test.chunks[15].parent,
        Some(Parent {
            address: 2,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[15].visible,
        parsed.project.funding.funding_options[0].visible
    );
    assert_eq!(
        test.chunks[15].title,
        parsed.project.funding.funding_options[0].title
    );
    assert_eq!(
        "Funding Option 1".to_string(),
        parsed.project.funding.funding_options[0].title
    );

    assert_eq!(test.chunks[16].address, 0);
    assert_eq!(
        test.chunks[16].parent,
        Some(Parent {
            address: 0,
            variety: Rank::FundingProgram
        })
    );
    assert_eq!(
        test.chunks[16].body.notes.notes[0].visible,
        parsed.project.funding.funding_options[0].annotation.notes[0].visible
    );
    assert_eq!(
        test.chunks[16].body.notes.notes[0].note,
        parsed.project.funding.funding_options[0].annotation.notes[0].note
    );
    assert_eq!(
        "Funding Option 1 annotation".to_string(),
        parsed.project.funding.funding_options[0].annotation.notes[0].note
    );

    assert_eq!(test.chunks[17].address, 1);
    assert_eq!(
        test.chunks[17].parent,
        Some(Parent {
            address: 0,
            variety: Rank::FundingProgram
        })
    );
    assert_eq!(
        test.chunks[17].body.phrase,
        parsed.project.funding.funding_options[0].hyperlink
    );
    assert_eq!(
        "link funding option 1".to_string(),
        parsed.project.funding.funding_options[0].hyperlink
    );

    assert_eq!(test.chunks[18].address, 2);
    assert_eq!(
        test.chunks[18].parent,
        Some(Parent {
            address: 0,
            variety: Rank::FundingProgram
        })
    );
    assert_eq!(
        test.chunks[18].body.date,
        parsed.project.funding.funding_options[0]
            .deadline
            .clone()
            .into()
    );
    assert_eq!(
        2025,
        parsed.project.funding.funding_options[0].deadline.year
    );

    assert_eq!(test.chunks[19].address, 1);
    assert_eq!(
        test.chunks[19].parent,
        Some(Parent {
            address: 2,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[19].visible,
        parsed.project.funding.funding_options[1].visible
    );
    assert_eq!(
        test.chunks[19].title,
        parsed.project.funding.funding_options[1].title
    );
    assert_eq!(
        "Funding Option 2".to_string(),
        parsed.project.funding.funding_options[1].title
    );

    /*
    // TODO: Funding Option 2 - Details: annotation, hyperlink and deadline
    */

    assert_eq!(test.chunks[20].address, 3);
    assert_eq!(test.chunks[20].parent, None);
    assert_eq!(test.chunks[20].visible, parsed.project.scope.visible);
    assert_eq!(test.chunks[20].title, parsed.project.scope.title);
    assert_eq!(
        "Project Scope: Objectives and Planned Activities".to_string(),
        parsed.project.scope.title
    );

    assert_eq!(test.chunks[21].address, 0);
    assert_eq!(
        test.chunks[21].parent,
        Some(Parent {
            address: 3,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[21].body.notes.notes.len(),
        parsed.project.scope.suggested_tasks.notes.len()
    );
    assert_eq!(
        test.chunks[21].body.notes.notes[0].visible,
        parsed.project.scope.suggested_tasks.notes[0].visible
    );
    assert_eq!(
        test.chunks[21].body.notes.notes[0].note,
        parsed.project.scope.suggested_tasks.notes[0].note
    );
    assert_eq!(
        "Suggested Tasks note 1".to_string(),
        parsed.project.scope.suggested_tasks.notes[0].note
    );

    assert_eq!(test.chunks[22].address, 1);
    assert_eq!(
        test.chunks[22].parent,
        Some(Parent {
            address: 3,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[22].body.notes.notes.len(),
        parsed.project.scope.objectives.notes.len()
    );
    assert_eq!(
        test.chunks[22].body.notes.notes[0].visible,
        parsed.project.scope.objectives.notes[0].visible
    );
    assert_eq!(
        test.chunks[22].body.notes.notes[0].note,
        parsed.project.scope.objectives.notes[0].note
    );
    assert_eq!(
        "Objectives note 1".to_string(),
        parsed.project.scope.objectives.notes[0].note
    );

    assert_eq!(test.chunks[23].address, 2);
    assert_eq!(
        test.chunks[23].parent,
        Some(Parent {
            address: 3,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[23].body.notes.notes.len(),
        parsed.project.scope.activities.notes.len()
    );
    assert_eq!(
        test.chunks[23].body.notes.notes[0].visible,
        parsed.project.scope.activities.notes[0].visible
    );
    assert_eq!(
        test.chunks[23].body.notes.notes[0].note,
        parsed.project.scope.activities.notes[0].note
    );
    assert_eq!(
        "Activities note 1".to_string(),
        parsed.project.scope.activities.notes[0].note
    );

    assert_eq!(test.chunks[24].address, 3);
    assert_eq!(
        test.chunks[24].parent,
        Some(Parent {
            address: 3,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[24].body.notes.notes.len(),
        parsed.project.scope.work_plan.notes.len()
    );
    assert_eq!(
        test.chunks[24].body.notes.notes[0].visible,
        parsed.project.scope.work_plan.notes[0].visible
    );
    assert_eq!(
        test.chunks[24].body.notes.notes[0].note,
        parsed.project.scope.work_plan.notes[0].note
    );
    assert_eq!(
        "Work Plan note 1".to_string(),
        parsed.project.scope.work_plan.notes[0].note
    );

    assert_eq!(test.chunks[25].address, 4);
    assert_eq!(
        test.chunks[25].parent,
        Some(Parent {
            address: 3,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[25].body.notes.notes.len(),
        parsed.project.scope.tasks.notes.len()
    );
    assert_eq!(
        test.chunks[25].body.notes.notes[0].visible,
        parsed.project.scope.tasks.notes[0].visible
    );
    assert_eq!(
        test.chunks[25].body.notes.notes[0].note,
        parsed.project.scope.tasks.notes[0].note
    );
    assert_eq!(
        "Tasks note 1".to_string(),
        parsed.project.scope.tasks.notes[0].note
    );

    assert_eq!(test.chunks[26].address, 4);
    assert_eq!(test.chunks[26].parent, None);
    assert_eq!(test.chunks[26].visible, parsed.project.timeline.visible);
    assert_eq!(test.chunks[26].title, parsed.project.timeline.title);
    assert_eq!("Timeline".to_string(), parsed.project.timeline.title);

    assert_eq!(test.chunks[27].address, 0);
    assert_eq!(
        test.chunks[27].parent,
        Some(Parent {
            address: 4,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[27].body.date.year,
        parsed.project.timeline.project_start.year
    ); // default date
    assert_eq!(
        test.chunks[27].body.date.month,
        parsed.project.timeline.project_start.month
    ); // default date
    assert_eq!(
        test.chunks[27].body.date.day,
        parsed.project.timeline.project_start.day
    ); // default date

    assert_eq!(test.chunks[28].address, 1);
    assert_eq!(
        test.chunks[28].parent,
        Some(Parent {
            address: 4,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[28].body.numeral,
        parsed.project.timeline.duration_years
    );
    assert_eq!(4.5, parsed.project.timeline.duration_years);

    assert_eq!(test.chunks[29].address, 2);
    assert_eq!(
        test.chunks[29].parent,
        Some(Parent {
            address: 4,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[29].body.notes.notes.len(),
        parsed.project.timeline.milestones.notes.len()
    );
    assert_eq!(
        test.chunks[29].body.notes.notes[0].visible,
        parsed.project.timeline.milestones.notes[0].visible
    );
    assert_eq!(
        test.chunks[29].body.notes.notes[0].note,
        parsed.project.timeline.milestones.notes[0].note
    );
    assert_eq!(
        "Milestones note 1".to_string(),
        parsed.project.timeline.milestones.notes[0].note
    );

    assert_eq!(test.chunks[30].address, 5);
    assert_eq!(test.chunks[30].parent, None);
    assert_eq!(test.chunks[30].visible, parsed.project.scripting.visible);
    assert_eq!(test.chunks[30].title, parsed.project.scripting.title);
    assert_eq!(
        "Who Will Write The Project Proposal".to_string(),
        parsed.project.scripting.title
    );

    assert_eq!(test.chunks[31].address, 0);
    assert_eq!(
        test.chunks[31].parent,
        Some(Parent {
            address: 5,
            variety: Rank::Top
        })
    );
    assert_eq!(test.chunks[31].body.option, parsed.project.scripting.answer);
    assert_eq!(test.chunks[31].body.option, ScriptingOption::Jointly);

    assert_eq!(test.chunks[32].address, 1);
    assert_eq!(
        test.chunks[32].parent,
        Some(Parent {
            address: 5,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[32].body.notes.notes.len(),
        parsed.project.scripting.scripting.notes.len()
    );
    assert_eq!(
        test.chunks[32].body.notes.notes[0].visible,
        parsed.project.scripting.scripting.notes[0].visible
    );
    assert_eq!(
        test.chunks[32].body.notes.notes[0].note,
        parsed.project.scripting.scripting.notes[0].note
    );
    assert_eq!(
        "Scripting Option note 1".to_string(),
        parsed.project.scripting.scripting.notes[0].note
    );

    assert_eq!(test.chunks[33].address, 6);
    assert_eq!(test.chunks[33].parent, None);
    assert_eq!(test.chunks[33].visible, parsed.project.team.visible);
    assert_eq!(test.chunks[33].title, parsed.project.team.title);
    assert_eq!("Team".to_string(), parsed.project.team.title);

    assert_eq!(test.chunks[34].address, 0);
    assert_eq!(
        test.chunks[34].parent,
        Some(Parent {
            address: 6,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[34].body.notes.notes.len(),
        parsed.project.team.proposed_partners.notes.len()
    );
    assert_eq!(
        test.chunks[34].body.notes.notes[0].visible,
        parsed.project.team.proposed_partners.notes[0].visible
    );
    assert_eq!(
        test.chunks[34].body.notes.notes[0].note,
        parsed.project.team.proposed_partners.notes[0].note
    );
    assert_eq!(
        "Proposed Partners note 1".to_string(),
        parsed.project.team.proposed_partners.notes[0].note
    );

    assert_eq!(test.chunks[35].address, 1);
    assert_eq!(
        test.chunks[35].parent,
        Some(Parent {
            address: 6,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[35].body.notes.notes.len(),
        parsed.project.team.project_leader.notes.len()
    );
    assert_eq!(
        test.chunks[35].body.notes.notes[0].visible,
        parsed.project.team.project_leader.notes[0].visible
    );
    assert_eq!(
        test.chunks[35].body.notes.notes[0].note,
        parsed.project.team.project_leader.notes[0].note
    );
    assert_eq!(
        "Project Leader note 1".to_string(),
        parsed.project.team.project_leader.notes[0].note
    );

    assert_eq!(test.chunks[36].address, 2);
    assert_eq!(
        test.chunks[36].parent,
        Some(Parent {
            address: 6,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[36].body.notes.notes.len(),
        parsed.project.team.industrial_partners.notes.len()
    );
    assert_eq!(
        test.chunks[36].body.notes.notes[0].visible,
        parsed.project.team.industrial_partners.notes[0].visible
    );
    assert_eq!(
        test.chunks[36].body.notes.notes[0].note,
        parsed.project.team.industrial_partners.notes[0].note
    );
    assert_eq!(
        "Industrial Partners note 1".to_string(),
        parsed.project.team.industrial_partners.notes[0].note
    );

    assert_eq!(test.chunks[37].address, 3);
    assert_eq!(
        test.chunks[37].parent,
        Some(Parent {
            address: 6,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[37].body.notes.notes.len(),
        parsed.project.team.proponents.notes.len()
    );
    assert_eq!(
        test.chunks[37].body.notes.notes[0].visible,
        parsed.project.team.proponents.notes[0].visible
    );
    assert_eq!(
        test.chunks[37].body.notes.notes[0].note,
        parsed.project.team.proponents.notes[0].note
    );
    assert_eq!(
        "Proponents note 1".to_string(),
        parsed.project.team.proponents.notes[0].note
    );

    assert_eq!(test.chunks[38].address, 0);
    assert_eq!(
        test.chunks[38].parent,
        Some(Parent {
            address: 4,
            variety: Rank::TeamParticipants
        })
    );
    assert_eq!(
        test.chunks[38].visible,
        parsed.project.team.participants[0].visible
    );
    assert_eq!(
        test.chunks[38].title,
        parsed.project.team.participants[0].name
    );
    assert_eq!(
        "Partner 1 Name".to_string(),
        parsed.project.team.participants[0].name
    );

    assert_eq!(test.chunks[39].address, 0);
    assert_eq!(
        test.chunks[39].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[39].body.phrase,
        parsed.project.team.participants[0].role
    );
    assert_eq!(
        "Partner 1 role".to_string(),
        parsed.project.team.participants[0].role
    );

    assert_eq!(test.chunks[40].address, 1);
    assert_eq!(
        test.chunks[40].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[40].body.phrase,
        parsed.project.team.participants[0].affiliation
    );
    assert_eq!(
        "Partner 1 affiliation".to_string(),
        parsed.project.team.participants[0].affiliation
    );

    assert_eq!(test.chunks[41].address, 2);
    assert_eq!(
        test.chunks[41].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[41].body.phrase,
        parsed.project.team.participants[0].hyperlink
    );
    assert_eq!(
        "Partner 1 hyperlink".to_string(),
        parsed.project.team.participants[0].hyperlink
    );

    assert_eq!(test.chunks[42].address, 3);
    assert_eq!(
        test.chunks[42].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[42].body.phrase,
        parsed.project.team.participants[0].country
    );
    assert_eq!(
        "Partner 1 country".to_string(),
        parsed.project.team.participants[0].country
    );

    assert_eq!(test.chunks[43].address, 4);
    assert_eq!(
        test.chunks[43].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[43].body.phrase,
        parsed.project.team.participants[0].expertise
    );
    assert_eq!(
        "Partner 1 expertise".to_string(),
        parsed.project.team.participants[0].expertise
    );

    assert_eq!(test.chunks[44].address, 5);
    assert_eq!(
        test.chunks[44].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[44].body.notes.notes.len(),
        parsed.project.team.participants[0].contribution.notes.len()
    );
    assert_eq!(
        test.chunks[44].body.notes.notes[0].visible,
        parsed.project.team.participants[0].contribution.notes[0].visible
    );
    assert_eq!(
        test.chunks[44].body.notes.notes[0].note,
        parsed.project.team.participants[0].contribution.notes[0].note
    );
    assert_eq!(
        "Partner 1 contribution note 1".to_string(),
        parsed.project.team.participants[0].contribution.notes[0].note
    );

    assert_eq!(test.chunks[45].address, 6);
    assert_eq!(
        test.chunks[45].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[45].body.notes.notes.len(),
        parsed.project.team.participants[0].team.notes.len()
    );
    assert_eq!(
        test.chunks[45].body.notes.notes[0].visible,
        parsed.project.team.participants[0].team.notes[0].visible
    );
    assert_eq!(
        test.chunks[45].body.notes.notes[0].note,
        parsed.project.team.participants[0].team.notes[0].note
    );
    assert_eq!(
        "Partner 1 team note 1".to_string(),
        parsed.project.team.participants[0].team.notes[0].note
    );

    assert_eq!(test.chunks[46].address, 7);
    assert_eq!(
        test.chunks[46].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[46].body.notes.notes.len(),
        parsed.project.team.participants[0].cv.notes.len()
    );
    assert_eq!(
        test.chunks[46].body.notes.notes[0].visible,
        parsed.project.team.participants[0].cv.notes[0].visible
    );
    assert_eq!(
        test.chunks[46].body.notes.notes[0].note,
        parsed.project.team.participants[0].cv.notes[0].note
    );
    assert_eq!(
        "Partner 1 CV note 1".to_string(),
        parsed.project.team.participants[0].cv.notes[0].note
    );

    assert_eq!(test.chunks[47].address, 8);
    assert_eq!(
        test.chunks[47].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[47].body.notes.notes.len(),
        parsed.project.team.participants[0].resources.notes.len()
    );
    assert_eq!(
        test.chunks[47].body.notes.notes[0].visible,
        parsed.project.team.participants[0].resources.notes[0].visible
    );
    assert_eq!(
        test.chunks[47].body.notes.notes[0].note,
        parsed.project.team.participants[0].resources.notes[0].note
    );
    assert_eq!(
        "Partner 1 resources note 1".to_string(),
        parsed.project.team.participants[0].resources.notes[0].note
    );

    assert_eq!(test.chunks[48].address, 9);
    assert_eq!(
        test.chunks[48].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[48].body.notes.notes.len(),
        parsed.project.team.participants[0].budget.notes.len()
    );
    assert_eq!(
        test.chunks[48].body.notes.notes[0].visible,
        parsed.project.team.participants[0].budget.notes[0].visible
    );
    assert_eq!(
        test.chunks[48].body.notes.notes[0].note,
        parsed.project.team.participants[0].budget.notes[0].note
    );
    assert_eq!(
        "Partner 1 budget note 1".to_string(),
        parsed.project.team.participants[0].budget.notes[0].note
    );

    nn = 48;
    nn += 1;

    assert_eq!(test.chunks[nn].address, 1);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 4,
            variety: Rank::TeamParticipants
        })
    );
    assert_eq!(
        test.chunks[nn].visible,
        parsed.project.team.participants[1].visible
    );
    assert_eq!(
        test.chunks[nn].title,
        parsed.project.team.participants[1].name
    );
    assert_eq!(
        "Partner 2 Name".to_string(),
        parsed.project.team.participants[1].name
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.phrase,
        parsed.project.team.participants[1].role
    );
    assert_eq!(
        "Partner 2 role".to_string(),
        parsed.project.team.participants[1].role
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 1);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.phrase,
        parsed.project.team.participants[1].affiliation
    );
    assert_eq!(
        "Partner 2 affiliation".to_string(),
        parsed.project.team.participants[1].affiliation
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 2);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.phrase,
        parsed.project.team.participants[1].hyperlink
    );
    assert_eq!(
        "Partner 2 hyperlink".to_string(),
        parsed.project.team.participants[1].hyperlink
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 3);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.phrase,
        parsed.project.team.participants[1].country
    );
    assert_eq!(
        "Partner 2 country".to_string(),
        parsed.project.team.participants[1].country
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 4);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.phrase,
        parsed.project.team.participants[1].expertise
    );
    assert_eq!(
        "Partner 2 expertise".to_string(),
        parsed.project.team.participants[1].expertise
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 5);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.team.participants[1].contribution.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.team.participants[1].contribution.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.team.participants[1].contribution.notes[0].note
    );
    assert_eq!(
        "Partner 2 contribution note 1".to_string(),
        parsed.project.team.participants[1].contribution.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 6);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.team.participants[1].team.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.team.participants[1].team.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.team.participants[1].team.notes[0].note
    );
    assert_eq!(
        "Partner 2 team note 1".to_string(),
        parsed.project.team.participants[1].team.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 7);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.team.participants[1].cv.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.team.participants[1].cv.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.team.participants[1].cv.notes[0].note
    );
    assert_eq!(
        "Partner 2 CV note 1".to_string(),
        parsed.project.team.participants[1].cv.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 8);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.team.participants[1].resources.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.team.participants[1].resources.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.team.participants[1].resources.notes[0].note
    );
    assert_eq!(
        "Partner 2 resources note 1".to_string(),
        parsed.project.team.participants[1].resources.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 9);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.team.participants[1].budget.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.team.participants[1].budget.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.team.participants[1].budget.notes[0].note
    );
    assert_eq!(
        "Partner 2 budget note 1".to_string(),
        parsed.project.team.participants[1].budget.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 7);
    assert_eq!(test.chunks[nn].parent, None);
    assert_eq!(
        test.chunks[nn].visible,
        parsed.project.prelim_results.visible
    );
    assert_eq!(test.chunks[nn].title, parsed.project.prelim_results.title);
    assert_eq!(
        "Preliminary Results".to_string(),
        parsed.project.prelim_results.title
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 7,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.prelim_results.prelim_results.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.prelim_results.prelim_results.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.prelim_results.prelim_results.notes[0].note
    );
    assert_eq!(
        "Preliminary Results note 1".to_string(),
        parsed.project.prelim_results.prelim_results.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 8);
    assert_eq!(test.chunks[nn].parent, None);
    assert_eq!(test.chunks[nn].visible, parsed.project.methodology.visible);
    assert_eq!(test.chunks[nn].title, parsed.project.methodology.title);
    assert_eq!(
        "Research Methodology".to_string(),
        parsed.project.methodology.title
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 8,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.methodology.methodology.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.methodology.methodology.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.methodology.methodology.notes[0].note
    );
    assert_eq!(
        "Methodology note 1".to_string(),
        parsed.project.methodology.methodology.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 9);
    assert_eq!(test.chunks[nn].parent, None);
    assert_eq!(test.chunks[nn].visible, parsed.project.outcomes.visible);
    assert_eq!(test.chunks[nn].title, parsed.project.outcomes.title);
    assert_eq!(
        "Expected Results, Impact and Dissemination".to_string(),
        parsed.project.outcomes.title
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 9,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.outcomes.results.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.outcomes.results.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.outcomes.results.notes[0].note
    );
    assert_eq!(
        "Expected Results note 1".to_string(),
        parsed.project.outcomes.results.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 1);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 9,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.outcomes.impact.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.outcomes.impact.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.outcomes.impact.notes[0].note
    );
    assert_eq!(
        "Impact note 1".to_string(),
        parsed.project.outcomes.impact.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 2);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 9,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.outcomes.propagation.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.outcomes.propagation.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.outcomes.propagation.notes[0].note
    );
    assert_eq!(
        "Propagation note 1".to_string(),
        parsed.project.outcomes.propagation.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 10);
    assert_eq!(test.chunks[nn].parent, None);
    assert_eq!(test.chunks[nn].visible, parsed.project.literature.visible);
    assert_eq!(test.chunks[nn].title, parsed.project.literature.title);
    assert_eq!(
        "Literature Survey".to_string(),
        parsed.project.literature.title
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 10,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.literature.literature_survey.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.literature.literature_survey.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.literature.literature_survey.notes[0].note
    );
    assert_eq!(
        "Literature Survey note 1".to_string(),
        parsed.project.literature.literature_survey.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::LiteratureSources
        })
    );
    assert_eq!(
        test.chunks[nn].visible,
        parsed.project.literature.references[0].visible
    );
    assert_eq!(
        test.chunks[nn].title,
        parsed.project.literature.references[0].title
    );
    assert_eq!(
        "Literature source 1".to_string(),
        parsed.project.literature.references[0].title
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::LiteratureSourcesCitation
        })
    );
    assert_eq!(
        test.chunks[nn].body.phrase,
        parsed.project.literature.references[0].hyperlink
    );
    assert_eq!(
        "Literature link source 1".to_string(),
        parsed.project.literature.references[0].hyperlink
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 1);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::LiteratureSourcesCitation
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.literature.references[0].source_details.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.literature.references[0].source_details.notes[0].note
    );
    assert_eq!(
        "Literature Source 1 details".to_string(),
        parsed.project.literature.references[0].source_details.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 1);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::LiteratureSources
        })
    );
    assert_eq!(
        test.chunks[nn].visible,
        parsed.project.literature.references[1].visible
    );
    assert_eq!(
        test.chunks[nn].title,
        parsed.project.literature.references[1].title
    );
    assert_eq!(
        "Literature source 2".to_string(),
        parsed.project.literature.references[1].title
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::LiteratureSourcesCitation
        })
    );
    assert_eq!(
        test.chunks[nn].body.phrase,
        parsed.project.literature.references[1].hyperlink
    );
    assert_eq!(
        "Literature link source 2".to_string(),
        parsed.project.literature.references[1].hyperlink
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 1);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::LiteratureSourcesCitation
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.literature.references[1].source_details.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.literature.references[1].source_details.notes[0].note
    );
    assert_eq!(
        "Literature Source 2 details".to_string(),
        parsed.project.literature.references[1].source_details.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 11);
    assert_eq!(test.chunks[nn].parent, None);
    assert_eq!(test.chunks[nn].visible, parsed.project.resources.visible);
    assert_eq!(test.chunks[nn].title, parsed.project.resources.title);
    assert_eq!("Resources".to_string(), parsed.project.resources.title);

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 11,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.resources.existing.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.resources.existing.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.resources.existing.notes[0].note
    );
    assert_eq!(
        "Existing Resources note 1".to_string(),
        parsed.project.resources.existing.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 1);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 11,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.resources.further.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.resources.further.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.resources.further.notes[0].note
    );
    assert_eq!(
        "Additional Resources note 1".to_string(),
        parsed.project.resources.further.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 12);
    assert_eq!(test.chunks[nn].parent, None);
    assert_eq!(test.chunks[nn].visible, parsed.project.budget.visible);
    assert_eq!(test.chunks[nn].title, parsed.project.budget.title);
    assert_eq!("Budget Estimates".to_string(), parsed.project.budget.title);

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.budget.personnel.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.budget.personnel.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.budget.personnel.notes[0].note
    );
    assert_eq!(
        "Budget Personnel note 1".to_string(),
        parsed.project.budget.personnel.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 1);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.budget.facilities.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.budget.facilities.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.budget.facilities.notes[0].note
    );
    assert_eq!(
        "Budget Facilities note 1".to_string(),
        parsed.project.budget.facilities.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 2);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.budget.materials.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.budget.materials.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.budget.materials.notes[0].note
    );
    assert_eq!(
        "Budget Materials note 1".to_string(),
        parsed.project.budget.materials.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 3);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.budget.workshops.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.budget.workshops.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.budget.workshops.notes[0].note
    );
    assert_eq!(
        "Budget Workshops note 1".to_string(),
        parsed.project.budget.workshops.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 4);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.budget.overheads.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.budget.overheads.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.budget.overheads.notes[0].note
    );
    assert_eq!(
        "Budget Overheads note 1".to_string(),
        parsed.project.budget.overheads.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 5);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.budget.misc.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.budget.misc.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.budget.misc.notes[0].note
    );
    assert_eq!(
        "Budget Miscellaneous note 1".to_string(),
        parsed.project.budget.misc.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 13);
    assert_eq!(test.chunks[nn].parent, None);
    assert_eq!(test.chunks[nn].visible, parsed.project.attachments.visible);
    assert_eq!(test.chunks[nn].title, parsed.project.attachments.title);
    assert_eq!("Attachments".to_string(), parsed.project.attachments.title);

    nn += 1;

    assert_eq!(test.chunks[nn].address, 0);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 13,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.attachments.tables.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.attachments.tables.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.attachments.tables.notes[0].note
    );
    assert_eq!(
        "Attachments Tables note 1".to_string(),
        parsed.project.attachments.tables.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 1);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 13,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.attachments.figures.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.attachments.figures.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.attachments.figures.notes[0].note
    );
    assert_eq!(
        "Attachments Figures note 1".to_string(),
        parsed.project.attachments.figures.notes[0].note
    );

    nn += 1;

    assert_eq!(test.chunks[nn].address, 2);
    assert_eq!(
        test.chunks[nn].parent,
        Some(Parent {
            address: 13,
            variety: Rank::Top
        })
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes.len(),
        parsed.project.attachments.other.notes.len()
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].visible,
        parsed.project.attachments.other.notes[0].visible
    );
    assert_eq!(
        test.chunks[nn].body.notes.notes[0].note,
        parsed.project.attachments.other.notes[0].note
    );
    assert_eq!(
        "Attachments Other note 1".to_string(),
        parsed.project.attachments.other.notes[0].note
    );
}
