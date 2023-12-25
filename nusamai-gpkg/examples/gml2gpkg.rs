use sqlx::Row;

#[tokio::main]
async fn main() {
    // TODO: Handle output path
    let output_path = "output.gpkg";
    let db_url: &str = &format!("sqlite://{}", output_path);

    let pool = nusamai_gpkg::init_gpkg(db_url).await.unwrap();

    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table'
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }
}
