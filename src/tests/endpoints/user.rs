use assert_json_diff::{assert_json_eq, assert_json_include};
use tide::StatusCode;

use crate::tests::test_helpers::*;

#[async_std::test]
async fn list_users() {
	let (server, _db) = test_setup().await;
	let mut resp = server.get("/users").send().await.unwrap();
	assert_eq!(resp.status(), StatusCode::Ok);
	let got: Vec<Value> = resp.body_json().await.unwrap();
	assert_json_eq!(got, json!([]));
}

#[async_std::test]
async fn create_user() {
	let (server, _db) = test_setup().await;
	let mut resp = server
		.post("/users")
		.body(json!({"username": "tigor", "password": "123"}))
		.send()
		.await
		.unwrap();

	if resp.status() != StatusCode::Created {
		println!("{}", resp.body_string().await.unwrap());
		panic!("expected 201, got {}", resp.status());
	}

	let got: Value = resp.body_json().await.unwrap();
	assert_json_include!(actual: got, expected: json!({"message": "user created"}));

	let mut resp = server.get("/users").send().await.unwrap();
	assert_eq!(resp.status(), StatusCode::Ok);
	let got: Vec<Value> = resp.body_json().await.unwrap();
	assert_json_include!(actual: got, expected: json!([{"username": "tigor"}]));

	let resp = server
		.post("/users")
		.body(json!({"username": "tigor"}))
		.send()
		.await
		.unwrap();

	assert_eq!(resp.status(), StatusCode::BadRequest);
}
