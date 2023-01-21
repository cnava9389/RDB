use serde::{Deserialize, Serialize};
use tauri::async_runtime::block_on;
use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use tauri::async_runtime::RwLock;
use sqlx;

use self::db::Db;

pub mod users;
pub mod config;
pub mod error;
pub mod db_user;
pub mod db;
pub mod permissions;
#[derive(Deserialize)]
pub struct Credentials {
    pub id: String,
    pub password: String,
    pub name: Option<String>,
}

pub struct CacheObj {
    pub dbs_users: Option<HashSet<(Uuid, String)>>,
    pub db: Option<db::Db>,
    pub permissions: Option<permissions::Permissions>,
    pub user: Option<users::User>,
}

pub struct AppState {
    pub admin_db: RwLock<Db>,
    pub config: config::Config,
    pub tokens: RwLock<HashMap<String, uuid::Uuid>>,
    pub cache: RwLock<HashMap<uuid::Uuid, CacheObj>>,
}

pub type LitePool = sqlx::Pool<sqlx::Sqlite>;
pub type PostPool = sqlx::Pool<sqlx::Postgres>;

pub enum Pool {
    Lite(LitePool),
    Post(PostPool),
}
impl Default for Pool {
    fn default() -> Self {
        Self::Lite(block_on(LitePool::connect(&dotenv::var("DATABASE_URL").unwrap_or_default())).unwrap())
    }
}

#[derive(Serialize)]
pub struct ClientState {
    user: Option<users::User>,
    dbs: Option<HashSet<(Uuid,String)>>,
}

impl ClientState {
    pub fn new(user: Option<users::User>, dbs: Option<HashSet<(Uuid, String)>>) -> Self {
        Self { user, dbs }
    }
}