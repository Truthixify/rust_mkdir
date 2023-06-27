use colorful::Color;
use colorful::Colorful;

fn main() {
    let args : Vec<String> = std::env::args().collect();

    let config = mkdir::Config::build(args.clone()).unwrap_or_else(|err| {
        eprintln!("{}", format!("⚠️ Problem parsing arguments: {}", err).gradient(Color::Red));
        eprintln!("{}", format!("ℹ Try 'mkdir --help' for more information.").gradient(Color::Green));
        std::process::exit(1);
    });

    mkdir::run(config.flags, config.dir_list);
}