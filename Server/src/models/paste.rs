use chrono::{DateTime, Utc};
use scylla::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct PasteById {
    pub(crate) paste_id: Uuid,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) syntax: Option<String>,
    pub(crate) password: bool,
    pub(crate) expire: Option<DateTime<Utc>>,
    pub(crate) burn: bool,
    pub(crate) user_id: Option<Uuid>,
}