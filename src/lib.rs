use std::fs;
use std::fs::Metadata;
use std::io::ErrorKind;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;



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
            print_long_listing(&name, &metadata);
        } else {
            print!("{} ", name);
        }
    }

    println!("");
    Ok(())
}

fn list_file(file: &String, long_listing: bool) -> std::io::Result<()> {
    if long_listing {
        let metadata = fs::metadata(file)?;
        print_long_listing(file, &metadata);
    } else {
        println!("{}", file);
    }
    Ok(())
}

fn print_long_listing(name: &String, metadata: &Metadata) {
    let permissions = metadata.permissions().mode();
    let len = metadata.len();
    let uid = metadata.uid();
    let gid = metadata.gid();

    println!("{:<name_width$} {} {} {:o} {:>len_width$}B",
        name, uid, gid, permissions, len, name_width=20, len_width=9
    );
}
