use std::sync::Arc;

use axum::{extract::{Path, State}, routing::{get, post}, Json, Router};
use log::{debug, info, error};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use database::DataBase;
use dotenv::dotenv;
use model::WBmodel;


pub use self::error::{Error, Result};
mod model;
mod error;
mod database;

struct ServerData {
    ip: String,
    port: String,

    db_username: String,
    db_password: String,
    db_table_name: String,
}

impl ServerData {
    fn get_adress(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    fn parse_env() -> Self {
        dotenv().ok();
        let ip = std::env::var("WB_IP").expect("WB_IP must be set.");
        let port = std::env::var("WB_PORT").expect("WB_PORT must be set.");

        let db_username = std::env::var("WB_DB_username").expect("WB_DB_username must be set.");
        let db_password = std::env::var("WB_DB_password").expect("WB_DB_password must be set.");
        let db_table_name = std::env::var("WB_DB_table_name").expect("WB_DB_table_name must be set.");

        ServerData{ip, port, db_username, db_password, db_table_name}
    }   
}
#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<DataBase>>,
}

impl AppState {
    fn init (db: DataBase) -> Self {
        Self {db: Arc::new(Mutex::new(db))}
    }
}

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Start!");
    let s_data = ServerData::parse_env();

    let listener = tokio::net::TcpListener::bind(s_data.get_adress())
        .await
        .unwrap();

    let app_state = AppState::init(DataBase::connect(s_data.db_username, s_data.db_password, s_data.db_table_name).await);
    let app = Router::new()
        .route("/get_order/:order_uid", get(get_order))
        .route("/save_order", post(save_order))
        .with_state(app_state);

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn save_order(State(app_state) : State<AppState>, Json(payload): Json<WBmodel>) -> Result<Json<Value>> {
    debug!("Handle /save_order with order_uid: {}", payload.order_uid);
    let db = app_state.db.lock().await;
    return match db.add_order(&payload).await {
        Ok(()) => Ok(Json(json!({"status": "success"}))),
        Err(msg) => {error!("Can't save WBmodel to DB: {:?}", msg); Err(Error::DBInsertError)}
    }
}

async fn get_order(State(app_state) : State<AppState>, Path(order_uid): Path<String>) -> Result<Json<WBmodel>> {
    debug!("Handle /get_order with order_uid: {}", order_uid);
    let db = app_state.db.lock().await;

    return match db.get_order(&order_uid).await {
        Ok((data)) => Ok(Json(data)),
        Err(msg) => {error!("Can't find WBmodel in DB: {:?}", msg); Err(Error::DBSelectError)}
    }
}