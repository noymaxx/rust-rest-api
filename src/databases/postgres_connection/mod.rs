use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn start_connection() -> Pool<Postgres> {
  let postgres_environment = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&postgres_environment)
    .await
    .expect("Failed to connect to Postgres.");

  let check_migrate = sqlx::migrate!("./src/databases/postgres_connection/migrations")
    .run(&pool)
    .await;

  match check_migrate {
    Ok(_) => println!("Migrations run successfully"),
    Err(e) => println!("Error running migrations: {:?}", e),
  }

  pool
}