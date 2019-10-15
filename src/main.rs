mod id;
mod init;
mod repository;
mod status;

use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use env_logger;
use log::info;
use repository::Repository;
use status::run_status;
use std::error::Error;

/// Application error for dynamic dispatch
type ApplicationError = Box<dyn Error + Send + Sync>;

fn main() {
    env_logger::init();
    info!("Starting the app");
    let matches = App::new("gel")
        .about("Distributed Version Control System")
        .author("Nazım Can Altınova")
        .version(&*format!("v{}", crate_version!()))
        .subcommand(
            SubCommand::with_name("init")
                .about(
                    "", // TODO
                )
                .arg(
                    Arg::with_name("directory")
                        .help("") // TODO:
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("status").about(
            "", // TODO
        ))
        .subcommand(
            SubCommand::with_name("add")
                .about(
                    "", // TODO
                )
                .arg(
                    Arg::with_name("file")
                        .short("l")
                        .long("file")
                        .help("")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("commit").about(""), // TODO
        )
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Runs the given sub command.
fn run(matches: ArgMatches) -> Result<(), ApplicationError> {
    info!("Running the app");
    match matches.subcommand() {
        ("init", Some(m)) => Repository::init(m)?,
        ("status", Some(m)) => run_status(m)?,
        ("add", Some(_m)) => unimplemented!(),
        ("commit", Some(_m)) => unimplemented!(),
        _ => (),
    };
    Ok(())
}
