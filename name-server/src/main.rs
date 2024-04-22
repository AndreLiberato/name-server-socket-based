mod middleware;

use middleware::name_server::{self, Server};
fn main() {
    let middleware: Server = name_server::Server::new("127.0.0.1", "8080");
    middleware.start();
}
