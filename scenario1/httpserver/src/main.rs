mod handler;
mod router;
mod server;
use server::Server;

fn main() {
    let mut server = Server::new("localhost:3000");
    server.run();
}
