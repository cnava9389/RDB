use std::str::FromStr;

use sqlx::{self, sqlite, ConnectOptions};
use crate::utils::Error;

pub async fn setup_db() -> Result<(), Error> {
    let mut conn = sqlite::SqliteConnectOptions::from_str(&dotenv::var("DATABASE_URL").unwrap_or_default())?
    .create_if_missing(true)
    .connect()
    .await?;
    sqlx::migrate!().run(&mut conn).await.map_err(|_|Error::Other("Migration error".into()))?;
    Ok(())
}


