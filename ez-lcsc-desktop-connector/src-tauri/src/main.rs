// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use axum::extract::Json as AxumJson;
use axum::{
    http::Method,
    routing::{get, post},
    Json, Router,
};
use db::Project;
use easyeda2kicad_rs::import_component;
use file_picker_utils::select_folder;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use tower_http::cors::{Any, CorsLayer};
mod db;
mod file_picker_utils;
use crate::kicad_file_fix::KiCadSymbolFixer;
use notify_rust::Notification;
use tauri_plugin_autostart::MacosLauncher;
mod kicad_file_fix;

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
    info!("Query for Projects!");
    // this should query the database and return all the projects we got
    let projs = db::get_all_records().unwrap_or(Vec::default());
    Json(ProjectResponse { message: projs })
}
// TODO add this function
async fn get_create_project() -> Json<ProjectResponse> {
    info!("Creating a Project");
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
    info!("Received POST request with CODE: {}", &payload.c);
    // now since the C code and the URL are correct, we need to send the build dir and the lCSC code to the generate_library_files command
    let proj = db::get_record_by_id(payload.id).unwrap().unwrap();
    let out = generate_lib_files_at_dir(&payload.c, &proj.dir).await;
    match out {
        true => {
            notify_complete(
                "Part Added To Project".to_string(),
                format!(
                    "Part: {}\nProject: {}\nLocation: {}",
                    payload.c, proj.proj_name, proj.dir
                ),
            );
        }
        false => {
            notify_complete(
                    "Part Failed".to_string(),
                    format!(
                        "Part: {} Failed\nPlease check LCSC or JLC Parts for an EASY EDA footprint, parts without footprints cannot be added!",
                        payload.c
                    ),
                );
        }
    }
    Json(Response {
        message: format!("YAY"),
    })
}
async fn remove_project(
    Json(payload): AxumJson<RemoveProjectRequest>, // Extract the JSON payload from the request
) -> Json<Response> {
    info!("Removing Project with ID {}", &payload.id);
    // now since the C code and the URL are correct, we need to send the build dir and the lCSC code to the generate_library_files command
    let _ = db::remove_record_by_id(payload.id);
    Json(Response {
        message: format!("record removed"),
    })
}

fn generate_library_files_at_dir(
    lcsc_code: &String,
    build_dir: &String,
) -> std::option::Option<i32> {
    info!("Generating project output");
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let output = Command::new("easyeda2kicad.exe")
        .arg("--lcsc_id")
        .arg(lcsc_code)
        .arg("--full")
        .arg("--output")
        .arg(format!("{}/lib", build_dir.replace("\\", "/")))
        .arg("--overwrite")
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("command failed to start");

    info!("stdout: {}", String::from_utf8_lossy(&output.stdout));

    // Get the exit code
    let exit_code = output.status.code();

    // After generating files, automatically fix any KiCad symbol files
    let lib_dir = format!("{}", build_dir.replace("\\", "/"));
    let fixer = KiCadSymbolFixer::new().with_verbose();

    match fixer.fix_directory(&lib_dir) {
        Ok((processed, fixed)) => {
            if processed > 0 {
                info!(
                    "KiCad Symbol Fixer: Processed {} files, fixed {} files",
                    processed, fixed
                );
            }
        }
        Err(e) => {
            error!("Error running KiCad symbol fixer: {}", e);
        }
    }

    exit_code
}

async fn generate_lib_files_at_dir(lcsc_code: &String, proj_dir: &String) -> bool {
    info!("Generating project output");
    match import_component(lcsc_code, Path::new(proj_dir)).await {
        Ok(_) => {
            info!("Successfully generated library files");
            return true;
        }
        Err(e) => {
            error!("Failed to generate library files: {}", e);
            return false;
        }
    }
}
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let state = Arc::new(AppState::default());

    // system tray stuff

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(state.clone())
        .system_tray(tray)
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![""]), /* arbitrary number of args to pass to your app */
        ))
        .setup(move |_app| {
            let http_state = state.clone();

            // Configure CORS
            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(tower_http::cors::Any);

            // Setup HTTP server
            let router = Router::new()
                .route("/api/getHealth ", get(get_health))
                .route("/api/getProjectList", get(get_project_list))
                .route("/api/addTOProject", post(add_2_project))
                .route("/api/createNewProject", get(get_create_project))
                .route("/api/removeProject", post(remove_project))
                .with_state(http_state)
                .layer(cors);
            // Get the app handle for potential use in Axum handlers

            // Spawn Axum server in Tauri's runtime
            tauri::async_runtime::spawn(async move {
                let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
                info!("HTTP server running on http://{}", addr);

                axum::Server::bind(&addr)
                    .serve(router.into_make_service())
                    .await
                    .unwrap();
            });

            Ok(())
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                // get a handle to the clicked menu item
                // note that `tray_handle` can be called anywhere,
                // just get an `AppHandle` instance with `app.handle()` on the setup hook
                // and move it to another function or thread
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
                            item_handle.set_title("Show").unwrap();
                        } else {
                            window.show().unwrap();
                            // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
                            item_handle.set_title("Hide").unwrap();
                        }
                    }
                    "quit" => {
                        let window = app.get_window("main").unwrap();
                        window.close().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_projects_invoke,
            delete_project_invoke,
            add_project_invoke,
            add_part_by_lcsc_invoke,
            open_build_dir_invoke,
            edit_project_invoke
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn edit_project_invoke(original: String, new: String) -> Project {
    return db::edit_record(&original, &new).expect("Project not found!");
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
    info!("removing {}", id);
    let _ = db::remove_record_by_id(id);
}
#[tauri::command]
fn add_project_invoke() -> Vec<Project> {
    let full_path = select_folder();

    // check to make sure that full_path is not none, then pass it along
    if full_path.is_none() {
        info!("No folder selected, returning empty project list.");
        return Vec::default();
    }
    let full_path = full_path.unwrap().as_os_str().to_str().unwrap().to_string();

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
async fn add_part_by_lcsc_invoke(id: String, c: String) -> bool {
    let proj = db::get_record_by_id(id).unwrap().unwrap();
    let out = generate_lib_files_at_dir(&c, &proj.dir).await;
    match out {
        true => {
            notify_complete(
                "Part Added To Project".to_string(),
                format!(
                    "Part: {}\nProject: {}\nLocation: {}",
                    c, proj.proj_name, proj.dir
                ),
            );
        }
        false => {
            notify_complete(
                    "Part Failed".to_string(),
                    format!(
                        "Part: {} Failed\nPlease check LCSC or JLC Parts for an EASY EDA footprint, parts without footprints cannot be added!",
                        c
                    ),
                );
        }
    }
    return out;
}

fn notify_complete(title: String, body: String) {
    let _notification = Notification::new()
        .summary(&title)
        .body(&body)
        .appname("EZ LCSC2KICAD") // Optional
        .show()
        .unwrap();
}
