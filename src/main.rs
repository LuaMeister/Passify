
use std::any::Any;

use clap::{App, Arg};
use rand::{seq::{SliceRandom, IteratorRandom}};

fn main() {
    let _application = App::new("Passify")
        .about("Creates a random password with confiruable parameters in the command line.")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("length")
                .short('l')
                .takes_value(true)
                .help("The length of the password to be generated.")
        )
        .get_matches();

    let password_size = _application
        .value_of("length")
        .unwrap_or("10")
        .parse::<i16>()
        .unwrap_or(10);

    let password = generate_password(password_size);    
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