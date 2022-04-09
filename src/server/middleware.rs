use std::{future::Future, pin::Pin};

use serde_json::json;
use tide::{Middleware, Next, Request};

use super::State;

#[derive(Debug, Default)]
pub struct ErrorMiddleware;

impl Middleware<State> for ErrorMiddleware {
	fn handle<'life0, 'life1, 'async_trait>(
		&'life0 self,
		request: Request<State>,
		next: Next<'life1, State>,
	) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'async_trait>>
	where
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		Box::pin(async move {
			let mut res = next.run(request).await;
			if let Some(err) = res.take_error() {
				res.set_body(json!({
					"message": "request failed",
					"error": err.to_string(),
				}));
				if let Some(err) = err.downcast_ref::<crate::Error>() {
					match &err {
						crate::Error::DecodeError(_) => res.set_status(400),
						_ => res.set_status(500),
					}
				}
			}
			Ok(res)
		})
	}
}
