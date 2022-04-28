
use clap::Parser;

fn main() {
    // Parse the CLI arguments
    let args = mass_scanner::Config::parse();

    // Run the mass scanner
    mass_scanner::run(args);
}
