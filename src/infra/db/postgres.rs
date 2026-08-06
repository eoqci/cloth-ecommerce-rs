use sqlx::ConnectOptions;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    let options: sqlx::postgres::PgConnectOptions = database_url.parse()?;

    PgPoolOptions::new()
        .max_connections(10)
        // turn off log query
        .connect_with(options.disable_statement_logging())
        .await
}
