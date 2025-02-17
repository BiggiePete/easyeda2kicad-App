// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use axum::extract::Json as AxumJson;
use axum::{
    http::Method,
    routing::{get, post},
    Json, Router,
};
use db::Project;
use file_picker_utils::select_folder;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::process::Command;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
mod db;
mod file_picker_utils;
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectResponse {
    message: Vec<Project>,
}
#[derive(Default)]
struct AppState {}
// Define the data structure for the incoming POST request
#[derive(Deserialize)]
struct Add2ProjectRequest {
    id: String,
    c: String,
}

#[derive(Deserialize)]
struct RemoveProjectRequest {
    id: String,
}

async fn get_health() -> Json<Response> {
    Json(Response {
        message: "Hello from EZ LCSC Handler".to_string(),
    })
}
// TODO, finish debugging why this isnt sending state properly to the frontend
async fn get_project_list() -> Json<ProjectResponse> {
    println!("GOT A QUERY");
    // this should query the database and return all the projects we got
    let projs = db::get_all_records().unwrap_or(Vec::default());
    Json(ProjectResponse { message: projs })
}
// TODO add this function
async fn get_create_project() -> Json<ProjectResponse> {
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
    let _ = db::add_record_with_details(&folder_name, &full_path);
    let projs = db::get_all_records();
    Json(ProjectResponse {
        message: projs.unwrap(),
    })
}

// TODO. finish this function
async fn add_2_project(
    Json(payload): AxumJson<Add2ProjectRequest>, // Extract the JSON payload from the request
) -> Json<Response> {
    println!("Received POST request with CODE: {}", &payload.c);
    // now since the C code and the URL are correct, we need to send the build dir and the lCSC code to the generate_library_files command

    let _err = generate_library_files_at_dir(
        &payload.c,
        db::get_record_by_id(payload.id).unwrap().unwrap().dir,
    );
    Json(Response {
        message: format!("YAY"),
    })
}
async fn remove_project(
    Json(payload): AxumJson<RemoveProjectRequest>, // Extract the JSON payload from the request
) -> Json<Response> {
    println!("Removing Project with ID {}", &payload.id);
    // now since the C code and the URL are correct, we need to send the build dir and the lCSC code to the generate_library_files command
    let _ = db::remove_record_by_id(payload.id);
    Json(Response {
        message: format!("record removed"),
    })
}

// TODO, call the EZLCSC executable and generate the project files at the dir

fn generate_library_files_at_dir(
    lcsc_code: &String,
    build_dir: String,
) -> std::option::Option<i32> {
    println!("Generating project output");
    let output = Command::new("easyeda2kicad.exe")
        .arg("--lcsc_id")
        .arg(lcsc_code)
        .arg("--full")
        .arg("--output")
        .arg(format!("{}/lib", build_dir.replace("\\", "/")))
        .arg("--overwrite")
        .output()
        .expect("command failed to start");
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    return output.status.code();
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
        .route("/api/addTOProject", post(add_2_project))
        .route("/api/createNewProject", get(get_create_project))
        .route("/api/removeProject", post(remove_project))
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
        .invoke_handler(tauri::generate_handler![
            get_projects_invoke,
            delete_project_invoke,
            add_project_invoke,
            add_part_by_lcsc_invoke,
            open_build_dir_invoke
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_projects_invoke() -> Vec<Project> {
    return db::get_all_records().unwrap_or(Vec::default());
}
#[tauri::command]
fn open_build_dir_invoke(dir: String) {
    let _ = opener::open(dir);
}
#[tauri::command]
fn delete_project_invoke(id: String) {
    let _ = db::remove_record_by_id(id);
}
#[tauri::command]
fn add_project_invoke() -> Vec<Project> {
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
    let _ = db::add_record_with_details(&folder_name, &full_path);
    return db::get_all_records().unwrap_or(Vec::default());
}
#[tauri::command]
fn add_part_by_lcsc_invoke(id: String, c: String) -> i32 {
    let err = generate_library_files_at_dir(&c, db::get_record_by_id(id).unwrap().unwrap().dir);
    return err.unwrap();
}
