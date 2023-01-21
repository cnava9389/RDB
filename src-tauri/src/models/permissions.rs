use std::collections::HashSet;
use super::LitePool;
use crate::utils::{u8_to_T, T_to_u8, Error};
pub struct Permissions {
    id: uuid::Uuid,
    read: HashSet<uuid::Uuid>,
    write: HashSet<uuid::Uuid>,
}

impl Permissions {
    pub fn new(id: uuid::Uuid) -> Self {
        Self { id, read: HashSet::new(), write: HashSet::new() }
    }

    pub async fn permissions(&mut self, pool: &LitePool) -> Result<(HashSet<uuid::Uuid>, HashSet<uuid::Uuid>), Error> {
        if self.read.is_empty() {
            self.read = u8_to_T(&sqlx::query!(r#"SELECT read FROM Permissions WHERE id = ?;"#, self.id)
                .fetch_one(pool)
                .await?.read)?;
        };
        if self.write.is_empty() {
            self.write = u8_to_T(&sqlx::query!(r#"SELECT write FROM Permissions WHERE id = ?;"#, self.id)
                .fetch_one(pool)
                .await?.write)?;
        }
        Ok((self.read.clone(), self.write.clone()))
    }

    pub async fn create_entry(self, pool: &LitePool) -> Result<Self, Error> {
        sqlx::query(r#"INSERT INTO Permissions (id, read, write) VALUES (?, ?, ?);"#)
            .bind(self.id)
            .bind(T_to_u8(&self.read))
            .bind(T_to_u8(&self.write))
            .execute(pool)
            .await?;
        Ok(self)
    }

    pub async fn update_permissions(&self, pool: &LitePool) -> Result<(), Error> {
        sqlx::query(r#"UPDATE Permissions SET read = ?, write = ? WHERE id = ?;"#)
            .bind(T_to_u8(&self.read))
            .bind(T_to_u8(&self.write))
            .bind(self.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub fn add_to_read(&mut self, id: uuid::Uuid) -> bool {
        self.read.insert(id)
    }
    
    pub fn add_to_write(&mut self, id: uuid::Uuid) -> bool {
        self.write.insert(id)
    }

    pub fn add_to_both(&mut self, id: uuid::Uuid) ->(bool, bool){
        (self.add_to_read(id), self.add_to_write(id))
    }

}