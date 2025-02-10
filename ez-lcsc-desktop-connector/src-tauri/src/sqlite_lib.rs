pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use schema::projects::id;
use std::env;

use self::models::*;
use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_projects_from_db() -> Vec<models::Projects> {
    use self::schema::projects::dsl::*;

    let connection = &mut establish_connection();
    let results = projects.select(Projects::as_select()).load(connection);

    match results {
        Ok(projs) => {
            println!("Displaying {} projects", projs.len());
            for proj in &projs {
                println!("id : {}", proj.id);
                println!("title : {}", proj.title);
                println!("dir : {}", proj.dir);
                println!("-----------\n");
            }
            return projs;
        }
        Err(_) => {
            println!("ERROR FETCHING FROM DB");
            let tmp_vec = Vec::new();
            return tmp_vec;
        }
    }
}

pub fn get_project_by_id(project_id: i32) -> Projects {
    use self::schema::projects::dsl::*;

    let connection = &mut establish_connection();
    let project = projects
        .find(project_id)
        .select(Projects::as_select())
        .first(connection)
        .optional();
    return project.unwrap().unwrap();
}

pub fn create_project(title: String, dir: String) -> Projects {
    use schema::projects;
    let conn = &mut establish_connection();

    let new_project = NewProject {
        title: &title,
        dir: &dir,
    };

    diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Projects::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn delete_project(project_id: i32) {
    use self::schema::projects::dsl::*;

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(projects.filter(id.eq(project_id)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
