extern crate clap;
use clap::{Arg, App};

extern crate glob;
use glob::glob;

extern crate gdbm;
use gdbm::{Gdbm};

use std::process::exit;
use std::path::{Path};


fn find_package(db_dir: &Path, command: &str) -> Result<Vec<String>, String> {
    let db_files_path_match = db_dir.join("programs.d").join("*.db");
    let mut packages: Vec<String> = Vec::new();

    for db_path in glob(db_files_path_match.to_str().unwrap()).expect("Expected glob") {
      match db_path {
        Ok(path) => {
          if let Ok(query) = query_database(path.as_path(), command) {
            packages.push(query); 
          }
        }
        Err(e) => println!("{:?}", e),
      }
    }
    if packages.is_empty() {
      return Err("The were no packages found.".to_string())
    }
    return Ok(packages);
}

fn query_database(db_file_path: &Path, command: &str) -> Result<String, bool> {
    let db = Gdbm::new(db_file_path, 0, gdbm::READER, 0).unwrap();
    match db.fetch(command) {
      Ok(data) => return Ok(data),
      Err(_) => return Err(false), // handle error enum
    }
}

fn print_packages(command: &str, packages_found: Vec<String>) {
  for package_name in &packages_found {
    println!("The program '{}' is currently not installed. You can install it by typing:\nsudo apt install {}", command, package_name);
  }
}

fn main() {
    let matches = App::new("command-not-found")
                  .version("0.3")
                  .author("Sergio Schvezov <sergio.schvezov@canonical.com>")
                  .about("Searches for commands")
                  .arg(Arg::with_name("data_dir")
                    .long("data-dir")
                    .value_name("data-dir")
                    .help("Use an alternate data directory for data fields")
                    .required(false))
                  .arg(Arg::with_name("ignore-installed")
                    .long("ignore-installed")
                    .help("ignore local binaries and display the available packages"))
                  .arg(Arg::with_name("command")
                               .help("Command to search package for")
                               .required(true)
                               .index(1))
                  .get_matches();

    let command = matches.value_of("command").unwrap();
    let data_dir = Path::new(matches.value_of("data_dir").unwrap_or("/usr/share/command-not-found"));
    
    let packages = find_package(data_dir, command);
    match packages {
      Ok(packages_found) => print_packages(command, packages_found),
      Err(e) => {
        println!("{:?}", e);
        exit(1);
      }
    }
}
