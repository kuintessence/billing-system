mod api;
mod infrastructure;
mod domain;
mod services;
mod server;

fn main() {
    server::run();
}
