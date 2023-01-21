use std::{collections::HashSet};
use actix_web::ResponseError;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("uuid error: {0}")]
    Uuid(#[from] uuid::Error),
    #[error("other error: {0}")]
    Other(String)
}

impl ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            Error::Sqlx(e) => {
                match e {
                    sqlx::Error::Database(e) => {
                        actix_web::HttpResponse::BadRequest().body(e.to_string())
                    },
                    sqlx::Error::RowNotFound => {
                        actix_web::HttpResponse::NotFound().body(e.to_string())
                    },
                    _ => actix_web::HttpResponse::InternalServerError().body(e.to_string())
                }
            },
            Error::Io(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
            Error::Serde(e) => actix_web::HttpResponse::Conflict().body(e.to_string()),
            Error::Uuid(e) => actix_web::HttpResponse::Conflict().body(e.to_string()),
            Error::Other(e) => actix_web::HttpResponse::NotFound().body(e.to_string())
        }
    }
}

pub fn u8_to_T<T>(bytes: &[u8]) -> Result<HashSet<T>, Error>
where T: std::fmt::Display + std::fmt::Debug + std::str::FromStr + std::cmp::Eq + std::hash::Hash {
        Ok(std::str::from_utf8(bytes).unwrap_or_default().split(",").map(|x| {
            x.parse::<T>().map_err(|_| Error::Other("Format error".into())).unwrap()
        }).collect::<HashSet<T>>())
}

pub fn T_to_u8<T>(set: &HashSet<T>) -> Vec<u8>
where T: std::fmt::Display + std::fmt::Debug + std::str::FromStr + std::cmp::Eq + std::hash::Hash {
    set.into_iter().fold(Vec::new(), |mut acc, v| {
        acc.extend_from_slice(format!("{},", v.to_string()).as_bytes());
        acc
    })
}