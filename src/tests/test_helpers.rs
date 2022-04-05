use crate::{make_pg_pool, server, State};
pub use assert_json_diff::assert_json_include;
pub use dotenv::dotenv;
pub use serde_json::{json, Value};
pub use tide::Server;
pub use tide_testing::TideTestingExt;

/// Setup testing
pub async fn test_setup() -> Server<State> {
	dotenv().ok();
	let pool = make_pg_pool("DATABASE_URL_TEST").await.unwrap();
	server(pool)
}
