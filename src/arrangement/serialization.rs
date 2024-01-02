use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeader {
    pub info: ArrangementHeaderInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeaderInfo {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeaderEditorInfo {
    pub requires_version: String,
}
