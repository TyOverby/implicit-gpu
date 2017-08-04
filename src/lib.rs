extern crate serde;
extern crate void;
extern crate mime;
extern crate mime_guess;
extern crate regex;
extern crate combine;
extern crate serde_json;
extern crate hyper;
extern crate futures;
extern crate hyper_router;

pub mod api;
pub mod routes;
pub mod parse_route;
pub mod static_file;

pub fn create() -> api::Server { api::Server::new() }
