use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub notes: String,
    pub tags: String,
    pub due_date: Option<DateTime<Local>>,
    pub project: Option<Area>,
    pub area: Option<Area>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub tags: String,
    pub area: Area,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Area {
    pub id: String,
    pub name: String,
    pub tags: String,
}
