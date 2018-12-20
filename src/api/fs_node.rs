use chrono::NaiveDateTime;
use uuid::Uuid;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "nodeType", /*rename_all = "camelCase"*/)]
pub enum FsNode {
    FILE {
        id: Uuid,
        path: String,
        name: String,
        hidden: bool,
        creation: NaiveDateTime,
        modification: NaiveDateTime,
        owner: Uuid,
        size: i64,
        #[serde(rename = "humanReadableSize")]
        human_readable_size: String,
        hash: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
        #[serde(rename = "hasThumbnail")]
        has_thumbnail: bool,
    },
    DIRECTORY {
        id: Uuid,
        path: String,
        name: String,
        hidden: bool,
        creation: NaiveDateTime,
        modification: NaiveDateTime,
        owner: Uuid,
        content: Vec<FsNode>,
    },
}
