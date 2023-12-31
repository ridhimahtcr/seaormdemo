use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn establish_connection() -> DatabaseConnection {
    let mut opt = ConnectOptions::new("postgres://postgres:root@localhost/seaorm");
    opt.max_connections(100)
        .min_connections(5)
        /*.connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
      */
        .sqlx_logging(false);
        //.sqlx_logging_level(log::LevelFilter::Info)
        //.set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await.unwrap();

    return db;
}


