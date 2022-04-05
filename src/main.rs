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
	let app = server().await?;
	app.listen("127.0.0.1:5000").await?;
	Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
	#[error(transparent)]
	DbError(#[from] sqlx::Error),

	#[error(transparent)]
	IoError(#[from] std::io::Error),

	#[error(transparent)]
	EnvVarError(#[from] std::env::VarError),
}

/// description
async fn server() -> Result<Server<State>, Error> {
	let db_url = std::env::var("DATABASE_URL")?;
	let pool = Pool::<Postgres>::connect(&db_url).await?;

	let mut app: Server<State> = Server::with_state(State { db_pool: pool });
	app.at("/").get(root_endpoint);

	Ok(app)
}

/// Root endpoint
async fn root_endpoint(req: Request<State>) -> tide::Result<Value> {
	let state = &req.state().db_pool;
	let rows = query!("select 1 as one").fetch_one(state).await?;
	dbg!(rows);
	let js = json!({"message": "server is running"});
	Ok(js)
}

#[derive(Debug, Clone)]
struct State {
	db_pool: Pool<Postgres>,
}
