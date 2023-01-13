
use actix_web::{web, Responder, Scope, HttpRequest, HttpResponse};

use crate::models::{Credentials, AppState, users::{User, ID}};


async fn login(credentials: web::Json<Credentials>, app_state: web::Data<AppState>) -> impl Responder {
    match User::get_user(&app_state.admin_db, &credentials.id, &credentials.password).await {
        Ok(user) => {
            let token = uuid::Uuid::new_v4().to_string();
            app_state.tokens.write().unwrap().insert(token.clone(), user.id());
            Ok(HttpResponse::Ok()
                    .cookie(cookie::Cookie::build("RDB", token)
                        .path("/")
                        .http_only(true)
                        .max_age(cookie::time::Duration::hours(8))
                        .finish())
                    .json(user))
        },
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e.to_string()))
    }
}

async fn logout() -> impl Responder {
    "Logout"
}

async fn create(credentials: web::Json<Credentials>, app_state: web::Data<AppState>) -> impl Responder {
    match User::create_user(&app_state.admin_db, User::new(credentials.name.clone(), &credentials.id, &credentials.password)).await {
        Ok(user) => Ok(user),
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e.to_string()))
    }
}

async fn account(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let cookie = match req.headers().get("cookie") {
        Some(cookie) => cookie.to_str().unwrap_or_default().split("=").nth(1).unwrap_or_default().to_string(),
        None => return Err(actix_web::error::ErrorUnauthorized("No cookie"))
    };

    match app_state.tokens.read().unwrap().get(&cookie){
        Some(id) => {
            match User::fetch_user(&app_state.admin_db, ID::Uuid(id)).await {
                Ok(user) => Ok(user),
                Err(e) => Err(actix_web::error::ErrorUnauthorized(e.to_string()))
            }
        },
        None => Err(actix_web::error::ErrorUnauthorized("Invalid cookie"))
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