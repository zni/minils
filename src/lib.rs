use std::fs;
use std::fs::Metadata;
use std::io::ErrorKind;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use users;



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

        if paths.is_empty() {
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

fn handle_error(file: &str, error: std::io::Error) {
    if let ErrorKind::NotFound = error.kind() {
        println!("{}: file not found", file);
    };
}

fn is_directory(file: &str) -> std::io::Result<bool> {
    let attrs = fs::metadata(file)?;
    Ok(attrs.is_dir())
}

fn list_directory(file: &str, long_listing: bool) -> std::io::Result<()> {
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

    println!();
    Ok(())
}

fn list_file(file: &str, long_listing: bool) -> std::io::Result<()> {
    if long_listing {
        let metadata = fs::metadata(file)?;
        print_long_listing(file, &metadata);
    } else {
        println!("{}", file);
    }
    Ok(())
}

fn print_long_listing(name: &str, metadata: &Metadata) {
    let permissions = metadata.permissions().mode();
    let permissions = format_permissions(permissions);
    let len = metadata.len();

    let uid = metadata.uid();
    let user = users::get_user_by_uid(uid).unwrap();
    let user = user.name().to_string_lossy();

    let gid = metadata.gid();
    let group = users::get_group_by_gid(gid).unwrap();
    let group = group.name().to_string_lossy();

    println!("{} {} {}  {:>len_width$}B {:<name_width$}",
        permissions, user, group, len, name, name_width=20, len_width=9
    );
}

fn format_permissions(permissions: u32) -> String {
    // directory
    let directory = match (permissions >> 14) & 1 {
        1 => "d",
        _ => "-",
    };

    // owner
    let owner_read = match (permissions >> 8) & 1 {
        1 => "r",
        _ => "-",
    };
    let owner_write = match (permissions >> 7) & 1 {
        1 => "w",
        _ => "-",
    };
    let owner_execute = match (permissions >> 6) & 1 {
        1 => "x",
        _ => "-",
    };

    // group
    let group_read = match (permissions >> 5) & 1 {
        1 => "r",
        _ => "-",
    };
    let group_write = match (permissions >> 4) & 1 {
        1 => "w",
        _ => "-",
    };
    let group_execute = match (permissions >> 3) & 1 {
        1 => "x",
        _ => "-",
    };

    // other
    let other_read = match (permissions >> 2) & 1 {
        1 => "r",
        _ => "-",
    };
    let other_write = match (permissions >> 1) & 1 {
        1 => "w",
        _ => "-",
    };
    let other_execute = match permissions & 1 {
        1 => "x",
        _ => "-",
    };

    format!("{}{}{}{}{}{}{}{}{}{}",
        directory,
        owner_read, owner_write, owner_execute,
        group_read, group_write, group_execute,
        other_read, other_write, other_execute
    )
}

