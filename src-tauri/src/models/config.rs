use uuid;
use crate::models::LitePool as Pool;
use sqlx;

#[derive(sqlx::FromRow)]
pub struct Config {
    id: uuid::Uuid,
    cache: bool,
    ttl_in_ms: i64 
}

impl Config {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            cache: false,
            ttl_in_ms: 0
        }
    }

    pub async fn add_config(self, pool: &Pool) -> Result<Config, sqlx::Error>{
        sqlx::query!(r#"INSERT INTO Config (id, cache, ttl) VALUES (?, ?, ?)"#, self.id, self.cache, self.ttl_in_ms)
        .fetch_one(pool)
        .await?;
        Ok(self)
    }

    pub async fn update_config(self, pool: &Pool) -> Result<Config, sqlx::Error> {
        sqlx::query!(r#"UPDATE Config SET cache = ?, ttl = ? WHERE id = ?"#, self.cache, self.ttl_in_ms, self.id)
        .fetch_one(pool)
        .await?;
        Ok(self)
    }

    pub async fn get_config(pool: &Pool, id: &uuid::Uuid) -> Result<Config, sqlx::Error> {
        Ok(sqlx::query_as!(Config, r#"SELECT id AS "id: uuid::Uuid", cache, ttl AS "ttl_in_ms" from Config WHERE id =?"#, id)
        .fetch_one(pool)
        .await?)
    }

    
    pub async fn delete_config(pool: &Pool, id: &uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM Config WHERE id = ?"#, id)
        .execute(pool)
        .await?;
        Ok(())
    }

}