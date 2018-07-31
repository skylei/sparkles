#![feature(
    async_await,
    await_macro,
    futures_api,
    pin,
    use_extern_macros
)]
#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use simple_server::{Method, StatusCode};
use sparkles_macros::{not_found, route, serve, server};

#[server]
struct Server;

impl Server {
    #[route(GET, "/hello")]
    fn hello(&self, mut response: simple_server::Builder) -> simple_server::ResponseResult {
        Ok(response
            .body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())
            .unwrap())
    }

    #[route(GET, "/lol")]
    fn bar(&self, mut response: simple_server::Builder) -> simple_server::ResponseResult {
        Ok(response
            .body("<h1>Hi!</h1><p>Hellololol</p>".as_bytes().to_vec())
            .unwrap())
    }

    #[not_found]
    fn four_oh_four(&self, mut response: simple_server::Builder) -> simple_server::ResponseResult {
        response.status(StatusCode::NOT_FOUND);
        Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
    }
}

#[serve]
fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server;
}