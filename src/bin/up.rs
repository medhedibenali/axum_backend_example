#[path = "../config.rs"]
mod config;

use config::DB_CONFIG;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let (client, connection) = tokio_postgres::connect(DB_CONFIG, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let query = r#"
        CREATE TABLE "user" (
            id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
            first_name VARCHAR NOT NULL,
            last_name VARCHAR NOT NULL,
            username VARCHAR NOT NULL UNIQUE
        );
    "#;

    client.execute(query, &[]).await?;

    Ok(())
}
