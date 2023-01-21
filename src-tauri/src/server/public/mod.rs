
use actix_web::{web, Responder, Scope, HttpRequest, HttpResponse};

use crate::models::{Credentials, AppState, users::{User, ID}, db_user::DbUser, ClientState, CacheObj, Pool};


async fn login(credentials: web::Json<Credentials>, app_state: web::Data<AppState>) -> impl Responder {
    match app_state.admin_db.read().await.pool() {
        Pool::Lite(pool) => {
            let user = User::get_user(pool, &credentials.id, &credentials.password).await?;
            let mut cache = app_state.cache.write().await;
            let cache = cache.entry(user.id()).or_insert(CacheObj{
                dbs_users: Some(DbUser::new(user.id()).ids(pool).await?),
                db: None,
                permissions: None, //Some(Permissions::new(user.id()).create_entry(pool).await?),
                user: Some(user),
            });

            let user = cache.user.as_ref().unwrap().clone();
            let dbs_users = cache.dbs_users.as_ref().unwrap().clone();

            let token = uuid::Uuid::new_v4().to_string();
            app_state.tokens.write().await.insert(token.clone(), user.id());
            
            Ok(HttpResponse::Ok()
                    .cookie(cookie::Cookie::build("RDB", token)
                        .path("/")
                        .http_only(true)
                        .max_age(cookie::time::Duration::hours(8))
                        .finish())
                    .json(ClientState::new(Some(user), Some(dbs_users))))       
        },
        _ => Err(actix_web::error::ErrorBadRequest("Invalid connection type")),
    }
}

async fn logout() -> impl Responder {
    Ok::<_, actix_web::Error>(HttpResponse::Ok()
                    .cookie(cookie::Cookie::build("RDB", "")
                        .path("/")
                        .http_only(true)
                        .max_age(cookie::time::Duration::microseconds(0))
                        .finish()).finish())
}

// ! check return
async fn create(credentials: web::Json<Credentials>, app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.admin_db.read().await;
    match db.pool() {
        Pool::Lite(pool) => {
            match User::create_user(pool, User::new(credentials.name.clone(), &credentials.id, &credentials.password)).await {
                Ok(user) => {
                    let mut res = DbUser::new(user.id());
                    if user.email().to_lowercase() == "cnava9389@gmail.com" {
                        res.add_id((db.id(), db.name())).await;
                    };
                    res.create_entry(pool).await?;
                    Ok(HttpResponse::Ok().finish())
                },
                Err(e) => Err(actix_web::error::ErrorBadRequest(e))
            }     
        },
        _ => Err(actix_web::error::ErrorBadRequest("Invalid connection type")),
    }
}

async fn account(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder{
    let cookie = match req.headers().get("cookie") {
        Some(cookie) => cookie.to_str().unwrap_or_default().split("=").nth(1).unwrap_or_default().to_string(),
        None => return Err(actix_web::error::ErrorUnauthorized("No cookie"))
    };
    match app_state.admin_db.read().await.pool() {
        Pool::Lite(pool) => {
          match app_state.tokens.read().await.get(&cookie){
            Some(id) => {
                match User::fetch_user(pool, ID::Uuid(id)).await {
                    Ok(user) => {
                        if let Some(cache) = app_state.cache.read().await.get(&user.id()) {
                            Ok(web::Json(ClientState::new(Some(cache.user.as_ref().unwrap().clone()), Some(cache.dbs_users.as_ref().unwrap().clone()))))
                        } else {
                            Err(actix_web::error::ErrorBadRequest("User not found in cache"))
                        }
                    },
                    Err(e) => Err(actix_web::error::ErrorBadRequest(e))
                }
            },
            None => Err(actix_web::error::ErrorBadRequest("Invalid cookie"))
        } 
        },
        _ => Err(actix_web::error::ErrorBadRequest("Invalid connection type")),
    }
}

pub fn auth_service () -> Scope {
    web::scope("/auth")
    .route("/login", web::post().to(login))
    .route("/logout", web::get().to(logout))
    .route("/create", web::post().to(create))
    .route("/account", web::get().to(account))
    .default_service(web::route().to(crate::server::not_found))
}