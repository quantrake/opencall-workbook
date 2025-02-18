use crate::workbook::chapter::Variety;
use crate::workbook::note::Notes;
use crate::workbook::project::Project;
use crate::workbook::sections::{
    budget::SectionBudget,
    idea::SectionIdea,
    scope::SectionScope,
    team::SectionTeam,
    timeline::{Date, SectionTimeline},
};

use super::v_b0004::{Association, Body, Chunk, Parent, Rank, Store, YearMonthDay};

pub const DATA_FORMAT_VERSION: &str = super::v_b0004::DATA_FORMAT_VERSION;

trait Stock {
    fn stock(&self) -> Self;
}

impl From<Date> for YearMonthDay {
    fn from(val: Date) -> Self {
        YearMonthDay {
            year: val.year,
            month: val.month,
            day: val.day,
        }
    }
}

impl From<Project> for Store {
    fn from(val: Project) -> Self {
        // Use the association array, not the `&self.index_list`,
        // so that the order of sections be relatively stable
        // and the mapping doesn't depend on the order in which
        // sections appear in workbook.
        let sections = Association::default().sections;
        let mut storage = Store {
            format: DATA_FORMAT_VERSION.to_string(),
            owner: val.owner.clone(),
            record: val.record,
            resolution: val.resolution,
            chunks: Vec::new(),
        };

        let empty_elem = Chunk::default();
        let element = &mut Chunk::default();

        for (i, section) in sections.iter().enumerate() {
            match section {
                Variety::WorkingName => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // WorkingName
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.working_name.title.clone(),
                        visible: val.working_name.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // WorkingNameOptions
                    storage.chunks.push(Chunk {
                        address: 1,
                        parent: Some(
                            // WorkingName
                            // Variant::WorkingName,
                            Parent {
                                address: 0,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.working_name.options.stock(),
                            ..Default::default()
                        },
                        visible: val.working_name.visible,
                        ..Default::default()
                    });
                }
                Variety::Funding => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Funding
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.funding.title.clone(),
                        visible: val.funding.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Funding Options
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    let funding_options = &val.funding.funding_options;
                    for (j, funding_program) in funding_options.iter().enumerate() {
                        // Set new field values.
                        element.visible = funding_program.visible;
                        // Same visibility, as above, for all subsections (segments).
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // Title
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // FundingProgram = FundingProgramName
                        element.address = j;
                        // This parent is for the title segment only.
                        element.parent = Some(
                            // Funding
                            // Variant::Funding,
                            Parent {
                                address: 2,
                                variety: Rank::Top,
                            },
                        );
                        element.title = funding_program.title.clone();
                        storage.chunks.push(element.clone());
                        // Reset Naïve field values to "empty".
                        {
                            element.address = empty_elem.address;
                            element.body = empty_elem.body.clone();
                            element.parent = empty_elem.parent.clone();
                            element.title = empty_elem.title.clone();
                            // element.visible = empty_elem.visible;
                        }
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // Subsections (segments)
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // Set new field values.
                        // Same parent for all subsections (segments) below.
                        element.parent = Some(
                            // FundingProgram
                            // Variant::FundingProgram,
                            Parent {
                                address: j,
                                variety: Rank::FundingProgram,
                            },
                        );
                        // FundingProgramAnnotation
                        element.address = 0;
                        element.body.notes = funding_program.annotation.stock();
                        storage.chunks.push(element.clone());
                        // Reset Naïve field values to "empty".
                        {
                            element.address = empty_elem.address;
                            element.body = empty_elem.body.clone();
                            // element.parent = empty_elem.parent.clone();
                            // element.title = empty_elem.title.clone();
                            // element.visible = empty_elem.visible;
                        }
                        // FundingProgramHyperlink
                        // Set new field values.
                        element.address = 1;
                        element.body.phrase = funding_program.hyperlink.clone();
                        storage.chunks.push(element.clone());
                        // Reset Naïve field values to "empty".
                        {
                            element.address = empty_elem.address;
                            element.body = empty_elem.body.clone();
                            // element.parent = empty_elem.parent.clone();
                            // element.title = empty_elem.title.clone();
                            // element.visible = empty_elem.visible;
                        }
                        // FundingProgramDeadline
                        // Set new field values.
                        element.address = 2;
                        element.body.date = funding_program.deadline.clone().into();
                        storage.chunks.push(element.clone());
                        // Reset all Naïve field values to "empty".
                        {
                            element.address = empty_elem.address;
                            element.body = empty_elem.body.clone();
                            element.parent = empty_elem.parent.clone();
                            element.title = empty_elem.title.clone();
                            element.visible = empty_elem.visible;
                        }
                    }
                }
                Variety::Idea => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Idea
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.idea.title.clone(),
                        visible: val.idea.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Use the association array, not the `&self.idea.index_list`,
                    // so that the order of sections be relatively stable
                    // and the mapping doesn't depend on the order in which
                    // sections appear in workbook.
                    let subsections = Association::default().section_idea;
                    for (j, subsection) in subsections.iter().enumerate() {
                        // Set new field values.
                        element.address = j;
                        element.parent = Some(
                            // Idea
                            // Variant::Idea,
                            Parent {
                                address: 1,
                                variety: Rank::Top,
                            },
                        );
                        element.visible = true;
                        match subsection {
                            Variety::SectionIdea(SectionIdea::Problem) => {
                                element.body.notes = val.idea.problem.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionIdea(SectionIdea::Hypothesis) => {
                                element.body.notes = val.idea.hypothesis.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionIdea(SectionIdea::Abstract) => {
                                element.body.notes = val.idea.summary.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionIdea(SectionIdea::ProjectDescription) => {
                                element.body.notes = val.idea.description.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionIdea(SectionIdea::KeyReferences) => {
                                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                // Title
                                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                // IdeaReferences
                                element.title = val.idea.references.title.clone();
                                element.visible = val.idea.references.visible;
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    // element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    element.title = empty_elem.title.clone();
                                    element.visible = empty_elem.visible;
                                }
                                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                // Subsections
                                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                let references = &val.idea.references.references;
                                for (k, ref_source) in references.iter().enumerate() {
                                    // Set new field values.
                                    element.visible = ref_source.visible;
                                    // Same visibility, as above, for all segments.
                                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                    // Title
                                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                    // IdeaReferencesCitation = IdeaReferencesCitationName
                                    element.address = k;
                                    // This parent is for the title segment only.
                                    element.parent = Some(
                                        // IdeaReferences
                                        // Variant::IdeaReferences,
                                        Parent {
                                            address: j,
                                            variety: Rank::IdeaReferences,
                                        },
                                    );
                                    element.title = ref_source.title.clone();
                                    storage.chunks.push(element.clone());
                                    // Reset Naïve field values to "empty".
                                    {
                                        element.address = empty_elem.address;
                                        element.body = empty_elem.body.clone();
                                        element.parent = empty_elem.parent.clone();
                                        element.title = empty_elem.title.clone();
                                        // element.visible = empty_elem.visible;
                                    }
                                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                    // Segments
                                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                    // Set new field values.
                                    // Same parent for all segments below.
                                    element.parent = Some(
                                        // IdeaReferencesCitation
                                        // Variant::IdeaReferencesCitation,
                                        Parent {
                                            address: k,
                                            variety: Rank::IdeaReferencesCitation,
                                        },
                                    );
                                    // IdeaReferencesCitationHyperlink
                                    // Set new field values.
                                    element.address = 0;
                                    element.body.phrase = ref_source.hyperlink.clone();
                                    storage.chunks.push(element.clone());
                                    // Reset Naïve field values to "empty".
                                    {
                                        element.address = empty_elem.address;
                                        element.body = empty_elem.body.clone();
                                        // element.parent = empty_elem.parent.clone();
                                        element.title = empty_elem.title.clone();
                                        // element.visible = empty_elem.visible;
                                    }
                                    // IdeaReferencesCitationDetails
                                    // Set new field values.
                                    element.address = 1;
                                    element.body.notes = ref_source.source_details.stock();
                                    storage.chunks.push(element.clone());
                                    // Reset all Naïve field values to "empty".
                                    {
                                        element.address = empty_elem.address;
                                        element.body = empty_elem.body.clone();
                                        element.parent = empty_elem.parent.clone();
                                        element.title = empty_elem.title.clone();
                                        element.visible = empty_elem.visible;
                                    }
                                }
                            }
                            // _ => {}
                            _ => unreachable!(),
                        }
                    }
                }
                Variety::Scope => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Scope
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.scope.title.clone(),
                        visible: val.scope.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Use the association array, not the `&self.scope.index_list`,
                    // so that the order of sections be relatively stable
                    // and the mapping doesn't depend on the order in which
                    // sections appear in workbook.
                    let subsections = Association::default().section_scope;
                    for (j, subsection) in subsections.iter().enumerate() {
                        // Set new field values.
                        element.address = j;
                        element.parent = Some(
                            // Scope
                            // Variant::Scope,
                            Parent {
                                address: 3,
                                variety: Rank::Top,
                            },
                        );
                        element.visible = true;
                        match subsection {
                            Variety::SectionScope(SectionScope::SuggestedTasks) => {
                                element.body.notes = val.scope.suggested_tasks.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionScope(SectionScope::Objectives) => {
                                element.body.notes = val.scope.objectives.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionScope(SectionScope::Activities) => {
                                element.body.notes = val.scope.activities.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionScope(SectionScope::WorkPlan) => {
                                element.body.notes = val.scope.work_plan.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionScope(SectionScope::Tasks) => {
                                element.body.notes = val.scope.tasks.stock();
                                storage.chunks.push(element.clone());
                                // Reset all Naïve field values to "empty".
                                {
                                    element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    element.parent = empty_elem.parent.clone();
                                    element.title = empty_elem.title.clone();
                                    element.visible = empty_elem.visible;
                                }
                            }
                            // _ => {}
                            _ => unreachable!(),
                        }
                    }
                }
                Variety::Timeline => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Timeline
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Scope
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.timeline.title.clone(),
                        visible: val.timeline.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Use the association array, not the `&self.timeline.index_list`,
                    // so that the order of sections be relatively stable
                    // and the mapping doesn't depend on the order in which
                    // sections appear in workbook.
                    let subsections = Association::default().section_timeline;
                    for (j, subsection) in subsections.iter().enumerate() {
                        // Set new field values.
                        // element.address = j;
                        element.parent = Some(
                            // Timeline
                            // Variant::Timeline,
                            Parent {
                                address: 4,
                                variety: Rank::Top,
                            },
                        );
                        element.visible = true;
                        match subsection {
                            Variety::SectionTimeline(SectionTimeline::ProjectTiming) => {
                                element.address = j;
                                element.body.date = val.timeline.project_start.clone().into();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                                element.address = j + 1;
                                element.body.numeral = val.timeline.duration_years;
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionTimeline(SectionTimeline::Milestones) => {
                                element.address = j + 1;
                                element.body.notes = val.timeline.milestones.stock();
                                storage.chunks.push(element.clone());
                                // Reset all Naïve field values to "empty".
                                {
                                    element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    element.parent = empty_elem.parent.clone();
                                    element.title = empty_elem.title.clone();
                                    element.visible = empty_elem.visible;
                                }
                            }
                            // _ => {}
                            _ => unreachable!(),
                        }
                    }
                }
                Variety::Scripting => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Scripting
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.scripting.title.clone(),
                        visible: val.scripting.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // ScriptingOptions - Answer
                    storage.chunks.push(Chunk {
                        address: 0,
                        parent: Some(
                            // Scripting
                            // Variant::Scripting,
                            Parent {
                                variety: Rank::Top,
                                address: 5,
                            },
                        ),
                        body: Body {
                            option: val.scripting.answer.clone(),
                            ..Default::default()
                        },
                        visible: val.scripting.visible,
                        ..Default::default()
                    });
                    // ScriptingOptions - Notes
                    storage.chunks.push(Chunk {
                        address: 1,
                        parent: Some(
                            // Scripting
                            // Variant::Scripting,
                            Parent {
                                variety: Rank::Top,
                                address: 5,
                            },
                        ),
                        body: Body {
                            notes: val.scripting.scripting.stock(),
                            ..Default::default()
                        },
                        visible: val.scripting.visible,
                        ..Default::default()
                    });
                }
                Variety::Team => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Team
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.team.title.clone(),
                        visible: val.team.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Use the association array, not the `&self.team.index_list`,
                    // so that the order of sections be relatively stable
                    // and the mapping doesn't depend on the order in which
                    // sections appear in workbook.
                    let subsections = Association::default().section_team;
                    for (j, subsection) in subsections.iter().enumerate() {
                        // Set new field values.
                        element.address = j;
                        element.parent = Some(
                            // Team
                            // Variant::Team,
                            Parent {
                                address: 6,
                                variety: Rank::Top,
                            },
                        );
                        element.visible = true;
                        match subsection {
                            Variety::SectionTeam(SectionTeam::ProposedPartners) => {
                                element.body.notes = val.team.proposed_partners.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionTeam(SectionTeam::ProjectLeader) => {
                                element.body.notes = val.team.project_leader.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionTeam(SectionTeam::IndustrialPartners) => {
                                element.body.notes = val.team.industrial_partners.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionTeam(SectionTeam::Proponents) => {
                                element.body.notes = val.team.proponents.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionTeam(SectionTeam::Participants) => {
                                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                // Subsections
                                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                let participants = &val.team.participants;
                                for (k, partner) in participants.iter().enumerate() {
                                    // Set new field values.
                                    element.visible = partner.visible;
                                    // Same visibility, as above, for all segments.
                                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                    // Title
                                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                    // TeamParticipantsPartner = TeamParticipantsPartnerName
                                    element.address = k;
                                    // This parent is for the title segment only.
                                    element.parent = Some(
                                        // TeamParticipants
                                        // Variant::TeamParticipants,
                                        Parent {
                                            address: j,
                                            variety: Rank::TeamParticipants,
                                        },
                                    );
                                    element.title = partner.name.clone();
                                    storage.chunks.push(element.clone());
                                    // Reset Naïve field values to "empty".
                                    {
                                        element.address = empty_elem.address;
                                        element.body = empty_elem.body.clone();
                                        element.parent = empty_elem.parent.clone();
                                        element.title = empty_elem.title.clone();
                                        // element.visible = empty_elem.visible;
                                    }
                                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                    // Segments
                                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                    // Set new field values.
                                    // Same parent for all segments below.
                                    element.parent = Some(
                                        // TeamParticipantsPartner
                                        // Variant::TeamParticipantsPartner,
                                        Parent {
                                            address: k,
                                            variety: Rank::TeamParticipantsPartner,
                                        },
                                    );
                                    // TeamParticipantsPartner - Role
                                    // Set new field values.
                                    element.address = 0;
                                    element.body.phrase = partner.role.clone();
                                    storage.chunks.push(element.clone());
                                    // TeamParticipantsPartner - Affiliation
                                    // Set new field values.
                                    element.address = 1;
                                    element.body.phrase = partner.affiliation.clone();
                                    storage.chunks.push(element.clone());
                                    // TeamParticipantsPartner - Hyperlink
                                    // Set new field values.
                                    element.address = 2;
                                    element.body.phrase = partner.hyperlink.clone();
                                    storage.chunks.push(element.clone());
                                    // TeamParticipantsPartner - Country
                                    // Set new field values.
                                    element.address = 3;
                                    element.body.phrase = partner.country.clone();
                                    storage.chunks.push(element.clone());
                                    // TeamParticipantsPartner - Expertise
                                    // Set new field values.
                                    element.address = 4;
                                    element.body.phrase = partner.expertise.clone();
                                    storage.chunks.push(element.clone());
                                    // TeamParticipantsPartner - Contribution
                                    // Set new field values.
                                    element.address = 5;
                                    element.body.notes = partner.contribution.stock();
                                    storage.chunks.push(element.clone());
                                    // TeamParticipantsPartner - Team
                                    // Set new field values.
                                    element.address = 6;
                                    element.body.notes = partner.team.stock();
                                    storage.chunks.push(element.clone());
                                    // TeamParticipantsPartner - CV
                                    // Set new field values.
                                    element.address = 7;
                                    element.body.notes = partner.cv.stock();
                                    storage.chunks.push(element.clone());
                                    // TeamParticipantsPartner - Resources
                                    // Set new field values.
                                    element.address = 8;
                                    element.body.notes = partner.resources.stock();
                                    storage.chunks.push(element.clone());
                                    // TeamParticipantsPartner - Budget
                                    // Set new field values.
                                    element.address = 9;
                                    element.body.notes = partner.budget.stock();
                                    storage.chunks.push(element.clone());
                                    // Reset Naïve field values to "empty".
                                    {
                                        element.address = empty_elem.address;
                                        element.body = empty_elem.body.clone();
                                        // element.parent = empty_elem.parent.clone();
                                        element.title = empty_elem.title.clone();
                                        // element.visible = empty_elem.visible;
                                    }
                                    // Reset all Naïve field values to "empty".
                                    {
                                        element.address = empty_elem.address;
                                        element.body = empty_elem.body.clone();
                                        element.parent = empty_elem.parent.clone();
                                        element.title = empty_elem.title.clone();
                                        element.visible = empty_elem.visible;
                                    }
                                }
                            }
                            // _ => {}
                            _ => unreachable!(),
                        }
                    }
                }
                Variety::PrelimResults => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // PrelimResults
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.prelim_results.title.clone(),
                        visible: val.prelim_results.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // PrelimResults
                    storage.chunks.push(Chunk {
                        address: 0,
                        parent: Some(
                            // PrelimResults
                            // Variant::PrelimResults,
                            Parent {
                                address: 7,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.prelim_results.prelim_results.stock(),
                            ..Default::default()
                        },
                        visible: val.prelim_results.visible,
                        ..Default::default()
                    });
                }
                Variety::Methodology => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Methodology
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.methodology.title.clone(),
                        visible: val.methodology.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Methodology
                    storage.chunks.push(Chunk {
                        address: 0,
                        parent: Some(
                            // Methodology
                            // Variant::Methodology,
                            Parent {
                                address: 8,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.methodology.methodology.stock(),
                            ..Default::default()
                        },
                        visible: val.methodology.visible,
                        ..Default::default()
                    });
                }
                Variety::Outcomes => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Outcomes
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.outcomes.title.clone(),
                        visible: val.outcomes.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Outcomes - Expected Results
                    storage.chunks.push(Chunk {
                        address: 0,
                        parent: Some(
                            // Outcomes
                            // Variant::Outcomes,
                            Parent {
                                address: 9,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.outcomes.results.stock(),
                            ..Default::default()
                        },
                        visible: val.outcomes.visible,
                        ..Default::default()
                    });
                    // Outcomes - Impact
                    storage.chunks.push(Chunk {
                        address: 1,
                        parent: Some(
                            // Outcomes
                            // Variant::Outcomes,
                            Parent {
                                address: 9,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.outcomes.impact.stock(),
                            ..Default::default()
                        },
                        visible: val.outcomes.visible,
                        ..Default::default()
                    });
                    // Outcomes - Propagation
                    storage.chunks.push(Chunk {
                        address: 2,
                        parent: Some(
                            // Outcomes
                            // Variant::Outcomes,
                            Parent {
                                address: 9,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.outcomes.propagation.stock(),
                            ..Default::default()
                        },
                        visible: val.outcomes.visible,
                        ..Default::default()
                    });
                }
                Variety::Literature => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Literature
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.literature.title.clone(),
                        visible: val.literature.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Literature Survey
                    storage.chunks.push(Chunk {
                        address: 0,
                        parent: Some(
                            // Literature
                            // Variant::Literature,
                            Parent {
                                address: 10,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.literature.literature_survey.stock(),
                            ..Default::default()
                        },
                        visible: val.literature.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Literature References
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    let bibliography = &val.literature.references;
                    for (j, bib_source) in bibliography.iter().enumerate() {
                        // Set new field values.
                        element.visible = bib_source.visible;
                        // Same visibility, as above, for all subsections (segments).
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // Title
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // LiteratureSourcesCitation = LiteratureSourcesCitationName
                        element.address = j;
                        // This parent is for the title segment only.
                        element.parent = Some(
                            // LiteratureSources
                            // Variant::LiteratureSources,
                            Parent {
                                address: 1,
                                variety: Rank::LiteratureSources,
                            },
                        );
                        element.title = bib_source.title.clone();
                        storage.chunks.push(element.clone());
                        // Reset Naïve field values to "empty".
                        {
                            element.address = empty_elem.address;
                            element.body = empty_elem.body.clone();
                            element.parent = empty_elem.parent.clone();
                            element.title = empty_elem.title.clone();
                            // element.visible = empty_elem.visible;
                        }
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // Subsections (segments)
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // Set new field values.
                        // Same parent for all subsections (segments) below.
                        element.parent = Some(
                            // LiteratureSourcesCitation
                            // Variant::LiteratureSourcesCitation,
                            Parent {
                                address: j,
                                variety: Rank::LiteratureSourcesCitation,
                            },
                        );
                        // LiteratureSourcesCitationHyperlink
                        // Set new field values.
                        element.address = 0;
                        element.body.phrase = bib_source.hyperlink.clone();
                        storage.chunks.push(element.clone());
                        // Reset Naïve field values to "empty".
                        {
                            element.address = empty_elem.address;
                            element.body = empty_elem.body.clone();
                            // element.parent = empty_elem.parent.clone();
                            // element.title = empty_elem.title.clone();
                            // element.visible = empty_elem.visible;
                        }
                        // LiteratureSourcesCitationDetails
                        element.address = 1;
                        element.body.notes = bib_source.source_details.stock();
                        storage.chunks.push(element.clone());
                        // Reset all Naïve field values to "empty".
                        {
                            element.address = empty_elem.address;
                            element.body = empty_elem.body.clone();
                            element.parent = empty_elem.parent.clone();
                            element.title = empty_elem.title.clone();
                            element.visible = empty_elem.visible;
                        }
                    }
                }
                Variety::Resources => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Resources
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.resources.title.clone(),
                        visible: val.resources.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Resources - Existing
                    storage.chunks.push(Chunk {
                        address: 0,
                        parent: Some(
                            // Resources
                            // Variant::Resources,
                            Parent {
                                address: 11,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.resources.existing.stock(),
                            ..Default::default()
                        },
                        visible: val.resources.visible,
                        ..Default::default()
                    });
                    // Resources - Additional
                    storage.chunks.push(Chunk {
                        address: 1,
                        parent: Some(
                            // Resources
                            // Variant::Resources,
                            Parent {
                                address: 11,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.resources.further.stock(),
                            ..Default::default()
                        },
                        visible: val.resources.visible,
                        ..Default::default()
                    });
                }
                Variety::Budget => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Budget
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.budget.title.clone(),
                        visible: val.budget.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Use the association array, not the `&self.budget.index_list`,
                    // so that the order of sections be relatively stable
                    // and the mapping doesn't depend on the order in which
                    // sections appear in workbook.
                    let subsections = Association::default().section_budget;
                    for (j, subsection) in subsections.iter().enumerate() {
                        // Set new field values.
                        element.address = j;
                        element.parent = Some(
                            // Budget
                            // Variant::Budget,
                            Parent {
                                address: 12,
                                variety: Rank::Top,
                            },
                        );
                        element.visible = true;
                        match subsection {
                            Variety::SectionBudget(SectionBudget::Personnel) => {
                                element.body.notes = val.budget.personnel.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionBudget(SectionBudget::Facilities) => {
                                element.body.notes = val.budget.facilities.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionBudget(SectionBudget::Materials) => {
                                element.body.notes = val.budget.materials.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionBudget(SectionBudget::Workshops) => {
                                element.body.notes = val.budget.workshops.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionBudget(SectionBudget::Overheads) => {
                                element.body.notes = val.budget.overheads.stock();
                                storage.chunks.push(element.clone());
                                // Reset Naïve field values to "empty".
                                {
                                    // element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    // element.parent = empty_elem.parent.clone();
                                    // element.title = empty_elem.title.clone();
                                    // element.visible = empty_elem.visible;
                                }
                            }
                            Variety::SectionBudget(SectionBudget::Miscellaneous) => {
                                element.body.notes = val.budget.misc.stock();
                                storage.chunks.push(element.clone());
                                // Reset all Naïve field values to "empty".
                                {
                                    element.address = empty_elem.address;
                                    element.body = empty_elem.body.clone();
                                    element.parent = empty_elem.parent.clone();
                                    element.title = empty_elem.title.clone();
                                    element.visible = empty_elem.visible;
                                }
                            }
                            // _ => {}
                            _ => unreachable!(),
                        }
                    }
                }
                Variety::Attachments => {
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Title
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Attachments
                    storage.chunks.push(Chunk {
                        address: i,
                        parent: None,
                        title: val.attachments.title.clone(),
                        visible: val.attachments.visible,
                        ..Default::default()
                    });
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Subsections
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    // Attachments - Tables
                    storage.chunks.push(Chunk {
                        address: 0,
                        parent: Some(
                            // Attachments
                            // Variant::Attachments,
                            Parent {
                                address: 13,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.attachments.tables.stock(),
                            ..Default::default()
                        },
                        visible: val.attachments.visible,
                        ..Default::default()
                    });
                    // Attachments - Figures
                    storage.chunks.push(Chunk {
                        address: 1,
                        parent: Some(
                            // Attachments
                            // Variant::Attachments,
                            Parent {
                                address: 13,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.attachments.figures.stock(),
                            ..Default::default()
                        },
                        visible: val.attachments.visible,
                        ..Default::default()
                    });
                    // Attachments - Other
                    storage.chunks.push(Chunk {
                        address: 2,
                        parent: Some(
                            // Attachments
                            // Variant::Attachments,
                            Parent {
                                address: 13,
                                variety: Rank::Top,
                            },
                        ),
                        body: Body {
                            notes: val.attachments.other.stock(),
                            ..Default::default()
                        },
                        visible: val.attachments.visible,
                        ..Default::default()
                    });
                }
                _ => {}
            }
        }
        storage
    }
}

impl Stock for Notes {
    fn stock(&self) -> Self {
        Self {
            notes: self
                .notes
                .iter()
                .filter(|note| note.visible)
                .cloned()
                .collect(),
        }
    }
}

// Test the correctness of the `Project` conversion into the storage format.
#[test]
fn conversion_into_store() {
    use chrono::Datelike;

    use crate::workbook::note::Note;
    use crate::workbook::project::ProjectOwner;
    use crate::workbook::sections::{
        attachments::Attachments,
        budget::Budget,
        funding::{FundingOptions, Programme},
        idea::Idea,
        literature::Literature,
        methodology::Methodology,
        outcomes::Outcomes,
        prelim_results::PrelimResults,
        references::{Reference, References},
        resources::Resources,
        scope::Scope,
        scripting::{Scripting, ScriptingOption},
        team::{Participant, Team},
        timeline::{Date, Timeline},
        working_name::ProjectTitle,
    };

    let test = &mut Project::default();
    test.owner = ProjectOwner {
        email: "abc@email.tst".to_string(),
        account: "abc123".to_string(),
    };
    test.record = "test_record".to_string();
    test.resolution = 3;

    test.working_name = ProjectTitle {
        title: "Test project name".to_string(),
        options: Notes {
            notes: vec![Note {
                note: "Name option 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
    };

    test.idea = Idea {
        title: "Idea".to_string(),
        problem: Notes {
            notes: vec![Note {
                note: "Problem note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        hypothesis: Notes {
            notes: vec![Note {
                note: "Hypothesis note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        summary: Notes::default(),
        description: Notes::default(),
        references: References {
            title: "Key References".to_string(),
            references: vec![
                Reference {
                    title: "Key source 1".to_string(),
                    hyperlink: "link source 1".to_string(),
                    source_details: Notes {
                        notes: vec![Note {
                            note: "Source 1 details".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    visible: true,
                },
                Reference {
                    title: "Key source 2".to_string(),
                    hyperlink: "link source 2".to_string(),
                    source_details: Notes {
                        notes: vec![Note {
                            note: "Source 2 details".to_string(),
                            visible: true,
                            ..Default::default()
                        }],
                    },
                    visible: true,
                },
            ],
            visible: true,
        },
        visible: true,
        ..Default::default()
    };

    test.funding = FundingOptions {
        title: "Funding Options".to_string(),
        funding_options: vec![
            Programme {
                title: "Funding Option 1".to_string(),
                annotation: Notes {
                    notes: vec![Note {
                        note: "Funding Option 1 annotation".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                hyperlink: "link funding option 1".to_string(),
                deadline: Date {
                    ..Default::default()
                },
                visible: true,
            },
            Programme {
                title: "Funding Option 2".to_string(),
                /* TODO: Funding Option 2 - Details: annotation, hyperlink and deadline
                */
                ..Default::default()
            },
        ],
        visible: true,
    };

    test.scope = Scope {
        title: "Project Scope: Objectives and Planned Activities".to_string(),
        suggested_tasks: Notes {
            notes: vec![Note {
                note: "Suggested Tasks note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        objectives: Notes {
            notes: vec![Note {
                note: "Objectives note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        activities: Notes {
            notes: vec![Note {
                note: "Activities note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        work_plan: Notes {
            notes: vec![Note {
                note: "Work Plan note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        tasks: Notes {
            notes: vec![Note {
                note: "Tasks note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
        ..Default::default()
    };

    test.timeline = Timeline {
        title: "Timeline".to_string(),
        project_start: Date {
            ..Default::default()
        },
        duration_years: 4.5,
        milestones: Notes {
            notes: vec![Note {
                note: "Milestones note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
        ..Default::default()
    };

    test.scripting = Scripting {
        title: "Who Will Write The Project Proposal".to_string(),
        answer: ScriptingOption::Jointly,
        scripting: Notes {
            notes: vec![Note {
                note: "Scripting Option note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
    };

    test.team = Team {
        title: "Team".to_string(),
        proposed_partners: Notes {
            notes: vec![Note {
                note: "Proposed Partners note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        project_leader: Notes {
            notes: vec![Note {
                note: "Project Leader note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        industrial_partners: Notes {
            notes: vec![Note {
                note: "Industrial Partners note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        proponents: Notes {
            notes: vec![Note {
                note: "Proponents note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        participants: vec![
            Participant {
                name: "Partner 1 Name".to_string(),
                role: "Partner 1 role".to_string(),
                affiliation: "Partner 1 affiliation".to_string(),
                hyperlink: "Partner 1 hyperlink".to_string(),
                country: "Partner 1 country".to_string(),
                expertise: "Partner 1 expertise".to_string(),
                contribution: Notes {
                    notes: vec![Note {
                        note: "Partner 1 contribution note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                team: Notes {
                    notes: vec![Note {
                        note: "Partner 1 team note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                cv: Notes {
                    notes: vec![Note {
                        note: "Partner 1 CV note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                resources: Notes {
                    notes: vec![Note {
                        note: "Partner 1 resources note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                budget: Notes {
                    notes: vec![Note {
                        note: "Partner 1 budget note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                visible: true,
                ..Default::default()
            },
            Participant {
                name: "Partner 2 Name".to_string(),
                role: "Partner 2 role".to_string(),
                affiliation: "Partner 2 affiliation".to_string(),
                hyperlink: "Partner 2 hyperlink".to_string(),
                country: "Partner 2 country".to_string(),
                expertise: "Partner 2 expertise".to_string(),
                contribution: Notes {
                    notes: vec![Note {
                        note: "Partner 2 contribution note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                team: Notes {
                    notes: vec![Note {
                        note: "Partner 2 team note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                cv: Notes {
                    notes: vec![Note {
                        note: "Partner 2 CV note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                resources: Notes {
                    notes: vec![Note {
                        note: "Partner 2 resources note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                budget: Notes {
                    notes: vec![Note {
                        note: "Partner 2 budget note 1".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                visible: true,
                ..Default::default()
            },
        ],
        visible: true,
        ..Default::default()
    };

    test.prelim_results = PrelimResults {
        title: "Preliminary Results".to_string(),
        prelim_results: Notes {
            notes: vec![Note {
                note: "Preliminary Results note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
    };

    test.methodology = Methodology {
        title: "Research Methodology".to_string(),
        methodology: Notes {
            notes: vec![Note {
                note: "Methodology note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
    };

    test.outcomes = Outcomes {
        title: "Expected Results, Impact and Dissemination".to_string(),
        results: Notes {
            notes: vec![Note {
                note: "Expected Results note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        impact: Notes {
            notes: vec![Note {
                note: "Impact note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        propagation: Notes {
            notes: vec![Note {
                note: "Propagation note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
        ..Default::default()
    };

    test.literature = Literature {
        title: "Literature Survey".to_string(),
        literature_survey: Notes {
            notes: vec![Note {
                note: "Literature Survey note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        references: vec![
            Reference {
                title: "Literature source 1".to_string(),
                hyperlink: "Literature link source 1".to_string(),
                source_details: Notes {
                    notes: vec![Note {
                        note: "Literature Source 1 details".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                visible: true,
            },
            Reference {
                title: "Literature source 2".to_string(),
                hyperlink: "Literature link source 2".to_string(),
                source_details: Notes {
                    notes: vec![Note {
                        note: "Literature Source 2 details".to_string(),
                        visible: true,
                        ..Default::default()
                    }],
                },
                visible: true,
            },
        ],
        visible: true,
        ..Default::default()
    };

    test.resources = Resources {
        title: "Resources".to_string(),
        existing: Notes {
            notes: vec![Note {
                note: "Existing Resources note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        further: Notes {
            notes: vec![Note {
                note: "Additional Resources note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
        ..Default::default()
    };

    test.budget = Budget {
        title: "Budget Estimates".to_string(),
        personnel: Notes {
            notes: vec![Note {
                note: "Budget Personnel note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        facilities: Notes {
            notes: vec![Note {
                note: "Budget Facilities note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        materials: Notes {
            notes: vec![Note {
                note: "Budget Materials note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        workshops: Notes {
            notes: vec![Note {
                note: "Budget Workshops note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        overheads: Notes {
            notes: vec![Note {
                note: "Budget Overheads note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        misc: Notes {
            notes: vec![Note {
                note: "Budget Miscellaneous note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
        ..Default::default()
    };

    test.attachments = Attachments {
        title: "Attachments".to_string(),
        tables: Notes {
            notes: vec![Note {
                note: "Attachments Tables note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        figures: Notes {
            notes: vec![Note {
                note: "Attachments Figures note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        other: Notes {
            notes: vec![Note {
                note: "Attachments Other note 1".to_string(),
                visible: true,
                ..Default::default()
            }],
        },
        visible: true,
        ..Default::default()
    };

    // Chapter counter.
    let mut nn: usize;
    let stored: Store = (*test).clone().into();

    assert_eq!(test.index_list.len(), 14);
    assert_eq!(test.index_list.len(), Association::default().sections.len());

    assert_eq!(test.owner, stored.owner);
    assert_eq!(test.record, stored.record);
    assert_eq!(test.resolution, stored.resolution);

    // nn = 0;

    // ProjectTitle = WorkingName
    assert_eq!(stored.chunks[0].address, 0);
    assert_eq!(stored.chunks[0].parent, None);
    assert_eq!(stored.chunks[0].visible, test.working_name.visible);
    assert_eq!(stored.chunks[0].title, test.working_name.title);
    assert_eq!(stored.chunks[0].title, "Test project name".to_string());

    // WorkingNameOptions
    assert_eq!(stored.chunks[1].address, 1);
    assert_eq!(
        stored.chunks[1].parent,
        Some(Parent {
            address: 0,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[1].body.notes.notes.len(),
        test.working_name.options.notes.len()
    );
    assert_eq!(
        stored.chunks[1].body.notes.notes[0].visible,
        test.working_name.options.notes[0].visible
    );
    assert_eq!(
        stored.chunks[1].body.notes.notes[0].note,
        test.working_name.options.notes[0].note
    );
    assert_eq!(
        stored.chunks[1].body.notes.notes[0].note,
        "Name option 1".to_string()
    );

    // Idea
    assert_eq!(stored.chunks[2].address, 1);
    assert_eq!(stored.chunks[2].parent, None);
    assert_eq!(stored.chunks[2].visible, test.idea.visible);
    assert_eq!(stored.chunks[2].title, test.idea.title);
    assert_eq!(stored.chunks[2].title, "Idea".to_string());

    // IdeaProblem
    assert_eq!(stored.chunks[3].address, 0);
    assert_eq!(
        stored.chunks[3].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[3].body.notes.notes.len(),
        test.idea.problem.notes.len()
    );
    assert_eq!(
        stored.chunks[3].body.notes.notes[0].visible,
        test.idea.problem.notes[0].visible
    );
    assert_eq!(
        stored.chunks[3].body.notes.notes[0].note,
        test.idea.problem.notes[0].note
    );
    assert_eq!(
        stored.chunks[3].body.notes.notes[0].note,
        "Problem note 1".to_string()
    );

    // IdeaHypothesis
    assert_eq!(stored.chunks[4].address, 1);
    assert_eq!(
        stored.chunks[4].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[4].body.notes.notes.len(),
        test.idea.hypothesis.notes.len()
    );
    assert_eq!(
        stored.chunks[4].body.notes.notes[0].visible,
        test.idea.hypothesis.notes[0].visible
    );
    assert_eq!(
        stored.chunks[4].body.notes.notes[0].note,
        test.idea.hypothesis.notes[0].note
    );
    assert_eq!(
        stored.chunks[4].body.notes.notes[0].note,
        "Hypothesis note 1".to_string()
    );

    // IdeaSummary
    assert_eq!(stored.chunks[5].address, 2);
    assert_eq!(
        stored.chunks[5].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[5].body.notes.notes.len(),
        test.idea.summary.notes.len()
    );

    // IdeaDescription
    assert_eq!(stored.chunks[6].address, 3);
    assert_eq!(
        stored.chunks[6].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[6].body.notes.notes.len(),
        test.idea.description.notes.len()
    );

    // IdeaReferences
    assert_eq!(stored.chunks[7].address, 4);
    assert_eq!(
        stored.chunks[7].parent,
        Some(Parent {
            address: 1,
            variety: Rank::Top
        })
    );
    assert_eq!(stored.chunks[7].visible, test.idea.references.visible);
    assert_eq!(stored.chunks[7].title, test.idea.references.title);
    assert_eq!(stored.chunks[7].title, "Key References".to_string());

    // IdeaReferencesCitation = IdeaReferencesCitationName
    assert_eq!(stored.chunks[8].address, 0);
    assert_eq!(
        stored.chunks[8].parent,
        Some(Parent {
            address: 4,
            variety: Rank::IdeaReferences
        })
    );
    assert_eq!(
        stored.chunks[8].visible,
        test.idea.references.references[0].visible
    );
    assert_eq!(
        stored.chunks[8].title,
        test.idea.references.references[0].title
    );
    assert_eq!(stored.chunks[8].title, "Key source 1".to_string());

    // IdeaReferencesCitationHyperlink
    assert_eq!(stored.chunks[9].address, 0);
    assert_eq!(
        stored.chunks[9].parent,
        Some(Parent {
            address: 0,
            variety: Rank::IdeaReferencesCitation
        })
    );
    assert_eq!(
        stored.chunks[9].body.phrase,
        test.idea.references.references[0].hyperlink
    );
    assert_eq!(stored.chunks[9].body.phrase, "link source 1".to_string());

    // IdeaReferencesCitationDetails
    assert_eq!(stored.chunks[10].address, 1);
    assert_eq!(
        stored.chunks[10].parent,
        Some(Parent {
            address: 0,
            variety: Rank::IdeaReferencesCitation
        })
    );
    assert_eq!(
        stored.chunks[10].body.notes.notes[0].visible,
        test.idea.references.references[0].source_details.notes[0].visible
    );
    assert_eq!(
        stored.chunks[10].body.notes.notes[0].note,
        test.idea.references.references[0].source_details.notes[0].note
    );
    assert_eq!(
        stored.chunks[10].body.notes.notes[0].note,
        "Source 1 details".to_string()
    );

    // IdeaReferencesCitation = IdeaReferencesCitationName
    assert_eq!(stored.chunks[11].address, 1);
    assert_eq!(
        stored.chunks[11].parent,
        Some(Parent {
            address: 4,
            variety: Rank::IdeaReferences
        })
    );
    assert_eq!(
        stored.chunks[11].visible,
        test.idea.references.references[1].visible
    );
    assert_eq!(
        stored.chunks[11].title,
        test.idea.references.references[1].title
    );
    assert_eq!(stored.chunks[11].title, "Key source 2".to_string());

    // IdeaReferencesCitationHyperlink
    assert_eq!(stored.chunks[12].address, 0);
    assert_eq!(
        stored.chunks[12].parent,
        Some(Parent {
            address: 1,
            variety: Rank::IdeaReferencesCitation
        })
    );
    assert_eq!(
        stored.chunks[12].body.phrase,
        test.idea.references.references[1].hyperlink
    );
    assert_eq!(stored.chunks[12].body.phrase, "link source 2".to_string());

    // IdeaReferencesCitationDetails
    assert_eq!(stored.chunks[13].address, 1);
    assert_eq!(
        stored.chunks[13].parent,
        Some(Parent {
            address: 1,
            variety: Rank::IdeaReferencesCitation
        })
    );
    assert_eq!(
        stored.chunks[13].body.notes.notes[0].visible,
        test.idea.references.references[1].source_details.notes[0].visible
    );
    assert_eq!(
        stored.chunks[13].body.notes.notes[0].note,
        test.idea.references.references[1].source_details.notes[0].note
    );
    assert_eq!(
        stored.chunks[13].body.notes.notes[0].note,
        "Source 2 details".to_string()
    );

    // Funding
    assert_eq!(stored.chunks[14].address, 2);
    assert_eq!(stored.chunks[14].parent, None);
    assert_eq!(stored.chunks[14].visible, test.funding.visible);
    assert_eq!(stored.chunks[14].title, test.funding.title);
    assert_eq!(stored.chunks[14].title, "Funding Options".to_string());

    // FundingOptions

    // FundingProgram = FundingProgramName
    assert_eq!(stored.chunks[15].address, 0);
    assert_eq!(
        stored.chunks[15].parent,
        Some(Parent {
            address: 2,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[15].visible,
        test.funding.funding_options[0].visible
    );
    assert_eq!(
        stored.chunks[15].title,
        test.funding.funding_options[0].title
    );
    assert_eq!(stored.chunks[15].title, "Funding Option 1".to_string());

    // Reset Naïve field values to "empty"
    assert_eq!(stored.chunks[15].body.phrase, "".to_string());
    assert_eq!(stored.chunks[15].body.notes.notes.len(), 0);

    // FundingProgramAnnotation
    assert_eq!(stored.chunks[16].address, 0);
    assert_eq!(
        stored.chunks[16].parent,
        Some(Parent {
            address: 0,
            variety: Rank::FundingProgram
        })
    );
    assert_eq!(
        stored.chunks[16].body.notes.notes[0].visible,
        test.funding.funding_options[0].annotation.notes[0].visible
    );
    assert_eq!(
        stored.chunks[16].body.notes.notes[0].note,
        test.funding.funding_options[0].annotation.notes[0].note
    );
    assert_eq!(
        stored.chunks[16].body.notes.notes[0].note,
        "Funding Option 1 annotation".to_string()
    );

    // Reset Naïve field values to "empty"
    assert_eq!(stored.chunks[16].title, "".to_string());
    assert_eq!(stored.chunks[16].body.phrase, "".to_string());

    // FundingProgramHyperlink
    assert_eq!(stored.chunks[17].address, 1);
    assert_eq!(
        stored.chunks[17].parent,
        Some(Parent {
            address: 0,
            variety: Rank::FundingProgram
        })
    );
    assert_eq!(
        stored.chunks[17].body.phrase,
        test.funding.funding_options[0].hyperlink
    );
    assert_eq!(
        stored.chunks[17].body.phrase,
        "link funding option 1".to_string()
    );

    // FundingProgramDeadline
    assert_eq!(stored.chunks[18].address, 2);
    assert_eq!(
        stored.chunks[18].parent,
        Some(Parent {
            address: 0,
            variety: Rank::FundingProgram
        })
    );
    assert_eq!(
        stored.chunks[18].body.date.year,
        test.funding.funding_options[0].deadline.year
    ); // default deadline
    assert_eq!(
        stored.chunks[18].body.date.month,
        test.funding.funding_options[0].deadline.month
    ); // default deadline
    assert_eq!(
        stored.chunks[18].body.date.day,
        test.funding.funding_options[0].deadline.day
    ); // default deadline

    // FundingProgram = FundingProgramName
    assert_eq!(stored.chunks[19].address, 1);
    assert_eq!(
        stored.chunks[19].parent,
        Some(Parent {
            address: 2,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[19].visible,
        test.funding.funding_options[1].visible
    );
    assert_eq!(
        stored.chunks[19].title,
        test.funding.funding_options[1].title
    );
    assert_eq!(stored.chunks[19].title, "Funding Option 2".to_string());

    // Reset Naïve field values to "empty"
    assert_eq!(stored.chunks[19].body.phrase, "".to_string());
    assert_eq!(stored.chunks[19].body.notes.notes.len(), 0);

    // FundingProgramAnnotation
    assert_eq!(stored.chunks[20].address, 0);
    assert_eq!(
        stored.chunks[20].parent,
        Some(Parent {
            address: 1,
            variety: Rank::FundingProgram
        })
    );
    assert_eq!(
        stored.chunks[20].body.notes.notes.len(),
        test.funding.funding_options[1].annotation.notes.len()
    ); // empty field
    assert_eq!(stored.chunks[20].body.notes.notes.len(), 0); // empty field

    // FundingProgramHyperlink
    assert_eq!(stored.chunks[21].address, 1);
    assert_eq!(
        stored.chunks[21].parent,
        Some(Parent {
            address: 1,
            variety: Rank::FundingProgram
        })
    );
    assert_eq!(
        stored.chunks[21].body.phrase,
        test.funding.funding_options[1].hyperlink
    );
    assert_eq!(stored.chunks[21].body.phrase, "".to_string()); // empty field

    // FundingProgramDeadline
    assert_eq!(stored.chunks[22].address, 2);
    assert_eq!(
        stored.chunks[22].parent,
        Some(Parent {
            address: 1,
            variety: Rank::FundingProgram
        })
    );
    assert_eq!(
        stored.chunks[22].body.date.year,
        test.funding.funding_options[0].deadline.year
    ); // default deadline
    assert_eq!(
        stored.chunks[22].body.date.month,
        test.funding.funding_options[0].deadline.month
    ); // default deadline
    assert_eq!(
        stored.chunks[22].body.date.day,
        test.funding.funding_options[0].deadline.day
    ); // default deadline

    // Reset Naïve field values to "empty"
    assert_eq!(stored.chunks[22].body.phrase, "".to_string());
    assert_eq!(stored.chunks[22].body.notes.notes.len(), 0);

    // Scope
    assert_eq!(stored.chunks[23].address, 3);
    assert_eq!(stored.chunks[23].parent, None);
    assert_eq!(stored.chunks[23].visible, test.scope.visible);
    assert_eq!(stored.chunks[23].title, test.scope.title);
    assert_eq!(
        stored.chunks[23].title,
        "Project Scope: Objectives and Planned Activities".to_string()
    );

    // ScopeSuggestedTasks
    assert_eq!(
        stored.chunks[24].body.notes.notes.len(),
        test.scope.suggested_tasks.notes.len()
    );
    assert_eq!(
        stored.chunks[24].body.notes.notes[0].note,
        test.scope.suggested_tasks.notes[0].note
    );
    assert_eq!(
        stored.chunks[24].body.notes.notes[0].note,
        "Suggested Tasks note 1".to_string()
    );

    // ScopeObjectives
    assert_eq!(
        stored.chunks[25].body.notes.notes.len(),
        test.scope.objectives.notes.len()
    );
    assert_eq!(
        stored.chunks[25].body.notes.notes[0].visible,
        test.scope.objectives.notes[0].visible
    );
    assert_eq!(
        stored.chunks[25].body.notes.notes[0].note,
        test.scope.objectives.notes[0].note
    );
    assert_eq!(
        stored.chunks[25].body.notes.notes[0].note,
        "Objectives note 1".to_string()
    );

    // ScopeActivities
    assert_eq!(
        stored.chunks[26].body.notes.notes.len(),
        test.scope.activities.notes.len()
    );
    assert_eq!(
        stored.chunks[26].body.notes.notes[0].visible,
        test.scope.activities.notes[0].visible
    );
    assert_eq!(
        stored.chunks[26].body.notes.notes[0].note,
        test.scope.activities.notes[0].note
    );
    assert_eq!(
        stored.chunks[26].body.notes.notes[0].note,
        "Activities note 1".to_string()
    );

    // ScopeWorkPlan
    assert_eq!(
        stored.chunks[27].body.notes.notes.len(),
        test.scope.work_plan.notes.len()
    );
    assert_eq!(
        stored.chunks[27].body.notes.notes[0].visible,
        test.scope.work_plan.notes[0].visible
    );
    assert_eq!(
        stored.chunks[27].body.notes.notes[0].note,
        test.scope.work_plan.notes[0].note
    );
    assert_eq!(
        stored.chunks[27].body.notes.notes[0].note,
        "Work Plan note 1".to_string()
    );

    // ScopeTasks
    assert_eq!(
        stored.chunks[28].body.notes.notes.len(),
        test.scope.tasks.notes.len()
    );
    assert_eq!(
        stored.chunks[28].body.notes.notes[0].visible,
        test.scope.tasks.notes[0].visible
    );
    assert_eq!(
        stored.chunks[28].body.notes.notes[0].note,
        test.scope.tasks.notes[0].note
    );
    assert_eq!(
        stored.chunks[28].body.notes.notes[0].note,
        "Tasks note 1".to_string()
    );

    // Timeline
    assert_eq!(stored.chunks[29].address, 4);
    assert_eq!(stored.chunks[29].parent, None);
    assert_eq!(stored.chunks[29].visible, test.timeline.visible);
    assert_eq!(stored.chunks[29].title, test.timeline.title);
    assert_eq!(stored.chunks[29].title, "Timeline".to_string());

    // TimelineProjectTiming - ProjectStart
    assert_eq!(
        stored.chunks[30].body.date.year,
        test.timeline.project_start.date.year()
    );
    assert_eq!(
        stored.chunks[30].body.date.month,
        test.timeline.project_start.date.month()
    );
    assert_eq!(
        stored.chunks[30].body.date.day,
        test.timeline.project_start.date.day()
    );
    assert_eq!(
        stored.chunks[30].body.date.year,
        test.timeline.project_start.year
    );
    assert_eq!(
        stored.chunks[30].body.date.month,
        test.timeline.project_start.month
    );
    assert_eq!(
        stored.chunks[30].body.date.day,
        test.timeline.project_start.day
    );

    // TimelineProjectTiming - ProjectDuration
    assert_eq!(stored.chunks[31].body.numeral, test.timeline.duration_years);
    assert_eq!(stored.chunks[31].body.numeral, 4.5);

    // TimelineMilestones
    assert_eq!(
        stored.chunks[32].body.notes.notes.len(),
        test.timeline.milestones.notes.len()
    );
    assert_eq!(
        stored.chunks[32].body.notes.notes[0].visible,
        test.timeline.milestones.notes[0].visible
    );
    assert_eq!(
        stored.chunks[32].body.notes.notes[0].note,
        test.timeline.milestones.notes[0].note
    );
    assert_eq!(
        stored.chunks[32].body.notes.notes[0].note,
        "Milestones note 1".to_string()
    );

    // Scripting
    assert_eq!(stored.chunks[33].address, 5);
    assert_eq!(stored.chunks[33].parent, None);
    assert_eq!(stored.chunks[33].visible, test.scripting.visible);
    assert_eq!(stored.chunks[33].title, test.scripting.title);
    assert_eq!(
        stored.chunks[33].title,
        "Who Will Write The Project Proposal".to_string()
    );

    // Scripting - ScriptingOption
    assert_eq!(stored.chunks[34].body.option, test.scripting.answer);
    assert_eq!(stored.chunks[34].body.option, ScriptingOption::Jointly);

    // Scripting - ScriptingNotes
    assert_eq!(
        stored.chunks[35].body.notes.notes.len(),
        test.scripting.scripting.notes.len()
    );
    assert_eq!(
        stored.chunks[35].body.notes.notes[0].visible,
        test.scripting.scripting.notes[0].visible
    );
    assert_eq!(
        stored.chunks[35].body.notes.notes[0].note,
        test.scripting.scripting.notes[0].note
    );
    assert_eq!(
        stored.chunks[35].body.notes.notes[0].note,
        "Scripting Option note 1".to_string()
    );

    nn = 35;
    nn += 1;

    // Team
    assert_eq!(stored.chunks[nn].address, 6);
    assert_eq!(stored.chunks[nn].parent, None);
    assert_eq!(stored.chunks[nn].visible, test.team.visible);
    assert_eq!(stored.chunks[nn].title, test.team.title);
    assert_eq!(stored.chunks[nn].title, "Team".to_string());

    nn += 1;

    // TeamProposedPartners
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 6,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.proposed_partners.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.proposed_partners.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.proposed_partners.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Proposed Partners note 1".to_string()
    );

    nn += 1;

    // TeamProjectLeader
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 6,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.project_leader.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.project_leader.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.project_leader.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Project Leader note 1".to_string()
    );

    nn += 1;

    // TeamIndustrialPartners
    assert_eq!(stored.chunks[nn].address, 2);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 6,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.industrial_partners.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.industrial_partners.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.industrial_partners.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Industrial Partners note 1".to_string()
    );

    nn += 1;

    // TeamProponents
    assert_eq!(stored.chunks[nn].address, 3);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 6,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.proponents.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.proponents.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.proponents.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Proponents note 1".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 4,
            variety: Rank::TeamParticipants
        })
    );
    assert_eq!(stored.chunks[nn].visible, test.team.participants[0].visible);
    assert_eq!(stored.chunks[nn].title, test.team.participants[0].name);
    assert_eq!(stored.chunks[nn].title, "Partner 1 Name".to_string());

    nn += 1;

    // TeamParticipantsPartner - Role
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[0].role
    );
    assert_eq!(stored.chunks[nn].body.phrase, "Partner 1 role".to_string());

    nn += 1;

    // TeamParticipantsPartner - Affiliation
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[0].affiliation
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Partner 1 affiliation".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Hyperlink
    assert_eq!(stored.chunks[nn].address, 2);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[0].hyperlink
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Partner 1 hyperlink".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Country
    assert_eq!(stored.chunks[nn].address, 3);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[0].country
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Partner 1 country".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Expertise
    assert_eq!(stored.chunks[nn].address, 4);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[0].expertise
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Partner 1 expertise".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Contribution
    assert_eq!(stored.chunks[nn].address, 5);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[0].contribution.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[0].contribution.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[0].contribution.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 1 contribution note 1".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Team
    assert_eq!(stored.chunks[nn].address, 6);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[0].team.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[0].team.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[0].team.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 1 team note 1".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - CV
    assert_eq!(stored.chunks[nn].address, 7);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[0].cv.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[0].cv.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[0].cv.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 1 CV note 1".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Resources
    assert_eq!(stored.chunks[nn].address, 8);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[0].resources.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[0].resources.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[0].resources.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 1 resources note 1".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Budget
    assert_eq!(stored.chunks[nn].address, 9);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[0].budget.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[0].budget.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[0].budget.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 1 budget note 1".to_string()
    );

    nn += 1;
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // TeamParticipantsPartner
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 4,
            variety: Rank::TeamParticipants
        })
    );
    assert_eq!(stored.chunks[nn].visible, test.team.participants[1].visible);
    assert_eq!(stored.chunks[nn].title, test.team.participants[1].name);
    assert_eq!(stored.chunks[nn].title, "Partner 2 Name".to_string());

    nn += 1;

    // TeamParticipantsPartner - Role
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[1].role
    );
    assert_eq!(stored.chunks[nn].body.phrase, "Partner 2 role".to_string());

    nn += 1;

    // TeamParticipantsPartner - Affiliation
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[1].affiliation
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Partner 2 affiliation".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Hyperlink
    assert_eq!(stored.chunks[nn].address, 2);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[1].hyperlink
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Partner 2 hyperlink".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Country
    assert_eq!(stored.chunks[nn].address, 3);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[1].country
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Partner 2 country".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Expertise
    assert_eq!(stored.chunks[nn].address, 4);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.team.participants[1].expertise
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Partner 2 expertise".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Contribution
    assert_eq!(stored.chunks[nn].address, 5);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[1].contribution.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[1].contribution.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[1].contribution.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 2 contribution note 1".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Team
    assert_eq!(stored.chunks[nn].address, 6);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[1].team.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[1].team.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[1].team.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 2 team note 1".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - CV
    assert_eq!(stored.chunks[nn].address, 7);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[1].cv.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[1].cv.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[1].cv.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 2 CV note 1".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Resources
    assert_eq!(stored.chunks[nn].address, 8);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[1].resources.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[1].resources.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[1].resources.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 2 resources note 1".to_string()
    );

    nn += 1;

    // TeamParticipantsPartner - Budget
    assert_eq!(stored.chunks[nn].address, 9);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::TeamParticipantsPartner
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.team.participants[1].budget.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.team.participants[1].budget.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.team.participants[1].budget.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Partner 2 budget note 1".to_string()
    );

    nn += 1;

    // PrelimResults
    assert_eq!(stored.chunks[nn].address, 7);
    assert_eq!(stored.chunks[nn].parent, None);
    assert_eq!(stored.chunks[nn].visible, test.prelim_results.visible);
    assert_eq!(stored.chunks[nn].title, test.prelim_results.title);
    assert_eq!(stored.chunks[nn].title, "Preliminary Results".to_string());

    nn += 1;

    // PrelimResults
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 7,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.prelim_results.prelim_results.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.prelim_results.prelim_results.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.prelim_results.prelim_results.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Preliminary Results note 1".to_string()
    );

    nn += 1;

    // Methodology
    assert_eq!(stored.chunks[nn].address, 8);
    assert_eq!(stored.chunks[nn].parent, None);
    assert_eq!(stored.chunks[nn].visible, test.methodology.visible);
    assert_eq!(stored.chunks[nn].title, test.methodology.title);
    assert_eq!(stored.chunks[nn].title, "Research Methodology".to_string());

    nn += 1;

    // Methodology
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 8,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.methodology.methodology.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.methodology.methodology.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.methodology.methodology.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Methodology note 1".to_string()
    );

    nn += 1;

    // Outcomes
    assert_eq!(stored.chunks[nn].address, 9);
    assert_eq!(stored.chunks[nn].parent, None);
    assert_eq!(stored.chunks[nn].visible, test.outcomes.visible);
    assert_eq!(stored.chunks[nn].title, test.outcomes.title);
    assert_eq!(
        stored.chunks[nn].title,
        "Expected Results, Impact and Dissemination".to_string()
    );

    nn += 1;

    // Outcomes - Expected Results
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 9,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.outcomes.results.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.outcomes.results.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.outcomes.results.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Expected Results note 1".to_string()
    );

    nn += 1;

    // Outcomes - Impact
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 9,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.outcomes.impact.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.outcomes.impact.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.outcomes.impact.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Impact note 1".to_string()
    );

    nn += 1;

    // Outcomes - Propagation
    assert_eq!(stored.chunks[nn].address, 2);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 9,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.outcomes.propagation.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.outcomes.propagation.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.outcomes.propagation.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Propagation note 1".to_string()
    );

    nn += 1;

    // Literature
    assert_eq!(stored.chunks[nn].address, 10);
    assert_eq!(stored.chunks[nn].parent, None);
    assert_eq!(stored.chunks[nn].visible, test.literature.visible);
    assert_eq!(stored.chunks[nn].title, test.literature.title);
    assert_eq!(stored.chunks[nn].title, "Literature Survey".to_string());

    nn += 1;

    // Literature Survey
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 10,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.literature.literature_survey.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.literature.literature_survey.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.literature.literature_survey.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Literature Survey note 1".to_string()
    );

    nn += 1;

    // Literature Sources - Bibliography
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::LiteratureSources
        })
    );
    assert_eq!(
        stored.chunks[nn].visible,
        test.literature.references[0].visible
    );
    assert_eq!(stored.chunks[nn].title, test.literature.references[0].title);
    assert_eq!(stored.chunks[nn].title, "Literature source 1".to_string());

    nn += 1;

    // LiteratureSourcesCitation - Hyperlink
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::LiteratureSourcesCitation
        })
    );
    assert_eq!(
        stored.chunks[nn].visible,
        test.literature.references[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.literature.references[0].hyperlink
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Literature link source 1".to_string()
    );

    nn += 1;

    // LiteratureSourcesCitation - Details
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 0,
            variety: Rank::LiteratureSourcesCitation
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.literature.references[0].source_details.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.literature.references[0].source_details.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.literature.references[0].source_details.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Literature Source 1 details".to_string()
    );

    nn += 1;
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Literature Sources - Bibliography
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::LiteratureSources
        })
    );
    assert_eq!(
        stored.chunks[nn].visible,
        test.literature.references[1].visible
    );
    assert_eq!(stored.chunks[nn].title, test.literature.references[1].title);
    assert_eq!(stored.chunks[nn].title, "Literature source 2".to_string());

    nn += 1;

    // LiteratureSourcesCitation - Hyperlink
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::LiteratureSourcesCitation
        })
    );
    assert_eq!(
        stored.chunks[nn].visible,
        test.literature.references[1].visible
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        test.literature.references[1].hyperlink
    );
    assert_eq!(
        stored.chunks[nn].body.phrase,
        "Literature link source 2".to_string()
    );

    nn += 1;

    // LiteratureSourcesCitation - Details
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 1,
            variety: Rank::LiteratureSourcesCitation
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.literature.references[1].source_details.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.literature.references[1].source_details.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.literature.references[1].source_details.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Literature Source 2 details".to_string()
    );

    nn += 1;

    // Resources
    assert_eq!(stored.chunks[nn].address, 11);
    assert_eq!(stored.chunks[nn].parent, None);
    assert_eq!(stored.chunks[nn].visible, test.resources.visible);
    assert_eq!(stored.chunks[nn].title, test.resources.title);
    assert_eq!(stored.chunks[nn].title, "Resources".to_string());

    nn += 1;

    // Resources - Existing
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 11,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.resources.existing.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.resources.existing.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.resources.existing.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Existing Resources note 1".to_string()
    );

    nn += 1;

    // Resources - Additional
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 11,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.resources.further.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.resources.further.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.resources.further.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Additional Resources note 1".to_string()
    );

    nn += 1;

    // Budget
    assert_eq!(stored.chunks[nn].address, 12);
    assert_eq!(stored.chunks[nn].parent, None);
    assert_eq!(stored.chunks[nn].visible, test.budget.visible);
    assert_eq!(stored.chunks[nn].title, test.budget.title);
    assert_eq!(stored.chunks[nn].title, "Budget Estimates".to_string());

    nn += 1;

    // Budget - Personnel
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.budget.personnel.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.budget.personnel.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.budget.personnel.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Budget Personnel note 1".to_string()
    );

    nn += 1;

    // Budget - Facilities
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.budget.facilities.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.budget.facilities.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.budget.facilities.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Budget Facilities note 1".to_string()
    );

    nn += 1;

    // Budget - Materials
    assert_eq!(stored.chunks[nn].address, 2);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.budget.materials.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.budget.materials.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.budget.materials.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Budget Materials note 1".to_string()
    );

    nn += 1;

    // Budget - Workshops
    assert_eq!(stored.chunks[nn].address, 3);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.budget.workshops.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.budget.workshops.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.budget.workshops.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Budget Workshops note 1".to_string()
    );

    nn += 1;

    // Budget - Overheads
    assert_eq!(stored.chunks[nn].address, 4);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.budget.overheads.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.budget.overheads.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.budget.overheads.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Budget Overheads note 1".to_string()
    );

    nn += 1;

    // Budget - Miscellaneous
    assert_eq!(stored.chunks[nn].address, 5);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 12,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.budget.misc.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.budget.misc.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.budget.misc.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Budget Miscellaneous note 1".to_string()
    );

    nn += 1;

    // Attachments
    assert_eq!(stored.chunks[nn].address, 13);
    assert_eq!(stored.chunks[nn].parent, None);
    assert_eq!(stored.chunks[nn].visible, test.attachments.visible);
    assert_eq!(stored.chunks[nn].title, test.attachments.title);
    assert_eq!(stored.chunks[nn].title, "Attachments".to_string());

    nn += 1;

    // Attachments - Tables
    assert_eq!(stored.chunks[nn].address, 0);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 13,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.attachments.tables.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.attachments.tables.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.attachments.tables.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Attachments Tables note 1".to_string()
    );

    nn += 1;

    // Attachments - Figures
    assert_eq!(stored.chunks[nn].address, 1);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 13,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.attachments.figures.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.attachments.figures.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.attachments.figures.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Attachments Figures note 1".to_string()
    );

    nn += 1;

    // Attachments - Other
    assert_eq!(stored.chunks[nn].address, 2);
    assert_eq!(
        stored.chunks[nn].parent,
        Some(Parent {
            address: 13,
            variety: Rank::Top
        })
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes.len(),
        test.attachments.other.notes.len()
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].visible,
        test.attachments.other.notes[0].visible
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        test.attachments.other.notes[0].note
    );
    assert_eq!(
        stored.chunks[nn].body.notes.notes[0].note,
        "Attachments Other note 1".to_string()
    );
}
