use lib_sql_lib::common::QueryLibrary;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: i32,
    pub name: String,
}
impl QueryLibrary for Group {}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AddGroup {
    pub id: Option<i32>,
    pub name: String,
}
impl QueryLibrary for AddGroup {}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeleteGroup {
    pub id: i32,
}
impl QueryLibrary for DeleteGroup {}
