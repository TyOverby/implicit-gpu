use futures;
use serde_json;
use serde::{Deserialize, Serialize};
use futures::{BoxFuture, Future};
use std::error::Error;

pub struct PathInfo;
pub struct QueryInfo;
pub struct FormInfo;

pub struct RequestInfo<P=PathInfo, Q=QueryInfo, R=FormInfo> {
    pub path_info: P,
    pub query_info: Q,
    pub form_info: R,
}

type BoxErr = Box<Error + Send + 'static>;
type Handler = Box<Fn(RequestInfo, Vec<u8>) -> BoxFuture<Vec<u8>, BoxErr>>;

pub struct ApiBuilder {
    apis: Vec<(String, Handler)>
}

impl ApiBuilder {
    pub fn new() -> ApiBuilder {
        ApiBuilder {
            apis: Vec::new()
        }
    }

    pub fn add_api<P, F, I, O, E>(&mut self, path: P, f: F) -> &mut Self
    where F: Fn(RequestInfo, I) -> Result<O, E> + 'static,
          for <'de> I: Deserialize<'de> + 'static,
          O: Serialize + 'static,
          E: Error + Send + 'static,
          P: Into<String>,
    {
        use futures::Future;
        
        self.apis.push((path.into(), Box::new(move |ri, in_body| {
            let mut bytes = &in_body[..];
            let value: I = match serde_json::from_reader(&mut bytes) {
                Ok(v) => v,
                Err(e) => return futures::future::err(Box::new(e) as BoxErr).boxed(),
            };

            let result = match f(ri, value) {
                Ok(r) => r,
                Err(e) => return futures::future::err(Box::new(e) as BoxErr).boxed(),
            };

            let out = if cfg!(debug) {
                serde_json::to_string_pretty(&result)
            } else {
                serde_json::to_string(&result)
            };

            let out = out.unwrap();
            futures::future::ok(out.into_bytes()).boxed()
        })));
        self
    }

    pub fn add_async_api<P, FT, F, I, O, E>(&mut self, path: P, f: F) -> &mut Self
    where F: Fn(RequestInfo, I) -> FT + 'static,
          FT: Future<Item=O, Error=E> + Send + 'static,
          for <'de> I: Deserialize<'de> + 'static,
          O: Serialize + 'static,
          E: Error + Send + 'static,
          // TODO: make this AsRef<str> at some point
          P: Into<String>,
    {
        use futures::Future;
        
        self.apis.push((path.into(), Box::new(move |ri, in_body| {
            let mut bytes = &in_body[..];
            let value: I = match serde_json::from_reader(&mut bytes) {
                Ok(v) => v,
                Err(e) => return futures::future::err(Box::new(e) as BoxErr).boxed(),
            };

            f(ri, value).and_then(|result| {
                let out = if cfg!(debug) {
                    serde_json::to_string_pretty(&result)
                } else {
                    serde_json::to_string(&result)
                };

                let out = out.unwrap();
                futures::future::ok(out.into_bytes())
            }).map_err(|e| Box::new(e) as BoxErr).boxed()
        })));
        self
    }
}
