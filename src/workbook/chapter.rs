use serde::{Deserialize, Serialize};

use crate::workbook::sections::budget::SectionBudget;
use crate::workbook::sections::idea::SectionIdea;
use crate::workbook::sections::outcomes::SectionOutcomes;
use crate::workbook::sections::scope::SectionScope;
use crate::workbook::sections::team::SectionTeam;
use crate::workbook::sections::timeline::SectionTimeline;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Segment {
    // The type of content.
    pub variety: Variety,

    // The rank (level) in the structure of the sections which visibility
    // to be managed with the `resolution` in the `Project`.
    pub tier: usize,
}

// Variety reflects the essence (character, nature) of contents.
// [Sections, chapters, divisions, segments, components, etc.]
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum Variety {
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
    SectionBudget(SectionBudget),
    SectionIdea(SectionIdea),
    SectionOutcomes(SectionOutcomes),
    SectionScope(SectionScope),
    SectionTeam(SectionTeam),
    SectionTimeline(SectionTimeline),
}
