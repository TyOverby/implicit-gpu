extern crate serde;
#[macro_use]
extern crate log;
extern crate void;
extern crate mime;
extern crate mime_guess;
extern crate regex;
extern crate combine;
extern crate serde_json;
extern crate hyper;
extern crate futures;

mod api;
mod routes;
mod parse_route;
mod static_file;

pub use api::Server;
pub use api::RequestInfo;

pub fn create() -> api::Server { api::Server::new() }

