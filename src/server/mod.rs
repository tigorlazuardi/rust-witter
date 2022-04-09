use std::ffi::OsStr;

use serde::Serialize;
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};
use tide::{Request, Response, Server};

use crate::Result;

use self::users::{get_users, post_user};

pub mod middleware;
pub mod users;

pub fn server(pool: Pool<Postgres>) -> Server<State> {
	let mut server: Server<State> = Server::with_state(State { db_pool: pool });
	server.at("/").get(root_endpoint);
	server.at("/users").get(get_users).post(post_user);
	server
}

pub async fn make_pg_pool<S: AsRef<OsStr>>(pg_env: S) -> Result<Pool<Postgres>> {
	let db_url = std::env::var(pg_env.as_ref())?;
	Ok(Pool::<Postgres>::connect(&db_url).await?)
}

#[derive(Debug, Clone)]
pub struct State {
	db_pool: Pool<Postgres>,
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

/// Root endpoint
pub async fn root_endpoint(_: Request<State>) -> tide::Result<Value> {
	let js = json!({"message": "server is running"});
	Ok(js)
}
