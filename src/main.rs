use std::ffi::OsStr;

use dotenv::dotenv;
use serde_json::{json, Value};
use sqlx::postgres::Postgres;
use sqlx::query;
use sqlx::Pool;
use tide::Request;
use tide::Server;

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
	app
}

/// Root endpoint
pub async fn root_endpoint(req: Request<State>) -> tide::Result<Value> {
	let state = &req.state().db_pool;
	let rows = query!("select 1 as one").fetch_one(state).await?;
	dbg!(rows);
	let js = json!({"message": "server is running"});
	Ok(js)
}

#[derive(Debug, Clone)]
pub struct State {
	db_pool: Pool<Postgres>,
}
