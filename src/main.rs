extern crate clap;
use clap::{Arg, App};

extern crate gdbm;
use gdbm::{Gdbm};

use std::path::Path;

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

    let data_dir = Path::new(matches.value_of("data_dir").unwrap_or("/usr/share/command-not-found"));
    let db_file_path = data_dir.join("programs.d").join("all-main.db");
    let db = Gdbm::new(db_file_path.as_path(), 1024, gdbm::READER, 0).unwrap();
    let data = db.fetch(matches.value_of("command").unwrap()).unwrap();
  
    println!("Please install: {}", data);
}
