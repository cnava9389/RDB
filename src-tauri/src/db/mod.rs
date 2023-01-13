use std::str::FromStr;

use sqlx::{self, Pool, Sqlite, sqlite, ConnectOptions};

pub async fn setup_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let url = dotenv::var("DATABASE_URL").unwrap_or_default();

    {
        let mut conn = sqlite::SqliteConnectOptions::from_str(&url)?
        .create_if_missing(true)
        .connect()
        .await?;
        sqlx::migrate!().run(&mut conn).await?;
    }

    let pool = sqlite::SqlitePoolOptions::new()
    .max_connections(5)
    .connect(&url)
    .await
    .expect("Error building a connection to admin database");

    // let pool = PgPoolOptions::new()
    // .connect(&url)
    // .await
    // .expect("Error building a connection to admin database");
    Ok(pool)
}


