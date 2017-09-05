extern crate clap;
extern crate glob;


use clap::{App, Arg};
use glob::glob;
use std::convert::From;
use std::fs;


fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("start_dir")
                .help(
                    "path to the top level directory containing the files to rename",
                )
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("from")
                .help("Change extension from")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("to")
                .help("Change extension to")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let start_dir = matches.value_of("start_dir").unwrap();
    let from = matches.value_of("from").unwrap();
    let to = matches.value_of("to").unwrap();

    let search = String::from(start_dir) + "/**/*." + from;
    let paths = glob(&search).expect("Could not find path given.").map(
        |p| {
            p.expect("Bad path name")
        },
    );

    for path in paths {
        match fs::rename(&path, &path.with_extension(to)) {
            Ok(_) => {
                println!(
                    "Renamed :  {:?} to {:?}",
                    path.clone(),
                    &path.with_extension(to)
                )
            }
            Err(msg) => println!("{}", msg),
        };
    }
}
