use super::{root_endpoint, server};
use assert_json_diff::assert_json_include;
use dotenv::dotenv;
use serde_json::{json, Value};
use tide_testing::TideTestingExt;

mod test_helpers;

#[async_std::test]
async fn test_root() -> Result<(), Box<dyn std::error::Error>> {
	dotenv().ok();
	let mut server = server().await?;
	server.at("/").get(root_endpoint);
	let got: Value = server.get("/").recv_json().await?;
	let expected = json!({"message": "server is running"});

	assert_json_include!(actual: got, expected: expected);

	Ok(())
}
