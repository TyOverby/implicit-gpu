extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate futures;
extern crate hyper_router;

use serde::{Deserialize, Serialize};
use futures::{BoxFuture, Future};
use std::error::Error;

pub struct PathInfo;
pub struct QueryInfo;

pub struct Builder {
    apis: Vec<(String, Box<Fn(PathInfo, QueryInfo, String) -> BoxFuture<Vec<u8>, Box<Error + Send>> + Send>)>
}

pub trait ApiFunction<I, R>
{
    fn build(self) -> Box<Fn(PathInfo, QueryInfo, I) -> R + Send>;
}

impl <I, R> ApiFunction<I, R> for fn(PathInfo, QueryInfo, I) -> R 
where I: 'static, R: 'static,
{
    fn build(self) -> Box<Fn(PathInfo, QueryInfo, I) -> R + Send> {
        Box::new(move |a, b, c,| self(a, b, c))
    }
}

impl <I, R> ApiFunction<I, R> for fn(PathInfo, I) -> R 
where I: 'static, R: 'static,
{
    fn build(self) -> Box<Fn(PathInfo, QueryInfo, I) -> R + Send> {
        Box::new(move |a, _, c,| self(a, c))
    }
}

impl <I, R> ApiFunction<I, R> for fn(I) -> R 
where I: 'static, R: 'static,
{
    fn build(self) -> Box<Fn(PathInfo, QueryInfo, I) -> R + Send> {
        Box::new(move |_, _, c,| self(c))
    }
}

impl Builder {
    pub fn add_api<S, I, R, F>(&mut self, path: S, f: F) -> &mut Self
    where F: ApiFunction<I, R> + Send + 'static,
          for <'de> I: Deserialize<'de> + 'static,
          R: Serialize + 'static,
          S: Into<String>,
    {
        use futures::Future;
        
        let built = f.build();
        self.apis.push((path.into(), Box::new(move |pi, qi, in_body| {
            let mut bytes = in_body.as_bytes();
            let value: I = match serde_json::from_reader(&mut bytes) {
                Ok(v) => v,
                Err(e) => return futures::future::err(Box::new(e) as Box<Error + Send>).boxed(),
            };

            let result = (built)(pi, qi, value);

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

    pub fn add_async_api<S, I, R, F, FT, ER>(&mut self, path: S, f: F) -> &mut Self
    where FT: Future<Item=R, Error=ER> + Send + 'static,
          ER: Into<Box<Error + Send>> + Send + 'static,
          F: ApiFunction<I, FT> + Send + 'static,
          for <'de> I: Deserialize<'de> + 'static,
          R: Serialize + 'static,
          S: Into<String>,
    {
        use futures::Future;
        
        let built = f.build();
        self.apis.push((path.into(), Box::new(move |pi, qi, in_body| {
            let mut bytes = in_body.as_bytes();
            let value: I = match serde_json::from_reader(&mut bytes) {
                Ok(v) => v,
                Err(e) => return futures::future::err(Box::new(e) as Box<Error + Send>).boxed(),
            };

            let result = (built)(pi, qi, value);
            result.and_then(move |result| {
                let out = if cfg!(debug) {
                    serde_json::to_string_pretty(&result)
                } else {
                    serde_json::to_string(&result)
                };

                let out = out.unwrap();
                futures::future::ok(out.into_bytes())
            }).map_err(|e| e.into()).boxed()
        })));
        self
    }
}
