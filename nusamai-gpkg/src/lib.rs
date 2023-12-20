use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

pub async fn init_gpkg(db_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    if Sqlite::database_exists(db_url).await.unwrap_or(true) {
        panic!("The database already exists at {}", db_url);
    }

    Sqlite::create_database(db_url).await?;
    let pool = SqlitePool::connect(db_url).await?;

    let create_query = include_str!("sql/gpkg_template.sql");
    sqlx::query(create_query).execute(&pool).await?;

    Ok(pool)
}
