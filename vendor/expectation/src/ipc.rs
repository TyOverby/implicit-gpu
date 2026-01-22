use expectation_shared::Result as EResult;
use serde_json;
use std::env;
use std::net::TcpStream;

fn get_stream() -> Option<TcpStream> {
    let env_var = match env::var("CARGO_EXPECT_IPC") {
        Ok(s) => s,
        Err(_) => {
            return None;
        }
    };

    let stream = match TcpStream::connect(env_var) {
        Ok(s) => s,
        Err(_) => {
            return None;
        }
    };

    Some(stream)
}

pub fn send(test_name: &str, results: &Vec<EResult>) {
    if let Some(mut s) = get_stream() {
        serde_json::to_writer_pretty(&mut s, &(test_name, results)).unwrap();
    }
}
