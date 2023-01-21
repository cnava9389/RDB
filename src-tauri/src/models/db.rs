use serde::{Deserialize, Serialize};
use crate::utils::Error;
use super::{LitePool, Pool};

#[derive(Deserialize, Serialize)]
pub struct Db {
    id: uuid::Uuid,
    uri: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pool: Pool
}
impl Db {
    pub async fn new(uri: String) -> Result<Self, Error> {
        let pool = if uri.starts_with("sqlite://") {
            Pool::Lite(sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&uri)
            .await
            .expect("Error building a connection to admin database"))
        } else {
            Pool::Post(sqlx::postgres::PgPool::connect(&uri).await?)
        };
        Ok(Self { id: uuid::Uuid::new_v4(), uri, pool })
    }

    pub fn name(&self) -> String {
        let name = self.uri.split('/').last().unwrap();
        if name.ends_with(".db") || name.ends_with(".sqlite") {
            name.split('.').next().unwrap().to_string()
        } else {
            name.to_string()
        }
    }
    
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
    
    pub fn pool(&self) -> &Pool {
        &self.pool
    }
    
    pub async fn create_entry(self, pool: &LitePool) -> Result<Db, Error> {
        sqlx::query(r#"INSERT INTO Db (id, uri) VALUES (?, ?);"#)
            .bind(self.id)
            .bind(&self.uri)
            .execute(pool)
            .await?;
        Ok(self)
    }

}
