use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Item {
    pub(crate) id: String,
    pub(crate) account: String,
    pub(crate) secret: String,
    pub(crate) desc: String,
    pub(crate) status: usize,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Export {
    pub version: i32,
    pub desc: String,
    pub time: String,
    pub content: String,
}