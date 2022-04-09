use dotenv::dotenv;

use self::server::make_pg_pool;

pub mod server;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, Error>;

#[async_std::main]
async fn main() -> Result<()> {
	dotenv().ok();
	pretty_env_logger::init();
	let app = server::server(make_pg_pool("DATABASE_URL").await?);
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
