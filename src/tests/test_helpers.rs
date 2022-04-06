use crate::{make_pg_pool, server, State};
pub use assert_json_diff::assert_json_include;
pub use dotenv::dotenv;
pub use serde_json::{json, Value};
use sqlx::{Connection, PgConnection, Postgres};
pub use tide::Server;
pub use tide_testing::TideTestingExt;

/// Setup testing
pub async fn test_setup() -> Server<State> {
	dotenv().ok();
	let var = "DATABASE_URL_TEST";
	let env_var = std::env::var(var).unwrap();
	drop_db(&env_var).await;
	exec_migrations(&env_var).await;
	let pool = make_pg_pool(var).await.unwrap();
	server(pool)
}

pub async fn drop_db(db_url: &str) {
	let (db_name, pg_conn) = parse_db_url(db_url);

	let mut conn = PgConnection::connect(pg_conn).await.unwrap();

	let query = format!(
		r#"
		SELECT pg_terminate_backend(pg_stat_activity.pid)
		FROM pg_stat_activity
		WHERE pg_stat_activity.datname = '{db}'
		AND pid <> pg_backend_pid();
		"#,
		db = db_name
	);

	sqlx::query::<Postgres>(&query)
		.execute(&mut conn)
		.await
		.unwrap();

	let query = format!(
		r#"
        DROP DATABASE IF EXISTS "{db}";
        "#,
		db = db_name
	);

	sqlx::query::<Postgres>(&query)
		.execute(&mut conn)
		.await
		.unwrap();
}

pub async fn exec_migrations(db_url: &str) {
	let (db_name, pg_con) = parse_db_url(db_url);

	let mut conn = PgConnection::connect(pg_con).await.unwrap();

	let query = format!(
		r#"
		CREATE DATABASE "{db}"
		"#,
		db = db_name
	);

	sqlx::query::<Postgres>(&query)
		.execute(&mut conn)
		.await
		.unwrap();

	drop(conn);
	let mut conn = PgConnection::connect(db_url).await.unwrap();

	let query = async_std::fs::read_to_string("setup/setup.sql")
		.await
		.unwrap();

	sqlx::query::<Postgres>(&query)
		.execute(&mut conn)
		.await
		.unwrap();
}

/// Returns (db_name, postgres_con)
pub fn parse_db_url(db_url: &str) -> (&str, &str) {
	let separator_pos = db_url.rfind('/').unwrap();
	let db_name = &db_url[separator_pos + 1..];
	let pg_conn = &db_url[..separator_pos];

	(db_name, pg_conn)
}
