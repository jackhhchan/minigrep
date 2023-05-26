use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// note:
//  1. refactor args to Config struct
//  2. impl a ::build function on Config that returns a Result struct.
//  3. unwrap Result struct and handle Error -> use std::process to call exit(1); - unwrap_or_else
//  4. println! why its failed before exiting.
//  5. create a run(config: Config) function that returns Result<(), Box<dyn Error>> -> returns Ok(())
//  6. use if let to handle the error case in the main function.
//  7. exit(1) if err.
//  8. move Config and run(config: Config) fn into lib.rs
//  9. we can use minigrep as the module name and call run in main.rs.

