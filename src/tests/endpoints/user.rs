use assert_json_diff::assert_json_eq;
use tide::StatusCode;

use crate::tests::test_helpers::*;

#[async_std::test]
async fn create_a_user() {
	let server = test_setup().await;
	let mut resp = server.get("/users").send().await.unwrap();
	assert_eq!(resp.status(), StatusCode::Ok);
	let got: Vec<Value> = resp.body_json().await.unwrap();
	assert_json_eq!(got, json!([]));
}
