use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Department {
  pub id: String,
  pub department_name: String,
}