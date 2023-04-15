
use clap::App;

fn main() {
    let _application = App::new("Passify")
        .about("Creates a random password with confiruable parameters in the command line.")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"));
}