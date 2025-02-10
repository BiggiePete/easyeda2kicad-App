use crate::sqlite_lib::schema;

use super::schema::projects;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Projects {
    pub id: i32,
    pub title: String,
    pub dir: String,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub dir: &'a str,
}
