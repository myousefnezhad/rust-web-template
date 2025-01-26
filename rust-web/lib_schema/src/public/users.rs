use lib_sql_lib::common::QueryLibrary;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ListUser {
    pub id: i32,
    pub email: String,
    pub name: String,
}
impl QueryLibrary for ListUser {
    fn get_query() -> String {
        include_str!("../../../SQL/select_users.sql").to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoginUser {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String,
}

impl QueryLibrary for LoginUser {
    fn get_query() -> String {
        include_str!("../../../SQL/login_user.sql").to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AddUser {
    pub id: Option<i32>,
    pub email: String,
    pub name: String,
    pub password: String,
}
impl QueryLibrary for AddUser {
    fn get_query() -> String {
        "".to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateUser {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String,
}
impl QueryLibrary for UpdateUser {
    fn get_query() -> String {
        "".to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeleteUser {
    pub id: i32,
}
impl QueryLibrary for DeleteUser {
    fn get_query() -> String {
        "".to_owned()
    }
}
