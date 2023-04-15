
use clap::{App, Arg};
use rand::seq::{SliceRandom, IteratorRandom};
use arboard::Clipboard;

fn main() {
    let application = App::new("Passify")
        .about("Creates a random password with confiruable arguments in the command line.")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("length")
                .short('l')
                .takes_value(true)
                .help("The length of the password to be generated.")
        )
        .arg(
            Arg::with_name("symbols")
                .short('s')
                .takes_value(false)
                .help("Whether or not to include symbols in the password.")
        )
        .get_matches();

    // Parse Arguments
    let password_size = application
        .value_of("length")
        .unwrap_or("10")
        .parse::<i16>()
        .unwrap_or(10);

    let symbols = application.is_present("symbols");

    // Generate password
    let password = generate_password(password_size, symbols);

    // Output password to the command line
    println!("{}", password);

    // Save password to the clipboard
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(password).unwrap();
}

fn generate_password(length: i16, symbols: bool) -> String {
    let mut password = String::new();
    let mut random = rand::thread_rng();

    let mut character_sets = vec!["abcdefghijklmnopqrstuvwxyz", "ABCDEFGHIJKLMNOPQRSTUVW", "01234567891"];

    if symbols {
        character_sets.push("!@#$%^&*()_+~`|}{[]:;?><,./-=");
    }

    for _ in 0..length {
        let character_set = character_sets.choose(&mut random);
        let character = character_set.unwrap().chars().choose(&mut random).unwrap();
        password.push(character);
    }

    password
}