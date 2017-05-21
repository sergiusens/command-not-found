extern crate clap;
use clap::{Arg, App};

extern crate glob;
use glob::glob;

extern crate gdbm;
use gdbm::{Gdbm};

use std::path::{Path, PathBuf};

fn find_package(db_file_path: PathBuf, command: &str) {
    let db = Gdbm::new(db_file_path.as_path(), 0, gdbm::READER, 0).unwrap();
    if let Ok(data) = db.fetch(command) {
      println!("The program '{}' is currently not installed. You can install it by typing:\nsudo apt install {}", command, data);
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
    let db_files_path_match = data_dir.join("programs.d").join("*.db");
    for db_path in glob(db_files_path_match.to_str().unwrap()).expect("Expected glob") {
      match db_path {
        Ok(path) => find_package(path, command),
        Err(e) => println!("{:?}", e),
      }
    }
}
