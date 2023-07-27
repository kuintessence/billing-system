use infrastructure::Host;

mod controllers;
mod infrastructure;
mod kernel;

fn main() {
    Host::new().run();
}
