pub mod server;
pub mod router;
pub mod handler;

use server::Server;
fn main() {
    let http_server = Server::new("127.0.0.1:7878");
    http_server.run();
}
