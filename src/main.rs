mod http;

use http::server::HttpServer;

fn main() {
    let server = HttpServer::new("127.0.0.1:8080");
    if let Err(e) = server.run() {
        eprintln!("{}", e);
    }
}
