use sqlx::postgres::PgPoolOptions;

pub async fn init() {
    // create the database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:admin@localhost/heimdallr")
        .await
        .unwrap();
    // run the migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
}
