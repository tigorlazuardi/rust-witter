use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as};
use tide::{Request, Response, StatusCode};
use uuid::Uuid;

use super::{SetBodyJson, State};

#[derive(Debug, Serialize)]
struct User {
	id: Uuid,
	username: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
	username: String,
	password: String,
}

/// Get users from database
pub async fn get_users(req: Request<State>) -> tide::Result<Response> {
	let pool = &req.state().db_pool;

	let users: Vec<User> = query_as!(User, "select id, username from users")
		.fetch_all(pool)
		.await?;

	let mut resp = Response::new(StatusCode::Ok);
	resp.set_body_json(&users)?;
	Ok(resp)
}

// TODO: map invalid data error to 400 status code.

/// Add new user to database
pub async fn post_user(mut req: Request<State>) -> tide::Result<Response> {
	let body: CreateUser = req.body_json().await?;
	let pool = &req.state().db_pool;
	let mut tx = pool.begin().await?;
	query!(
		r#"
			insert into users (id, username)
			values ($1, $2)
		"#,
		Uuid::new_v4(),
		body.username,
	)
	.execute(&mut tx)
	.await?;

	tx.commit().await?;

	let mut resp = Response::new(StatusCode::Created);
	resp.set_body_json(&json!({"message": "user created"}))?;
	Ok(resp)
}
