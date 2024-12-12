use lib_sql_lib::common::QueryLibrary;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ListUser {
    pub id: i32,
    pub email: String,
    pub name: String,
}
impl QueryLibrary for ListUser {}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AddUser {
    pub id: Option<i32>,
    pub email: String,
    pub name: String,
    pub password: String,
}
impl QueryLibrary for AddUser {}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateUser {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String,
}
impl QueryLibrary for UpdateUser {}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeleteUser {
    pub id: i32,
}
impl QueryLibrary for DeleteUser {}
