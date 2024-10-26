use lib_config::AppConfig;
use lib_error::http::ResponseError;
use log::*;
use std::fs::read_to_string;

pub trait QueryLibrary {
    fn _read_sql_file(file: &str) -> Result<String, ResponseError> {
        Ok(read_to_string(file)?)
    }

    fn read_query_from_path(path: &str) -> Result<String, ResponseError> {
        Ok(Self::_read_sql_file(path)?)
    }

    fn read_query_from_lib(config: &AppConfig, filename: &str) -> Result<String, ResponseError> {
        let full_path = format!("{}/{}", config.pq_sql_lib, filename);
        debug!("{}", format!("SQL lib is reading from {}", &full_path));
        Ok(Self::_read_sql_file(&full_path)?)
    }

    fn function_call(function: &str) -> String {
        format!("SELECT {};", function)
    }
}
