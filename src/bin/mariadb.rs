use std::any::type_name;
use std::fs;

use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("mariadb");
    let conn_string = fs::read_to_string("connection_string.txt")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&conn_string).await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let recs = sqlx::query!(
        r#"
SELECT id_azienda, cod_area
FROM anag_aree
        "#
    )
        .fetch_all(&pool)
        .await?;

    // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
    //       0 = false, non-0 = true
    for rec in recs {
        println!(
            " {} {}",
            rec.id_azienda,
            &rec.cod_area,
        );
    }

    Ok(())
}

