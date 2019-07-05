use std::fs;
use std::io::ErrorKind;

pub struct Config {
    paths: Vec<String>
}

impl Config {
    pub fn new(args: &mut Vec<String>) -> Result<Config, &'static str> {
        args.remove(0);

        if args.len() == 0 {
            args.push(".".to_string());
        }

        Ok(Config { paths: args.clone() })
    }
}

pub fn run(config: &Config) {
    list_files(&config.paths);
}

fn list_files(files: &Vec<String>) -> Result<(), std::io::Error> {
    for file in files {
         match is_directory(&file) {
            Ok(true) => list_directory(file)?,
            Ok(false) => list_file(file)?,
            Err(e) => handle_error(file, e),
        }
    }
    Ok(())
}

fn handle_error(file: &String, error: std::io::Error) {
    match error.kind() {
        ErrorKind::NotFound => println!("{}: file not found", file),
        _ => (),
    };
}

fn is_directory(file: &String) -> Result<bool, std::io::Error> {
    let attrs = fs::metadata(file)?;
    Ok(attrs.is_dir())
}

fn list_directory(file: &String) -> Result<(), std::io::Error> {
    println!("{}:", file);
    let dir_entries = fs::read_dir(file)?;
    for entry in dir_entries {
        let entry = entry?;
        println!("\t{:?}", entry.file_name());
    }
    Ok(())
}

fn list_file(file: &String) -> Result<(), std::io::Error> {
    println!("{}", file);
    Ok(())
}
