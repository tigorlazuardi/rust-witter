use super::*;
use assert_json_diff::assert_json_include;
use serde_json::json;
use tide_testing::TideTestingExt;

#[async_std::test]
async fn test_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut server = server().await?;
    server.at("/").get(root_endpoint);
    let raw = server.get("/").recv_string().await?;
    let got: Value = serde_json::from_str(&raw)?;
    let expected = json!({"message": "server is running"});

    assert_json_include!(actual: got, expected: expected);

    Ok(())
}
