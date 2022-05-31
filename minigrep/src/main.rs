use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|e| {
        println!("Application error: {}", e);
        process::exit(1);
    });

    if let Err(e) = run(cfg) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
