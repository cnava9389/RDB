use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::LitePool;
use crate::utils::{u8_to_T, T_to_u8, Error};

#[derive(Deserialize, Serialize)]
pub struct DbUser {
    id: Uuid,
    ids: HashSet<(Uuid, String)>,
}

impl DbUser {
    pub fn new(id: Uuid) -> Self {
        Self { id, ids: HashSet::new() }
    }

    pub async fn ids(&mut self, pool: &LitePool) -> Result<HashSet<(Uuid, String)>, Error> {
        if self.ids.is_empty() {
            let mut new_set = HashSet::new();
            for x in u8_to_T::<String>(&sqlx::query!(r#"SELECT relation_ids FROM DbUser WHERE id = ?;"#, self.id)
                .fetch_one(pool)
                .await?.relation_ids)? {
                if x.len() < 3 {
                    continue;
                }
                let mut split = x.split_whitespace();
                let id = split.next().unwrap();
                let other = split.next().unwrap();
                new_set.insert((Uuid::parse_str(id).unwrap(), other.to_string()));
            }
            self.ids = new_set;
        };
        Ok(self.ids.clone())
    }

    pub async fn create_entry(self, pool: &LitePool) -> Result<Self, Error> {
        sqlx::query(r#"INSERT INTO DbUser (id, relation_ids) VALUES (?, ?);"#)
            .bind(self.id)
            .bind(T_to_u8(&self.ids.iter().map(|x| format!("{} {}", x.0, x.1)).collect()))
            .execute(pool)
            .await?;
        Ok(self)
    }

    pub async fn update_ids(&self, pool: &LitePool) -> Result<(), Error> {
        sqlx::query(r#"UPDATE DbUser SET relation_ids = ? WHERE id = ?;"#)
            .bind(T_to_u8(&self.ids.iter().map(|x| format!("{} {}", x.0, x.1)).collect()))
            .bind(self.id)
            .execute(pool)
            .await?;
        Ok(())
    }
    pub async fn add_id(&mut self, id: (Uuid, String),) -> bool {
        self.ids.insert(id)
    }
}

