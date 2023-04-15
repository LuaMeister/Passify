
use clap::App;
use rand::{seq::{SliceRandom, IteratorRandom}};

fn main() {
    let _application = App::new("Passify")
        .about("Creates a random password with confiruable parameters in the command line.")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .get_matches();

    let password = generate_password(10);
    println!("{}", password);
}

fn generate_password(length: i16) -> String {
    let mut password = String::new();
    let mut random = rand::thread_rng();

    let character_sets = vec!["abcdefghijklmnopqrstuvwxyz", "ABCDEFGHIJKLMNOPQRSTUVW"];

    for _ in 0..length {
        let character_set = character_sets.choose(&mut random);
        let character = character_set.unwrap().chars().choose(&mut random).unwrap();
        password.push(character);
    }

    password
}