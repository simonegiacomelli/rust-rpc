use std::fs;

use sqlx::migrate::Migrator;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::SqlitePool;
use rpc_api::rpc::properties::properties;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let env = fs::read_to_string(".env")?;
    let props = properties(&env);
    let conn_string = props.get("DATABASE_URL").unwrap();
    let pool = SqlitePool::connect(&conn_string).await?;


    println!("sqlx_migrate.rs");
    sqlx::migrate!()
        .run(&pool)
        .await?;
    Ok(())
}

