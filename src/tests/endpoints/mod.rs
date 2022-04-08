use crate::tests::test_helpers::*;

#[cfg(test)]
mod user;

#[async_std::test]
async fn test_root() -> Result<(), Box<dyn std::error::Error>> {
	let (server, _db) = test_setup().await;
	let got: Value = server.get("/").recv_json().await?;
	let expected = json!({"message": "server is running"});

	assert_json_include!(actual: got, expected: expected);

	Ok(())
}
