use lib_sql_lib::common::QueryLibrary;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Groups {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl QueryLibrary for Groups {}
