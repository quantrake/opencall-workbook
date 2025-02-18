use serde::{Deserialize, Serialize};

use crate::workbook::note::Notes;

#[derive(Deserialize, Serialize, Clone)]
pub struct Attachments {
    pub title: String,
    pub tables: Notes,
    pub figures: Notes,
    pub other: Notes,
    pub visible: bool,
}

impl Default for Attachments {
    fn default() -> Self {
        Self {
            title: String::from("Attachments"),
            tables: Notes::default(),
            figures: Notes::default(),
            other: Notes::default(),
            visible: false,
        }
    }
}
