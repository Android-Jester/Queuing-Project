use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Worker {
    pub id: String,
    pub name: String,
    pub role: String,
    pub section: String,
    pub task_id: String,
    pub department_id: String,
}
