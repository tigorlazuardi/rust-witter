use crate::server::{server, State};
pub use assert_json_diff::assert_json_include;
pub use dotenv::dotenv;
use rand::{
	distributions::{Alphanumeric, DistString},
	thread_rng,
};
pub use serde_json::{json, Value};
use sqlx::{Pool, Postgres};
pub use tide::Server;
pub use tide_testing::TideTestingExt;

/// Setup testing
pub async fn test_setup() -> (Server<State>, TestDB) {
	dotenv().ok();
	pretty_env_logger::try_init().ok();
	let test_db = create_db_for_testing().await;
	(server(test_db.pool.clone()), test_db)
}

async fn create_db_for_testing() -> TestDB {
	dotenv().ok();
	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let (pg_conn, db_name) = parse_db_url(&db_url);

	let mut rng = thread_rng();
	let tail = Alphanumeric.sample_string(&mut rng, 5);

	let db_name = db_name.to_owned() + &tail;

	let admin_pool = Pool::<Postgres>::connect(pg_conn).await.unwrap();

	let query = format!(
		r#"
		CREATE DATABASE "{db}"
		"#,
		db = db_name
	);

	sqlx::query::<Postgres>(&query)
		.execute(&admin_pool)
		.await
		.expect("failed to create test database");

	let pg_conn = pg_conn.to_owned() + "/" + &db_name;
	let pool = Pool::<Postgres>::connect(&pg_conn).await.unwrap();

	let query = async_std::fs::read_to_string("setup/setup.sql")
		.await
		.expect("failed to read setup.sql");

	for query in query.split(';') {
		if query.trim().is_empty() {
			continue;
		}
		sqlx::query::<Postgres>(query)
			.execute(&pool)
			.await
			.expect("failed to run setup query");
	}

	TestDB {
		admin_pool,
		db_name,
		pool,
	}
}

#[derive(Debug)]
pub struct TestDB {
	db_name: String,
	admin_pool: Pool<Postgres>,
	pool: Pool<Postgres>,
}

impl Drop for TestDB {
	fn drop(&mut self) {
		let query = format!(
			r#"
			SELECT pg_terminate_backend(pg_stat_activity.pid)
			FROM pg_stat_activity
			WHERE pg_stat_activity.datname = '{db}'
			AND pid <> pg_backend_pid();
			"#,
			db = self.db_name
		);

		async_std::task::block_on(async {
			sqlx::query::<Postgres>(&query)
				.execute(&self.admin_pool)
				.await
				.unwrap();
		});

		let query = format!(
			r#"
			DROP DATABASE IF EXISTS "{db}";
			"#,
			db = self.db_name
		);
		async_std::task::block_on(async {
			sqlx::query::<Postgres>(&query)
				.execute(&self.admin_pool)
				.await
				.unwrap();
		});
	}
}

/// Returns (db_name, postgres_con)
pub fn parse_db_url(db_url: &str) -> (&str, &str) {
	let separator_pos = db_url.rfind('/').unwrap();
	let pg_conn = &db_url[..separator_pos];
	let db_name = &db_url[separator_pos + 1..];

	(pg_conn, db_name)
}
