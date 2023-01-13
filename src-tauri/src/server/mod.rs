use actix_web::{web, App, HttpServer, Responder, http};
use actix_cors::Cors;
use dotenv;
use std::sync::RwLock;

pub mod public;

async fn not_found() -> impl Responder {
    "Not Found!"
}

#[actix_web::main]
pub async fn start_server(host: &str, port: &str) -> Result<(), sqlx::Error> {
    let admin_db = crate::db::setup_db().await?;
    let app_state = web::Data::new(crate::models::AppState {
            admin_db: admin_db.clone(),
            config: crate::models::config::Config::new(),
            tokens: RwLock::new(std::collections::HashMap::new()),
        });

    println!("Starting server on {}:{}", host, port);
    return Ok(HttpServer::new(move || {
        let cors = if dotenv::var("VITE_DEV").unwrap_or_default() == "true" { Cors::permissive()}
        else {
            Cors::default()
            .allowed_origin("http://0.0.0.0:3000")
            .allowed_origin("http://25.14.132.208:3000")
            .allowed_origin("http://localhost:3000")
            // .allowed_origin_fn(|origin, _req_head| {
            //     origin.as_bytes().ends_with(b".rust-lang.org")
            // })
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600)
        };

        App::new()
        .app_data(app_state.clone())
        .wrap(cors)
        .service(public::auth_service())
        .default_service(web::route().to(not_found))
        })
    .bind((host, port.parse().unwrap()))?
    .run()
    .await?);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }