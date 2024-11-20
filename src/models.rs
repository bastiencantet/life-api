use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::response::Response;
use crate::schema::projects;


#[derive(Insertable, Queryable)]
#[diesel(table_name = crate::schema::projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub image: &'a str,
}

#[derive(Queryable, Debug, AsChangeset, Selectable, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
}


pub type Projects = Response<Project>;