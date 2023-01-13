use serde::{Deserialize};
use std::collections::HashMap;
use std::sync::RwLock;

pub mod users;
pub mod config;
pub mod error;

use users::Pool;

#[derive(Deserialize)]
pub struct Credentials {
    pub id: String,
    pub password: String,
    pub name: Option<String>,
}

pub struct AppState {
    pub admin_db: sqlx::Pool<sqlx::sqlite::Sqlite>,
    pub config: config::Config,
    pub tokens: RwLock<HashMap<String, uuid::Uuid>>
}

