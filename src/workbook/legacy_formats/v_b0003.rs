use serde::Deserialize;

use crate::workbook::{
    app::Workbook,
    chapter::{Segment, Variety},
    note::{Note, Notes},
    project::{Project, ProjectOwner},
    sections::{
        attachments::Attachments,
        budget::{Budget, SectionBudget},
        funding::{FundingOptions, Programme},
        idea::{Idea, SectionIdea},
        literature::Literature,
        methodology::Methodology,
        outcomes::{Outcomes, SectionOutcomes},
        prelim_results::PrelimResults,
        references::{Reference, References},
        resources::Resources,
        scope::{Scope, SectionScope},
        scripting::{Scripting, ScriptingOption},
        team::{ModuleParticipant, Participant, SectionTeam, Team},
        timeline::{Date, SectionTimeline, Timeline},
        working_name::ProjectTitle,
    },
};

#[derive(Deserialize)]
pub struct WorkVersionB0003(String, ProjectVersion0003, Vec<ProjectVersion0003>);

impl WorkVersionB0003 {
    pub fn decode_bincode(encoded: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(encoded)
    }
}

#[derive(Deserialize)]
struct ProjectVersion0003 {
    // Project record name for named revisions and snapshots.
    record: String,

    // Project owner
    owner: ProjectOwner0003,

    // The level of details, i.e. resolution (resolving power),
    // the ability to separate or distinguish the sections or
    // subjects (topics) of entered pieces of information
    // (e.g. notes, references, etc.)
    resolution: usize,

    // The ordered list of sections.
    // Note: This vector allows to order the sections of Workbook as required.
    index_list: Vec<Segment0003>,

    // Note: The following is the alphabetically ordered list of all Workbook section types.
    attachments: Attachments0003, // TODO
    budget: Budget0003, // Provide a detailed budget estimate, including anticipated expenses for research materials, equipment, participant compensation, travel, and other relevant costs. Justify the budget based on the project's scope and requirements.
    funding: FundingOptions0003,
    idea: Idea0003,
    literature: Literature0003,
    methodology: Methodology0003, // `Methodology` - research design, methodology and techniques
    outcomes: Outcomes0003,       // Outcomes and Impact (+ Dissemination)
    prelim_results: PrelimResults0003,
    resources: Resources0003, // Distinguish: Existing equipment, computation clusters; Facilities, Equipment; Personnel – PhD students and postdocs
    scope: Scope0003,         // `Scope` ('Objectives' / 'The Scope of Research' / 'Subject Matter')
    scripting: Scripting0003,
    team: Team0003,                 // `Participants`
    timeline: Timeline0003, // The time required for performing each part of the research project. Develop a realistic timeline that outlines the major milestones and activities of the research project
    working_name: ProjectTitle0003, // Distinguished as a section to enable alternative versions of the working title of project
}

#[derive(Deserialize)]
struct Attachments0003 {
    title: String,
    tables: Notes0003,
    figures: Notes0003,
    other: Notes0003,
    visible: bool,
}

#[derive(Deserialize)]
struct Budget0003 {
    title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    index_list: Vec<Segment0003>,

    personnel: Notes0003, // Personnel (incl. stipends for PhD students and postdocs)
    facilities: Notes0003, // Facilities, Equipment
    materials: Notes0003, // Materials, software, publications, conference fee and travel expenses
    workshops: Notes0003, // Organization of meetings and workshops
    overheads: Notes0003, // Overheads
    misc: Notes0003,      // Miscellaneous
    visible: bool,
}

#[derive(Deserialize)]
struct Date0003 {
    year: i32,
    month: u32,
    day: u32,
    date: chrono::DateTime<chrono::Local>,
}

#[derive(Deserialize)]
struct FundingOptions0003 {
    title: String,
    funding_options: Vec<Programme0003>,
    visible: bool,
}

#[derive(Deserialize)]
struct Idea0003 {
    title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    index_list: Vec<Segment0003>,

    problem: Notes0003,         // Introduction – what is the problem to be solved
    hypothesis: Notes0003,      // Main idea, research hypothesis
    summary: Notes0003, // Abstract. Would be useful to add on later stages as general information about the project
    description: Notes0003, // Short description of the project
    references: References0003, // The 'Key References' section is moved here
    visible: bool,
}

