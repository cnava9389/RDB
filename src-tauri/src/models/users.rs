use actix_web::{HttpRequest, HttpResponse, http::header::ContentType};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

use super::LitePool;
use crate::utils::Error;

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(skip_serializing)]
    id: uuid::Uuid,
    name: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
}
// !TODO take care of errors

pub enum ID<'a> {
    Email(&'a str),
    Uuid(&'a uuid::Uuid),
}

impl User {
    pub fn new(name: Option<String>, email: &str, password: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.unwrap_or_default(),
            email: email.into(),
            password: password.into(),
        }
    }
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }
    
    pub async fn fetch_user(pool: &LitePool, id: ID<'_>) -> Result<User, Error> {
        match id {
            ID::Uuid(uuid) => Ok(sqlx::query_as!(Self, r#"SELECT id AS "id: uuid::Uuid", name, email, password AS "password: String" FROM Users WHERE id = ?;"#, uuid)
        .fetch_one(pool).await?),
            ID::Email(email) => Ok(sqlx::query_as!(Self, r#"SELECT id AS "id: uuid::Uuid", name, email, password AS "password: String" FROM Users WHERE email = ?;"#, email)
        .fetch_one(pool).await?),
        }
    }

    pub async fn get_user(pool: &LitePool, email: &str, password: &str) -> Result<User, Error> {
        let user = Self::fetch_user(pool, ID::Email(email)).await?;
        Argon2::default().verify_password(password.as_bytes(), &PasswordHash::new(&user.password).map_err(|e| Error::Other(e.to_string()))?).map_err(|e| Error::Other(e.to_string()))?;
        Ok(user)
    }

    pub async fn create_user(pool: &LitePool, user: User) -> Result<User, Error> {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(user.password.as_bytes(), &salt).map_err(|e| Error::Other(e.to_string()))?.to_string();
        sqlx::query_as!(Self, r#"INSERT INTO Users (id, name, email, password) VALUES (?, ?, ?, ?)"#, user.id, user.name, user.email, password_hash)
        .execute(pool)
        .await?;

        Ok( User::fetch_user(pool, ID::Email(&user.email)).await?)
    }

    pub async fn delete_user(pool: &LitePool, email:&str) -> Result<(), Error> {
        sqlx::query!(r#"DELETE FROM Users WHERE email = ?"#, email)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update_user(self, pool: &LitePool) -> Result<Self, Error> {

        sqlx::query!(r#"UPDATE Users SET name = ?, email = ? WHERE id = ?"#, self.name, self.email, self.id)
        .execute(pool)
        .await?;
        Ok(self)
    }
}

impl actix_web::Responder for User {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}