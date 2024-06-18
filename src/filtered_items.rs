use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct FilteredItem {
    pub value: String,
    pub title: bool,
    pub oplink: bool,
}
