use std::ffi::OsStr;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{postgres::Postgres, query_as};
use sqlx::{query, Pool};
use tide::{Middleware, Request, Response};
use tide::{Server, StatusCode};
use uuid::Uuid;

#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, Error>;

#[async_std::main]
async fn main() -> Result<()> {
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

	#[error(transparent)]
	DecodeError(#[from] serde_json::Error),
}

pub async fn make_pg_pool<S: AsRef<OsStr>>(pg_env: S) -> Result<Pool<Postgres>> {
	let db_url = std::env::var(pg_env.as_ref())?;
	Ok(Pool::<Postgres>::connect(&db_url).await?)
}

/// description
pub fn server(pool: Pool<Postgres>) -> Server<State> {
	let mut server: Server<State> = Server::with_state(State { db_pool: pool });
	server.at("/").get(root_endpoint);
	server.at("/users").get(get_users).post(post_user);
	server
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

pub trait SetBodyJson {
	fn set_body_json<T: Serialize>(&mut self, t: &T) -> Result<()>;
}

impl SetBodyJson for Response {
	fn set_body_json<T: Serialize>(&mut self, t: &T) -> Result<()> {
		let val = serde_json::to_value(t)?;
		self.insert_header("Content-Type", "application/json; utf-8");
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
