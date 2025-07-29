use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Create a Config instance from command line arguments
    // If the Config instance is created successfully, it runs the minigrep application.
    // If the arguments are insufficient, it prints an error message and exits.
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Run the minigrep application with the provided configuration
    // If an error occurs during execution, it prints the error message and exits with a 1 status code.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
