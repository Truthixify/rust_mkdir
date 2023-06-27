use colorful::Color;
use colorful::Colorful;

pub struct Config {
    pub dir_list: Vec<String>,
    pub flags: Vec<String>,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("Not enough argument");
        }

        let mut flags: Vec<String> = vec![];
        
        for arg in &args[1..] {
            if arg == "-v" || arg == "--verbose" {
                flags.push(String::from("-v"));
            }
            else if arg == "-h" || arg == "--help" {
                flags.push(String::from("-h"));
            }
            else if arg == "-p" || arg == "--parent" {
                flags.push(String::from("-p"));
            }
        }

        let mut dir_list: Vec<String>  = vec![];

        for arg in &args[1..] {
            if !flags.contains(&arg) {
                dir_list.push(arg.clone());
            }
        } 

        Ok(Config{ dir_list, flags })
    }
}

pub fn run(flags: Vec<String>, dir_list: Vec<String>) {
    if flags.contains(&String::from("-h")) {
        let help_txt = "NAME
        mkdir - make directories
 
 SYNOPSIS
        mkdir [OPTION]... DIRECTORY...
 
 DESCRIPTION
        Create the DIRECTORY(ies), if they do not already exist.
 
        Mandatory arguments to long options are mandatory for short options too.
 
        -p, --parents
               no error if existing, make parent directories as needed, with their file modes unaffected by any -m option.
 
        -v, --verbose
               print a message for each created directory
 
        --help display this help and exit
 
        --version
               output version information and exit
 
 AUTHOR
        Written by Truthixify.";
        println!("{}", help_txt.gradient(Color::Aquamarine1a));
        return
    }
    else if flags.contains(&String::from("-v")) {
        create(flags, dir_list.clone());
        for dir in dir_list {
            println!("{}", format!("✔️ mkdir: created directory(s) '{}'",dir).gradient(Color::Blue));
        }
    }
    else {
        create(flags, dir_list);
    }
}

pub fn create (flags: Vec<String>, dir_list: Vec<String>) {
    if flags.contains(&String::from("-p")) || flags.contains(&String::from("-p")) {
        for dir in dir_list.clone() {
            std::fs::create_dir_all(dir).unwrap_or_else(|err| {
                eprintln!("{}", format!("Error creating the directory(s): {err}").gradient(Color::Red));
                std::process::exit(1);
            });
        }
    }
    else {
        for dir in dir_list {
            std::fs::create_dir(dir).unwrap_or_else(|err| {
                eprintln!("{}", format!("Error creating the directory(s): {err}").gradient(Color::Red));
                std::process::exit(1);
            });
        }
    }
}
