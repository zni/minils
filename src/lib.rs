use std::fs;
use std::io::ErrorKind;

pub struct Config {
    paths: Vec<String>,
    long_listing: bool,
}

impl Config {
    pub fn new(args: &mut Vec<String>) -> Result<Config, &'static str> {
        args.remove(0);

        let mut paths = Vec::new();
        let mut long_listing = false;
        let opts = args.iter();
        for opt in opts {
            match opt.as_str() {
                "-l" => { long_listing = true; }
                _ => paths.push(opt.clone()),
            }
        }

        if paths.len() == 0 {
            paths.push(String::from("."));
        }

        Ok(Config { paths, long_listing })
    }
}

pub fn run(config: &Config) -> std::io::Result<()> {
    list_files(&config)
}

fn list_files(config: &Config) -> std::io::Result<()> {
    for file in &config.paths {
         match is_directory(&file) {
            Ok(true) => list_directory(&file, config.long_listing)?,
            Ok(false) => list_file(&file, config.long_listing)?,
            Err(e) => handle_error(&file, e),
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

fn is_directory(file: &String) -> std::io::Result<bool> {
    let attrs = fs::metadata(file)?;
    Ok(attrs.is_dir())
}

fn list_directory(file: &String, long_listing: bool) -> std::io::Result<()> {
    println!("{}:", file);

    let dir_entries = fs::read_dir(file)?;
    for entry in dir_entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let name = entry.file_name().into_string().unwrap();

        if long_listing {
            print_long_listing(&name, metadata.len());
        } else {
            print!("{} ", name);
        }
    }

    println!("");
    Ok(())
}

fn list_file(file: &String, long_listing: bool) -> std::io::Result<()> {
    if long_listing {
        let attrs = fs::metadata(file)?;
        print_long_listing(file, attrs.len());
    } else {
        println!("{}", file);
    }
    Ok(())
}

fn print_long_listing(name: &String, len: u64) {
    println!("{:<name_width$} {:>len_width$}B",
        name, len, name_width=20, len_width=9
    );
}
