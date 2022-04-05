use std::ffi::OsStr;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{postgres::Postgres, query_as};
use sqlx::{query, Pool};
use tide::{Request, Response};
use tide::{Server, StatusCode};
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[async_std::main]
async fn main() -> Result<(), Error> {
	dotenv().ok();
	pretty_env_logger::init();
	let app = server(make_pg_pool("DATABASE_URL").await?);
	app.listen("127.0.0.1:5000").await?;
	Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	DbError(#[from] sqlx::Error),

	#[error(transparent)]
	IoError(#[from] std::io::Error),

	#[error(transparent)]
	EnvVarError(#[from] std::env::VarError),
}

pub async fn make_pg_pool<S: AsRef<OsStr>>(pg_env: S) -> Result<Pool<Postgres>, Error> {
	let db_url = std::env::var(pg_env.as_ref())?;
	Ok(Pool::<Postgres>::connect(&db_url).await?)
}

/// description
pub fn server(pool: Pool<Postgres>) -> Server<State> {
	let mut app: Server<State> = Server::with_state(State { db_pool: pool });
	app.at("/").get(root_endpoint);
	app.at("/users").get(get_users).post(post_user);
	app
}

/// Root endpoint
pub async fn root_endpoint(_: Request<State>) -> tide::Result<Value> {
	let js = json!({"message": "server is running"});
	Ok(js)
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
	query!(
		r#"
			insert into users (id, username)
			values ($1, $2)
		"#,
		Uuid::new_v4(),
		body.username,
	)
	.execute(pool)
	.await?;

	let mut resp = Response::new(StatusCode::Created);
	resp.set_body_json(&json!({"message": "user created"}))?;
	Ok(resp)
}

pub trait SetBodyJson {
	fn set_body_json<T: Serialize>(&mut self, t: &T) -> Result<(), serde_json::Error>;
}

impl SetBodyJson for Response {
	fn set_body_json<T: Serialize>(&mut self, t: &T) -> Result<(), serde_json::Error> {
		let val = serde_json::to_value(t)?;
		self.set_body(val);
		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct State {
	db_pool: Pool<Postgres>,
}

#[derive(Debug, Serialize)]
struct User {
	id: Uuid,
	username: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
	username: String,
}
