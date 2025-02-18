use serde::{Deserialize, Serialize};

use crate::workbook::{
    chapter::Variety,
    note::Notes,
    project::ProjectOwner,
    sections::{
        budget::SectionBudget,
        idea::SectionIdea,
        outcomes::SectionOutcomes,
        scope::SectionScope,
        scripting::ScriptingOption,
        team::{ModuleParticipant, SectionTeam},
        timeline::SectionTimeline,
    },
};

pub const DATA_FORMAT_VERSION: &str = "B0004"; // "B" for Bincode

// The Association struct establishes the order of sections (subsections)
// for correct mapping.
// Note: Fixed-size arrays, not a resizable array type `Vec<Variety>`.
pub struct Association {
    // The associative array for sections.
    // The value, i.e. the respective variant of `Variety` of section,
    // is associated with a key, i.e. an index (position) in the array.
    // The enum is used for keys to disallow key-value pairs having
    // the same keys as each other.
    pub sections: [Variety; 14],

    // The associative arrays for subsections.
    // The value, i.e. the respective variant of `Variety` of subsection,
    // is associated with a key, i.e. an index (position) in the array.
    pub section_budget: [Variety; 6],
    pub section_idea: [Variety; 5],
    #[allow(unused, reason = "required for impl Default")]
    pub section_outcomes: [Variety; 3],
    pub section_scope: [Variety; 5],
    pub section_team: [Variety; 5],
    pub section_timeline: [Variety; 2],
    #[allow(unused, reason = "required for impl Default")]
    pub module_participant: [Variety; 5],
}

impl Default for Association {
    fn default() -> Self {
        Self {
            // IMPORTANT! This order should be preserved for correct mapping
            // between the storage struct and the internal struct.
            // New sections should be added to the end of this array.
            // (The initial order for respective `index_list`s is used.)
            sections: [
                Variety::WorkingName,   // 0
                Variety::Idea,          // 1
                Variety::Funding,       // 2
                Variety::Scope,         // 3
                Variety::Timeline,      // 4
                Variety::Scripting,     // 5
                Variety::Team,          // 6
                Variety::PrelimResults, // 7
                Variety::Methodology,   // 8
                Variety::Outcomes,      // 9
                Variety::Literature,    // 10
                Variety::Resources,     // 11
                Variety::Budget,        // 12
                Variety::Attachments,   // 13
            ],
            section_budget: [
                Variety::SectionBudget(SectionBudget::Personnel),
                Variety::SectionBudget(SectionBudget::Facilities),
                Variety::SectionBudget(SectionBudget::Materials),
                Variety::SectionBudget(SectionBudget::Workshops),
                Variety::SectionBudget(SectionBudget::Overheads),
                Variety::SectionBudget(SectionBudget::Miscellaneous),
            ],
            section_idea: [
                Variety::SectionIdea(SectionIdea::Problem),
                Variety::SectionIdea(SectionIdea::Hypothesis),
                Variety::SectionIdea(SectionIdea::Abstract),
                Variety::SectionIdea(SectionIdea::ProjectDescription),
                Variety::SectionIdea(SectionIdea::KeyReferences),
            ],
            section_outcomes: [
                Variety::SectionOutcomes(SectionOutcomes::ExpectedResults),
                Variety::SectionOutcomes(SectionOutcomes::Impact),
                Variety::SectionOutcomes(SectionOutcomes::Propagation),
            ],
            section_scope: [
                Variety::SectionScope(SectionScope::SuggestedTasks),
                Variety::SectionScope(SectionScope::Objectives),
                Variety::SectionScope(SectionScope::Activities),
                Variety::SectionScope(SectionScope::WorkPlan),
                Variety::SectionScope(SectionScope::Tasks),
            ],
            section_team: [
                Variety::SectionTeam(SectionTeam::ProposedPartners),
                Variety::SectionTeam(SectionTeam::ProjectLeader),
                Variety::SectionTeam(SectionTeam::IndustrialPartners),
                Variety::SectionTeam(SectionTeam::Proponents),
                Variety::SectionTeam(SectionTeam::Participants),
            ],
            section_timeline: [
                Variety::SectionTimeline(SectionTimeline::ProjectTiming),
                Variety::SectionTimeline(SectionTimeline::Milestones),
            ],
            module_participant: [
                Variety::SectionTeam(SectionTeam::ModuleParticipant(
                    ModuleParticipant::Contribution,
                )),
                Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::Team)),
                Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::CV)),
                Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::Resources)),
                Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::Budget)),
            ],
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Store {
    pub format: String, // Data format version
    pub owner: ProjectOwner,
    pub record: String,
    pub resolution: usize,

    // The list of chapters, ordered by the app developer.
    pub chunks: Vec<Chunk>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            format: DATA_FORMAT_VERSION.to_string(),
            owner: ProjectOwner::default(),
            record: String::new(),
            resolution: 1,
            chunks: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Chunk {
    // Relative address on the respective level of the tree (`Rank`).
    pub address: usize,

    pub parent: Option<Parent>,
    pub title: String,
    pub body: Body, // payload
    pub visible: bool,
}

#[derive(Deserialize, Serialize, Default, Clone, Debug, PartialEq)]
pub struct Parent {
    // Relative address on the respective level of the tree (`Rank`).
    pub address: usize,

    // Probably, the `Rank` makes the storage format more flexible.
    pub variety: Rank,
}

// Body describes the payload of an element.
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Body {
    // Naïve approach: duplicate necessary types for correct mapping.
    pub date: YearMonthDay,
    pub notes: Notes,
    pub phrase: String,
    pub numeral: f32,
    pub option: ScriptingOption,
}

#[derive(Deserialize, Serialize, Default, Clone, Debug, PartialEq)]
pub struct YearMonthDay {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

// The `Rank` enum provides several types according to the level of each section
// in the tree. The `Rank` enum only defines one dimension (the level) of 'Parent'.
// As the `address` is relative on each level, it's necessary that 'parents'
// have unique `address`-`level` pairs. Thus, the the `parent` and the `address`
// can define the location of any component (element) in the tree, so the
// respective mapping rule in the matrix of correspondence between the internal
// and the external (storage) structs can be set.
// Note: The sequence of sections is set ether with the `address` or the index
//       number in the array of the elements of the same `variety` and `Parent`.
#[derive(Deserialize, Serialize, Default, Clone, Debug, PartialEq)]
pub enum Rank {
    // `Top` sections don't have parents.
    /* Examples
    Attachments
    Budget
    ExpectedResults
    Funding
    Idea
    Literature
    Methodology
    PrelimResults
    Resources
    Scope
    Scripting
    Team
    Timeline
    WorkingName
    */
    #[default]
    Top,

    // Elements of the `Mid` rank, which have parents of the `Top` rank.
    // Distinguish children of each of these branches from children of
    // other branches of `Rank::Mid`.
    IdeaReferences,
    FundingProgram,
    TeamParticipants,
    LiteratureSources,

    // Elements of the `Low` rank, which have parents of the `Mid` rank.
    // Distinguish children of each of these branches from children of
    // other branches of `Rank::Low`.
    IdeaReferencesCitation,
    TeamParticipantsPartner,
    LiteratureSourcesCitation,
    // #############################################
    // New sections and subsections should be added at the end.
}
