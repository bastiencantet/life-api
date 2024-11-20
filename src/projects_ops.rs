use diesel::prelude::*;
use crate::db::establish_connection;
use crate::models::{NewProject, Project};


pub fn new_project(project: NewProject, connection: &mut PgConnection) {
    use crate::schema::projects::dsl::*;
    
    let new_project = NewProject {
        name: &project.name,
        description: &project.description,
        image: &project.image,
    };

    diesel::insert_into(projects)
        .values(&new_project)
        .execute(connection)
        .expect("Error saving new project");
}

pub fn update_project(project: Project, connection: &mut PgConnection) {
    use crate::schema::projects::dsl::*;
    
    let updated_project = diesel::update(projects.find(project.id))
        .set(project)
        .execute(connection)
        .expect(&"Unable to find project".to_string());
}

pub fn get_all_projects(connection: &mut PgConnection) -> Vec<Project> {
    use crate::schema::projects::dsl::*;

    let results = projects.select(Project::as_select()).load(connection)
        .expect("Error loading projects");
    results
}