use agent::AgentAddress;
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    routing, Form, Json, RequestExt, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

pub mod agent;

#[derive(Deserialize, Debug)]
pub struct EmulateMessage {
    code: String,
    shots: usize,
    agent: agent::AgentType,
}

#[derive(Clone)]
pub struct ServerState {
    pub agent_address: agent::AgentAddress,
}

pub async fn consume_task(
    Form(message): Form<EmulateMessage>,
    agent: &AgentAddress,
) -> (StatusCode, Json<Value>) {
    let code = message.code;
    let shots = message.shots;

    match agent::run(&code, shots, message.agent, agent).await {
        Ok(response) if response.status() == reqwest::StatusCode::OK => (
            StatusCode::OK,
            Json(response.json::<Value>().await.unwrap()),
        ),
        Ok(response) => (
            StatusCode::BAD_REQUEST,
            Json(response.json::<Value>().await.unwrap()),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"Error": format!("{}", err)})),
        ),
    }
}

pub async fn submit(state: State<ServerState>, request: Request) -> (StatusCode, Json<Value>) {
    match request.headers().get(header::CONTENT_TYPE) {
        Some(content_type) => match content_type.to_str().unwrap() {
            "application/x-www-form-urlencoded" => {
                let Form(message) = request.extract().await.unwrap();
                consume_task(Form(message), &state.agent_address).await
            }
            _ => (
                StatusCode::BAD_REQUEST,
                Json(json!({"Error": format!("content type {:?} not support", content_type)})),
            ),
        },
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({"Error": format!("content type not specified")})),
        ),
    }
}

#[tokio::main]
async fn main() {
    let state = ServerState {
        agent_address: agent::read_config(),
    };

    let qpp_router = Router::new()
        .route("/submit", routing::post(submit))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await.unwrap();
    axum::serve(listener, qpp_router).await.unwrap();
}
