use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Department {
    pub id: String,
    pub department_name: String,
}

#[derive(Serialize, Deserialize)]
pub enum TaskStatus {
    Completed,
    Incomplete,
    Pending,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    pub task_id: String,
    pub task: String,
    pub task_name: String,
    #[serde(with = "ts_seconds_option")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub completed_date: Option<DateTime<Utc>>,
    pub task_status: TaskStatus,
    pub task_details: String,
    pub task_report: String,
}

#[derive(Serialize, Deserialize)]
pub struct Worker {
    pub id: String,
    pub name: String,
    pub role: String,
    pub section: String,
    pub task_id: String,
    pub department_id: String,
}
