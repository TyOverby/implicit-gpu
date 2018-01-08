extern crate combine;
extern crate flame;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate mime;
extern crate mime_guess;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate void;

mod api;
mod routes;
mod parse_route;
mod static_file;

pub use hyper::{Request, Response};

pub use api::Server;
pub use api::RequestInfo;

pub fn create() -> api::Server {
    api::Server::new()
}
