// Prevents additional console window on Windows in release, DO NOT REMOVE!!

use axum::extract::Json as AxumJson;
use axum::{
    http::Method,
    routing::{get, post},
    Json, Router,
};
use file_picker_utils::select_folder;
use serde::{Deserialize, Serialize};
use sqlite_lib::models::Projects;
use std::net::SocketAddr;
use std::process::{Command, Output};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
mod sqlite_lib;
use sqlite_lib::{create_project, get_project_by_id, get_projects_from_db};
mod file_picker_utils;
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectResponse {
    message: Vec<Projects>,
}

#[derive(Default)]
struct AppState {}

#[derive(Deserialize)]
struct CreateProjectRequest {
    projectName: String,
}
// Define the data structure for the incoming POST request
#[derive(Deserialize)]
struct Add2ProjectRequest {
    id: i32,
    C: String,
}

async fn get_health(state: axum::extract::State<Arc<AppState>>) -> Json<Response> {
    Json(Response {
        message: "Hello from EZ LCSC Handler".to_string(),
    })
}
// TODO, finish debugging why this isnt sending state properly to the frontend
async fn get_project_list(state: axum::extract::State<Arc<AppState>>) -> Json<ProjectResponse> {
    println!("GOT A QUERY");
    // this should query the database and return all the projects we got
    let projs = get_projects_from_db();
    Json(ProjectResponse { message: projs })
}
// TODO add this function
async fn get_create_project(state: axum::extract::State<Arc<AppState>>) -> Json<ProjectResponse> {
    println!("Creating a Project");
    let full_path = select_folder()
        .unwrap()
        .as_mut_os_string()
        .to_str()
        .unwrap()
        .to_string();

    let folder_name = std::path::Path::new(&full_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    create_project(folder_name, full_path);
    let projs = get_projects_from_db();
    Json(ProjectResponse { message: projs })
}

// TODO. finish this function
async fn add_2_project(
    Json(payload): AxumJson<Add2ProjectRequest>, // Extract the JSON payload from the request
) -> Json<Response> {
    println!("Received POST request with CODE: {}", &payload.C);
    match generate_library_files_at_dir(&payload.C, get_project_by_id(payload.id).dir) {
        Ok(result) => {
            println!("{}", result.status);
        }
        Err(err) => {
            println!("{}", err);
        }
    };
    // You can process the 'url' string here and send back a response
    Json(Response {
        message: format!("Received CODE: {}", payload.C),
    })
}

// TODO, call the EZLCSC executable and generate the project files at the dir
fn generate_library_files_at_dir(
    lcsc_code: &String,
    build_dir: String,
) -> Result<Output, std::io::Error> {
    return Command::new("cmd").args(["ls"]).output();
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());
    let http_state = state.clone();

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(tower_http::cors::Any);

    // Setup HTTP server
    let app = Router::new()
        .route("/api/getHealth ", get(get_health))
        .route("/api/getProjectList", get(get_project_list))
        .route("/api/add2Project", post(add_2_project))
        .route("/api/createNewProject", get(get_create_project))
        .with_state(http_state)
        .layer(cors);

    // Run HTTP server in separate task
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("HTTP server running on http://{}", addr);

    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    tauri::Builder::default()
        .manage(state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
