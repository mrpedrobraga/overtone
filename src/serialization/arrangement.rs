use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ArrangementHeader {
    info: ArrangementHeaderInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArrangementHeaderInfo {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArrangementHeaderEditorInfo {
    requires_version: String,
}