#[derive(Deserialize)]
struct Literature0003 {
    title: String,
    literature_survey: Notes0003,
    references: Vec<Reference0003>,
    visible: bool,
}

#[derive(Deserialize)]
struct Methodology0003 {
    title: String,
    methodology: Notes0003, // Methodology: research design, methodology and techniques
    visible: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Note0003 {
    note: String,
    #[serde(skip)]
    hint: String,
    visible: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Notes0003 {
    notes: Vec<Note0003>,
}

#[derive(Deserialize)]
struct Outcomes0003 {
    title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    index_list: Vec<Segment0003>,

    results: Notes0003,
    impact: Notes0003,
    propagation: Notes0003,
    visible: bool,
}

#[derive(Deserialize)]
struct Participant0003 {
    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    index_list: Vec<Segment0003>,

    name: String,
    role: String,
    affiliation: String,
    hyperlink: String,
    country: String,
    expertise: String,
    contribution: Notes0003,
    team: Notes0003,
    cv: Notes0003, // The short CV of the Participant (Partner's team leader)
    resources: Notes0003,
    budget: Notes0003,
    visible: bool,
}

#[derive(Deserialize)]
struct PrelimResults0003 {
    title: String,
    prelim_results: Notes0003,
    visible: bool,
}

#[derive(Deserialize)]
struct Programme0003 {
    title: String,
    hyperlink: String,

    deadline: Date0003,

    annotation: Notes0003,
    visible: bool,
}

#[derive(Deserialize)]
struct ProjectOwner0003 {
    email: String,
    account: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ProjectTitle0003 {
    title: String,

    options: Notes0003, // Alternative versions of project title
    visible: bool,
}

#[derive(Deserialize)]
struct Reference0003 {
    title: String,
    hyperlink: String,
    source_details: Notes0003,
    visible: bool,
}

#[derive(Deserialize)]
struct References0003 {
    title: String,
    references: Vec<Reference0003>,
    visible: bool,
}

#[derive(Deserialize)]
struct Resources0003 {
    title: String,
    existing: Notes0003, // Resources, available
    further: Notes0003,  // Resources, required (extra)
    visible: bool,
}

#[derive(Deserialize)]
struct Scope0003 {
    // `Scope` - Describe 'The Scope of Research' ('Subject Matter')
    title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    index_list: Vec<Segment0003>,

    suggested_tasks: Notes0003, // Tasks for the partners as suggested by the participants
    objectives: Notes0003,      // 'Objectives'
    activities: Notes0003,      // 'Activities' – General description of the planned work
    work_plan: Notes0003,
    tasks: Notes0003,
    visible: bool,
}

#[derive(Deserialize)]
struct Scripting0003 {
    title: String,
    answer: ScriptingOption0003,
    scripting: Notes0003,
    visible: bool,
}

#[derive(Deserialize)]
struct Segment0003 {
    // The type of content.
    variety: Variety0003,

    // The rank (level) in the structure of the sections which visibility
    // to be managed with the `resolution` in the `Project`.
    tier: usize,
}

#[derive(Deserialize)]
struct Team0003 {
    title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    index_list: Vec<Segment0003>,

    proposed_partners: Notes0003, // Suggestion of the project team and possible partners
    project_leader: Notes0003,    // Suggestion of the project leader
    industrial_partners: Notes0003, // Possible industrial partners (if necessary)
    proponents: Notes0003,        // Suggestion for cooperation with non-participants of the project

    participants: Vec<Participant0003>, // Full information about participants, institutions, teams

    visible: bool,
}

#[derive(Deserialize)]
struct Timeline0003 {
    title: String,

    // The ordered list of chapters (paragraphs).
    // Note: This vector allows to order the paragraphs of this section as required.
    index_list: Vec<Segment0003>,

    project_start: Date0003,
    duration_years: f32,
    milestones: Notes0003,
    visible: bool,
}

// ============================================================================

#[derive(Deserialize)]
enum ScriptingOption0003 {
    Myself,
    Jointly,
    Other,
}

// Variety reflects the essence (character, nature) of contents.
// [Sections, chapters, divisions, segments, components, etc.]
#[derive(Deserialize)]
enum Variety0003 {
    // Sections (sort ascending for convenience).
    Attachments,
    Budget,
    Funding,
    Idea,
    Literature,
    Methodology,
    Outcomes,
    PrelimResults,
    Resources,
    Scope,
    Scripting,
    Team,
    Timeline,
    WorkingName,

    Section, // Generic section containing notes

    // Extensions for subsections.
    // Chapters or paragraphs (subsections), i.e. the bits of text
    // which, being combined, make up the contents of sections.
    // Note: They are in separate enums (extensions).
    SectionBudget(SectionBudget0003),
    SectionIdea(SectionIdea0003),
    SectionOutcomes(SectionOutcomes0003),
    SectionScope(SectionScope0003),
    SectionTeam(SectionTeam0003),
    SectionTimeline(SectionTimeline0003),
}

#[derive(Deserialize)]
enum SectionBudget0003 {
    Facilities,
    Materials,
    Miscellaneous,
    Overheads,
    Personnel,
    Workshops,
}

#[derive(Deserialize)]
enum SectionIdea0003 {
    Abstract,
    Hypothesis,
    KeyReferences,
    Problem,
    ProjectDescription,
}

#[derive(Deserialize)]
enum SectionOutcomes0003 {
    ExpectedResults,
    Impact,
    Propagation,
}

#[derive(Deserialize)]
enum SectionScope0003 {
    Activities,
    Objectives,
    SuggestedTasks,
    Tasks,
    WorkPlan,
}

#[derive(Deserialize)]
enum SectionTeam0003 {
    IndustrialPartners,
    Participants,
    ProjectLeader,
    Proponents,
    ProposedPartners,

    // Extension for subsections.
    // Note: It is in a separate enum (extension).
    ModuleParticipant(ModuleParticipant0003),
}

#[derive(Deserialize)]
enum ModuleParticipant0003 {
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

#[derive(Deserialize)]
enum SectionTimeline0003 {
    Milestones,
    ProjectTiming,
}

impl From<ProjectVersion0003> for Project {
    fn from(val: ProjectVersion0003) -> Self {
        Project {
            record: val.record,
            owner: val.owner.into(),

            index_list: val.index_list.into_iter().map(|x| x.into()).collect(),

            resolution: val.resolution,

            attachments: val.attachments.into(),
            budget: val.budget.into(),
            funding: val.funding.into(),
            idea: val.idea.into(),
            literature: val.literature.into(),
            methodology: val.methodology.into(),
            outcomes: val.outcomes.into(),
            prelim_results: val.prelim_results.into(),
            resources: val.resources.into(),
            scope: val.scope.into(),
            scripting: val.scripting.into(),
            team: val.team.into(),
            timeline: val.timeline.into(),
            working_name: val.working_name.into(),
        }
    }
}

impl From<Attachments0003> for Attachments {
    fn from(val: Attachments0003) -> Self {
        Attachments {
            title: val.title,
            tables: val.tables.into(),
            figures: val.figures.into(),
            other: val.other.into(),
            visible: val.visible,
        }
    }
}

impl From<Budget0003> for Budget {
    fn from(val: Budget0003) -> Self {
        Budget {
            title: val.title,
            index_list: val.index_list.into_iter().map(|x| x.into()).collect(),
            personnel: val.personnel.into(),
            facilities: val.facilities.into(),
            materials: val.materials.into(),
            workshops: val.workshops.into(),
            overheads: val.overheads.into(),
            misc: val.misc.into(),
            visible: val.visible,
        }
    }
}

impl From<Date0003> for Date {
    fn from(val: Date0003) -> Self {
        Date {
            year: val.year,
            month: val.month,
            day: val.day,
            date: val.date,
        }
    }
}

impl From<FundingOptions0003> for FundingOptions {
    fn from(val: FundingOptions0003) -> Self {
        FundingOptions {
            title: val.title,
            funding_options: val.funding_options.into_iter().map(|x| x.into()).collect(),
            visible: val.visible,
        }
    }
}

impl From<Idea0003> for Idea {
    fn from(val: Idea0003) -> Self {
        Idea {
            title: val.title,
            index_list: val.index_list.into_iter().map(|x| x.into()).collect(),
            problem: val.problem.into(),
            hypothesis: val.hypothesis.into(),
            summary: val.summary.into(),
            description: val.description.into(),
            references: val.references.into(),
            visible: val.visible,
        }
    }
}

impl From<Literature0003> for Literature {
    fn from(val: Literature0003) -> Self {
        Literature {
            title: val.title,
            literature_survey: val.literature_survey.into(),
            references: val.references.into_iter().map(|x| x.into()).collect(),
            visible: val.visible,
        }
    }
}

impl From<Methodology0003> for Methodology {
    fn from(val: Methodology0003) -> Self {
        Methodology {
            title: val.title,
            methodology: val.methodology.into(),
            visible: val.visible,
        }
    }
}

impl From<Note0003> for Note {
    fn from(val: Note0003) -> Self {
        Note {
            note: val.note,
            hint: val.hint,
            visible: val.visible,
        }
    }
}

impl From<Notes0003> for Notes {
    fn from(val: Notes0003) -> Self {
        Notes {
            notes: val.notes.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<Outcomes0003> for Outcomes {
    fn from(val: Outcomes0003) -> Self {
        Outcomes {
            title: val.title,
            index_list: val.index_list.into_iter().map(|x| x.into()).collect(),
            results: val.results.into(),
            impact: val.impact.into(),
            propagation: val.propagation.into(),
            visible: val.visible,
        }
    }
}

impl From<Participant0003> for Participant {
    fn from(val: Participant0003) -> Self {
        Participant {
            name: val.name,
            role: val.role,
            affiliation: val.affiliation,
            hyperlink: val.hyperlink,
            country: val.country,
            expertise: val.expertise,
            index_list: val.index_list.into_iter().map(|x| x.into()).collect(),
            contribution: val.contribution.into(),
            team: val.team.into(),
            cv: val.cv.into(),
            resources: val.resources.into(),
            budget: val.budget.into(),
            visible: val.visible,
        }
    }
}

impl From<PrelimResults0003> for PrelimResults {
    fn from(val: PrelimResults0003) -> Self {
        PrelimResults {
            title: val.title,
            prelim_results: val.prelim_results.into(),
            visible: val.visible,
        }
    }
}

impl From<Programme0003> for Programme {
    fn from(val: Programme0003) -> Self {
        Programme {
            title: val.title,
            hyperlink: val.hyperlink,
            deadline: val.deadline.into(),
            annotation: val.annotation.into(),
            visible: val.visible,
        }
    }
}

impl From<ProjectOwner0003> for ProjectOwner {
    fn from(val: ProjectOwner0003) -> Self {
        ProjectOwner {
            email: val.email,
            account: val.account,
        }
    }
}

impl From<ProjectTitle0003> for ProjectTitle {
    fn from(val: ProjectTitle0003) -> Self {
        ProjectTitle {
            title: val.title,
            options: val.options.into(),
            visible: val.visible,
        }
    }
}

impl From<Reference0003> for Reference {
    fn from(val: Reference0003) -> Self {
        Reference {
            title: val.title,
            hyperlink: val.hyperlink,
            source_details: val.source_details.into(),
            visible: val.visible,
        }
    }
}

impl From<References0003> for References {
    fn from(val: References0003) -> Self {
        References {
            title: val.title,
            references: val.references.into_iter().map(|x| x.into()).collect(),
            visible: val.visible,
        }
    }
}

impl From<Resources0003> for Resources {
    fn from(val: Resources0003) -> Self {
        Resources {
            title: val.title,
            existing: val.existing.into(),
            further: val.further.into(),
            visible: val.visible,
        }
    }
}

impl From<Scripting0003> for Scripting {
    fn from(val: Scripting0003) -> Self {
        Scripting {
            title: val.title,
            answer: val.answer.into(),
            scripting: val.scripting.into(),
            visible: val.visible,
        }
    }
}

impl From<Scope0003> for Scope {
    fn from(val: Scope0003) -> Self {
        Scope {
            title: val.title,
            index_list: val.index_list.into_iter().map(|x| x.into()).collect(),
            suggested_tasks: val.suggested_tasks.into(),
            objectives: val.objectives.into(),
            activities: val.activities.into(),
            work_plan: val.work_plan.into(),
            tasks: val.tasks.into(),
            visible: val.visible,
        }
    }
}

impl From<Segment0003> for Segment {
    fn from(val: Segment0003) -> Self {
        Segment {
            variety: val.variety.into(),
            tier: val.tier,
        }
    }
}

impl From<Team0003> for Team {
    fn from(val: Team0003) -> Self {
        Team {
            title: val.title,
            index_list: val.index_list.into_iter().map(|x| x.into()).collect(),
            proposed_partners: val.proposed_partners.into(),
            project_leader: val.project_leader.into(),
            industrial_partners: val.industrial_partners.into(),
            proponents: val.proponents.into(),
            participants: val.participants.into_iter().map(|x| x.into()).collect(),
            visible: val.visible,
        }
    }
}

impl From<Timeline0003> for Timeline {
    fn from(val: Timeline0003) -> Self {
        Timeline {
            title: val.title,
            index_list: val.index_list.into_iter().map(|x| x.into()).collect(),
            project_start: val.project_start.into(),
            duration_years: val.duration_years,
            milestones: val.milestones.into(),
            visible: val.visible,
        }
    }
}

impl From<ScriptingOption0003> for ScriptingOption {
    fn from(val: ScriptingOption0003) -> Self {
        match val {
            ScriptingOption0003::Myself => ScriptingOption::Myself,
            ScriptingOption0003::Jointly => ScriptingOption::Jointly,
            ScriptingOption0003::Other => ScriptingOption::Other,
        }
    }
}

impl From<Variety0003> for Variety {
    fn from(val: Variety0003) -> Self {
        match val {
            Variety0003::Attachments => Variety::Attachments,
            Variety0003::Budget => Variety::Budget,
            Variety0003::Funding => Variety::Funding,
            Variety0003::Idea => Variety::Idea,
            Variety0003::Literature => Variety::Literature,
            Variety0003::Methodology => Variety::Methodology,
            Variety0003::Outcomes => Variety::Outcomes,
            Variety0003::PrelimResults => Variety::PrelimResults,
            Variety0003::Resources => Variety::Resources,
            Variety0003::Scope => Variety::Scope,
            Variety0003::Scripting => Variety::Scripting,
            Variety0003::Section => Variety::Section,
            Variety0003::Team => Variety::Team,
            Variety0003::Timeline => Variety::Timeline,
            Variety0003::WorkingName => Variety::WorkingName,

            Variety0003::SectionBudget(SectionBudget0003::Facilities) => {
                Variety::SectionBudget(SectionBudget::Facilities)
            }
            Variety0003::SectionBudget(SectionBudget0003::Materials) => {
                Variety::SectionBudget(SectionBudget::Materials)
            }
            Variety0003::SectionBudget(SectionBudget0003::Miscellaneous) => {
                Variety::SectionBudget(SectionBudget::Miscellaneous)
            }
            Variety0003::SectionBudget(SectionBudget0003::Overheads) => {
                Variety::SectionBudget(SectionBudget::Overheads)
            }
            Variety0003::SectionBudget(SectionBudget0003::Personnel) => {
                Variety::SectionBudget(SectionBudget::Personnel)
            }
            Variety0003::SectionBudget(SectionBudget0003::Workshops) => {
                Variety::SectionBudget(SectionBudget::Workshops)
            }

            Variety0003::SectionIdea(SectionIdea0003::Abstract) => {
                Variety::SectionIdea(SectionIdea::Abstract)
            }
            Variety0003::SectionIdea(SectionIdea0003::Hypothesis) => {
                Variety::SectionIdea(SectionIdea::Hypothesis)
            }
            Variety0003::SectionIdea(SectionIdea0003::KeyReferences) => {
                Variety::SectionIdea(SectionIdea::KeyReferences)
            }
            Variety0003::SectionIdea(SectionIdea0003::Problem) => {
                Variety::SectionIdea(SectionIdea::Problem)
            }
            Variety0003::SectionIdea(SectionIdea0003::ProjectDescription) => {
                Variety::SectionIdea(SectionIdea::ProjectDescription)
            }

            Variety0003::SectionOutcomes(SectionOutcomes0003::ExpectedResults) => {
                Variety::SectionOutcomes(SectionOutcomes::ExpectedResults)
            }
            Variety0003::SectionOutcomes(SectionOutcomes0003::Impact) => {
                Variety::SectionOutcomes(SectionOutcomes::Impact)
            }
            Variety0003::SectionOutcomes(SectionOutcomes0003::Propagation) => {
                Variety::SectionOutcomes(SectionOutcomes::Propagation)
            }

            Variety0003::SectionScope(SectionScope0003::Activities) => {
                Variety::SectionScope(SectionScope::Activities)
            }
            Variety0003::SectionScope(SectionScope0003::Objectives) => {
                Variety::SectionScope(SectionScope::Objectives)
            }
            Variety0003::SectionScope(SectionScope0003::SuggestedTasks) => {
                Variety::SectionScope(SectionScope::SuggestedTasks)
            }
            Variety0003::SectionScope(SectionScope0003::Tasks) => {
                Variety::SectionScope(SectionScope::Tasks)
            }
            Variety0003::SectionScope(SectionScope0003::WorkPlan) => {
                Variety::SectionScope(SectionScope::WorkPlan)
            }

            Variety0003::SectionTeam(SectionTeam0003::IndustrialPartners) => {
                Variety::SectionTeam(SectionTeam::IndustrialPartners)
            }
            Variety0003::SectionTeam(SectionTeam0003::Participants) => {
                Variety::SectionTeam(SectionTeam::Participants)
            }
            Variety0003::SectionTeam(SectionTeam0003::ProjectLeader) => {
                Variety::SectionTeam(SectionTeam::ProjectLeader)
            }
            Variety0003::SectionTeam(SectionTeam0003::Proponents) => {
                Variety::SectionTeam(SectionTeam::Proponents)
            }
            Variety0003::SectionTeam(SectionTeam0003::ProposedPartners) => {
                Variety::SectionTeam(SectionTeam::ProposedPartners)
            }

            Variety0003::SectionTeam(SectionTeam0003::ModuleParticipant(
                ModuleParticipant0003::Budget,
            )) => Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::Budget)),
            Variety0003::SectionTeam(SectionTeam0003::ModuleParticipant(
                ModuleParticipant0003::Contribution,
            )) => Variety::SectionTeam(SectionTeam::ModuleParticipant(
                ModuleParticipant::Contribution,
            )),
            Variety0003::SectionTeam(SectionTeam0003::ModuleParticipant(
                ModuleParticipant0003::CV,
            )) => Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::CV)),
            // Variety0003::SectionTeam(SectionTeam0003::ModuleParticipant(ModuleParticipant0003::Details)) => Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::Details)),
            Variety0003::SectionTeam(SectionTeam0003::ModuleParticipant(
                ModuleParticipant0003::Resources,
            )) => {
                Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::Resources))
            }
            Variety0003::SectionTeam(SectionTeam0003::ModuleParticipant(
                ModuleParticipant0003::Team,
            )) => Variety::SectionTeam(SectionTeam::ModuleParticipant(ModuleParticipant::Team)),

            Variety0003::SectionTimeline(SectionTimeline0003::Milestones) => {
                Variety::SectionTimeline(SectionTimeline::Milestones)
            }
            Variety0003::SectionTimeline(SectionTimeline0003::ProjectTiming) => {
                Variety::SectionTimeline(SectionTimeline::ProjectTiming)
            }
        }
    }
}

impl Workbook {
    pub fn apply_format_b0003(&mut self, work: WorkVersionB0003) {
        let _old_version = work.0;
        let old_project = work.1;

        // Map ProjectVersion0003 to Project.
        self.project = old_project.into();

        let old_stored_projects = work.2;
        self.stored_projects = old_stored_projects.into_iter().map(|x| x.into()).collect();
    }
}
