use lib_sql_lib::common::QueryLibrary;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: i32,
    pub name: String,
}
impl QueryLibrary for Group {
    fn get_query() -> String {
        include_str!("../../../SQL/select_groups.sql").to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AddGroup {
    pub id: Option<i32>,
    pub name: String,
}
impl QueryLibrary for AddGroup {
    fn get_query() -> String {
        "".to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeleteGroup {
    pub id: i32,
}
impl QueryLibrary for DeleteGroup {
    fn get_query() -> String {
        "".to_owned()
    }
}
