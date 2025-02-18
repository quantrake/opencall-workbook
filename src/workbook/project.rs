use serde::{Deserialize, Serialize};

use super::chapter::{Segment, Variety};
use super::sections::{
    attachments::Attachments, budget::Budget, funding::FundingOptions, idea::Idea,
    literature::Literature, methodology::Methodology, outcomes::Outcomes,
    prelim_results::PrelimResults, resources::Resources, scope::Scope, scripting::Scripting,
    team::Team, timeline::Timeline, working_name::ProjectTitle,
};

#[derive(Deserialize, Serialize, Clone)]
pub struct Project {
    // Project record name for named revisions and snapshots.
    pub record: String,

    pub owner: ProjectOwner,

    // The level of details, i.e. resolution (resolving power),
    // the ability to separate or distinguish the sections or
    // subjects (topics) of entered pieces of information
    // (e.g. notes, references, etc.)
    pub resolution: usize,

    // The ordered list of sections.
    // Note: This vector allows to order the sections of Workbook as required.
    pub index_list: Vec<Segment>,

    // Workbook section types.
    pub attachments: Attachments, // TODO
    pub budget: Budget, // Provide a detailed budget estimate, including anticipated expenses for research materials, equipment, participant compensation, travel, and other relevant costs. Justify the budget based on the project's scope and requirements.
    pub funding: FundingOptions,
    pub idea: Idea,
    pub literature: Literature,
    pub methodology: Methodology, // `Methodology` - research design, methodology and techniques
    pub outcomes: Outcomes,       // Outcomes and Impact (+ Dissemination)
    pub prelim_results: PrelimResults,
    pub resources: Resources, // Distinguish: Existing equipment, computation clusters; Facilities, Equipment; Personnel – PhD students and postdocs
    pub scope: Scope,         // `Scope` ('Objectives' / 'The Scope of Research' / 'Subject Matter')
    pub scripting: Scripting,
    pub team: Team,                 // `Participants`
    pub timeline: Timeline, // The time required for performing each part of the research project. Develop a realistic timeline that outlines the major milestones and activities of the research project
    pub working_name: ProjectTitle, // Distinguished as a section to enable alternative versions of the working title of project
}

impl Default for Project {
    fn default() -> Self {
        Self {
            record: String::new(),
            owner: ProjectOwner::default(),
            resolution: 1,
            index_list: vec![
                Segment {
                    variety: Variety::WorkingName,
                    tier: 1,
                },
                Segment {
                    variety: Variety::Idea,
                    tier: 1,
                },
                Segment {
                    variety: Variety::Funding,
                    tier: 1,
                },
                Segment {
                    variety: Variety::Scope,
                    tier: 2,
                },
                Segment {
                    variety: Variety::Timeline,
                    tier: 1,
                },
                Segment {
                    variety: Variety::Scripting,
                    tier: 2,
                },
                Segment {
                    variety: Variety::Team,
                    tier: 2,
                },
                Segment {
                    variety: Variety::PrelimResults,
                    tier: 4, // Full description of the project
                },
                Segment {
                    variety: Variety::Methodology,
                    tier: 4, // Full description of the project
                },
                Segment {
                    variety: Variety::Outcomes,
                    tier: 4,
                },
                Segment {
                    variety: Variety::Literature,
                    tier: 4,
                },
                Segment {
                    variety: Variety::Resources,
                    tier: 4,
                },
                Segment {
                    variety: Variety::Budget,
                    tier: 4,
                },
                Segment {
                    variety: Variety::Attachments,
                    tier: 999,
                },
            ],
            working_name: ProjectTitle::default(),
            funding: FundingOptions::default(),
            idea: Idea::default(),
            prelim_results: PrelimResults::default(),
            scope: Scope::default(),
            methodology: Methodology::default(),
            team: Team::default(),
            timeline: Timeline::default(),
            scripting: Scripting::default(),
            literature: Literature::default(),
            outcomes: Outcomes::default(),
            resources: Resources::default(),
            budget: Budget::default(),
            attachments: Attachments::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)] // for unit tests
pub struct ProjectOwner {
    pub email: String,
    pub account: String,
}

impl Default for ProjectOwner {
    fn default() -> Self {
        let config = crate::open_call::Settings::default();
        let customer = config.customer;

        Self {
            email: customer.email,
            account: customer.id,
        }
    }
}
