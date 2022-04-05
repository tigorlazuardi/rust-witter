#![allow(unused_imports)]
mod test_helpers;

use super::{root_endpoint, server};

use test_helpers::*;

#[async_std::test]
async fn test_root() -> Result<(), Box<dyn std::error::Error>> {
	let server = test_setup().await;
	let got: Value = server.get("/").recv_json().await?;
	let expected = json!({"message": "server is running"});

	assert_json_include!(actual: got, expected: expected);

	Ok(())
}
