// Prevents additional console window on Windows in release, DO NOT REMOVE!!

use axum::{
  routing::{get, post},
  Json,
  Router,
  http::{Method},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use axum::extract::Json as AxumJson;


// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectResponse {
    message: Vec<String>,
}

#[derive(Default)]
struct AppState {
}

// Define the data structure for the incoming POST request
#[derive(Deserialize)]
struct PostRequest {
    C: String,
}

async fn get_health(
  state: axum::extract::State<Arc<AppState>>,
) -> Json<Response> {
  Json(Response {
      message: "Hello from EZ LCSC Handler".to_string(),
  })
}
async fn get_project_list(
  state: axum::extract::State<Arc<AppState>>,
) -> Json<ProjectResponse> {
  Json(ProjectResponse {
      message: ["test".to_string(),"test2".to_string()].to_vec(),
  })
}

async fn post_item_code(
  Json(payload): AxumJson<PostRequest>, // Extract the JSON payload from the request
) -> Json<Response> {
  println!("Received POST request with CODE: {}", payload.C);
  
  // You can process the 'url' string here and send back a response
  Json(Response {
      message: format!("Received CODE: {}", payload.C),
  })
}


#[tokio::main]
async fn main() {
  let state = Arc::new(AppState::default());
  let http_state = state.clone();

  // Configure CORS
  let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET,Method::POST])
        .allow_headers(tower_http::cors::Any);

  // Setup HTTP server
  let app = Router::new()
      .route("/api/getLCSC", get(get_health))
      .route("/api/getLCSC", post(post_item_code))
      .route("/api/getProjectList",get(get_project_list))
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
