use futures;
use serde_json;
use hyper;
use serde::{Deserialize, Serialize};
use futures::{BoxFuture, Future};
use futures::Stream;
use std::error::Error;
use std::sync::Arc;
use hyper::server::{Http, Request, Response};

pub struct PathInfo;
pub struct QueryInfo;
pub struct FormInfo;

pub struct RequestInfo;

type BoxErr = Box<Error + Send + 'static>;
type Handler = Box<Fn((
        (hyper::Method, 
         hyper::Uri, 
         hyper::HttpVersion, 
         hyper::header::Headers)), Vec<u8>) -> 
            BoxFuture<Vec<u8>, BoxErr> + Send + Sync> ;

#[derive(Clone)]
pub struct Server {
    static_paths: Vec<String>,
    apis: Vec<(String, Arc<Handler>)>
}

impl Server {
    pub fn new() -> Server {
        Server {
            static_paths: vec![],
            apis: vec![],
        }
    }

    pub fn api<P, F, I, O, E>(mut self, path: P, f: F) -> Self
    where F: Fn(RequestInfo, I) -> Result<O, E> + 'static + Send + Sync,
          for <'de> I: Deserialize<'de> + 'static,
          O: Serialize + 'static,
          E: Error + Send + 'static,
          P: Into<String>,
    {
        use futures::Future;
        
        self.apis.push((path.into(), Arc::new(Box::new(move |_ri, in_body| {
            let mut bytes = &in_body[..];
            let value: I = match serde_json::from_reader(&mut bytes) {
                Ok(v) => v,
                Err(e) => return futures::future::err(Box::new(e) as BoxErr).boxed(),
            };

            let result = match f(RequestInfo, value) {
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
        }))));
        self
    }

    pub fn async_api<P, FT, F, I, O, E>(mut self, path: P, f: F) -> Self
    where F: Fn(RequestInfo, I) -> FT + 'static + Send + Sync,
          FT: Future<Item=O, Error=E> + Send + 'static,
          for <'de> I: Deserialize<'de> + 'static,
          O: Serialize + 'static,
          E: Error + Send + 'static,
          // TODO: make this AsRef<str> at some point
          P: Into<String>,
    {
        use futures::Future;
        
        self.apis.push((path.into(), Arc::new(Box::new(move |_ri, in_body| {
            let mut bytes = &in_body[..];
            let value: I = match serde_json::from_reader(&mut bytes) {
                Ok(v) => v,
                Err(e) => return futures::future::err(Box::new(e) as BoxErr).boxed(),
            };

            f(RequestInfo, value).and_then(|result| {
                let out = if cfg!(debug) {
                    serde_json::to_string_pretty(&result)
                } else {
                    serde_json::to_string(&result)
                };

                let out = out.unwrap();
                futures::future::ok(out.into_bytes())
            }).map_err(|e| Box::new(e) as BoxErr).boxed()
        }))));
        self
    }

    pub fn static_dir<S: Into<String>>(mut self, path: S) -> Self {
        self.static_paths.push(path.into());
        self
    }

    pub fn run(self) {
        let addr = "127.0.0.1:1337".parse().unwrap();
        let routes = self.apis.iter().cloned().map(|(s, h)| {
            let parsed = super::parse_route::parse(&s).unwrap();
            let (method, regex) = parsed.compile();
            (method, regex, h)
        });
        let routes = super::routes::RouteBuilder::new(routes);

        let service = RunningService {
            static_route: self.static_paths,
            routes: routes
        };

        let server = Http::new().bind(&addr, move || Ok(service.clone())).unwrap();

        println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
        server.run().unwrap();
    }
}

#[derive(Clone)]
struct RunningService {
    static_route: Vec<String>,
    routes: super::routes::RouteBuilder<Arc<Handler>>,
}

impl ::hyper::server::Service for RunningService {
    type Request = Request;
    type Response = Response;
    type Error = ::hyper::Error;
    type Future = BoxFuture<Response, ::hyper::Error>;


    fn call(&self, req: Request) -> Self::Future {
        use futures::future::{ok, FutureResult};
        use futures::Future;
        use hyper::StatusCode;
        use hyper::header::ContentType;
        use super::static_file::serve_statically;

        let (method, uri, http_version, headers, body) = req.deconstruct();

        let static_routes = self.static_route.iter().map(AsRef::as_ref);
        if let Some((mime, content)) = serve_statically(static_routes, uri.path()) {
            let response = Response::new()
                .with_header(ContentType(mime))
                .with_body(content);
            return ok(response).boxed()
        }

        if let Some((handler, _matches)) = self.routes.match_path(method.clone(), uri.clone().path()) {
            let handler = handler.clone();

            let body =  body.fold(Vec::new(), |mut v, b| -> FutureResult<Vec<u8>, ::hyper::Error> { 
                v.extend_from_slice(&*b); 
                ok(v)
            }).map_err(|e| {
                Box::new(e) as Box<Error + Send>
            });

            // Process the body
            let result = body.and_then(move |body_vec| {
                (handler)((method, uri, http_version, headers), body_vec)
            });

            // Return with the result body
            let result = result.and_then(|res_body| {
                ok(Response::new().with_body(res_body))
            });

            // Convert application errors to HTTP Errors
            let result = result.or_else(|error| {
                eprintln!("{}", error);
                // TODO: custom error body
                ok(Response::new().with_status(StatusCode::InternalServerError).with_body("oh no."))
            });

            result.boxed()

        } else {
            // FIXME 
            println!("no handler found: {}", uri.path());
            let res = Response::new()
                .with_body("404 not found")
                .with_status(StatusCode::NotFound);
            ok(res).boxed()
        }
    }
}
