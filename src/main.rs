mod commands;
mod hash;
mod objects;
mod repository;
mod user;

use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use env_logger;
use log::info;
use repository::Repository;
use std::error::Error;

/// Application error for dynamic dispatch
type ApplicationError = Box<dyn Error + Send + Sync>;

fn main() {
    env_logger::init();
    info!("Starting the app");

    let matches = App::new("gel")
        .about(
            "Distributed version control system that is completely different and better than git.",
        )
        .author("Nazım Can Altınova")
        .version(&*format!("v{}", crate_version!()))
        .subcommand(
            SubCommand::with_name("init")
                .about("Initializes an empty gel repository.")
                .arg(
                    Arg::with_name("directory")
                        .help("Directory name to create and initialize gel.")
                        .takes_value(true)
                        .required(true),
                ),
        )
        // `hash-object` and `cat-file` is mainly for debug purpose.
        // Git also has them but no one knows/needs.
        .subcommand(
            SubCommand::with_name("hash-object")
                .about("Compute object hash and optionally creates a blob from a file")
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .help("") // todo
                        .takes_value(true)
                        .default_value("blob"),
                )
                .arg(
                    Arg::with_name("write").short("w").long("write").help(""), // todo,
                )
                .arg(
                    Arg::with_name("file")
                        .help("")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("cat-file")
                .about("Provide content of repository objects")
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .help("Type of the object")
                        .takes_value(true)
                        .default_value("blob"),
                )
                .arg(
                    Arg::with_name("object")
                        .help("")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("Displays commit history of a repository.")
                .arg(
                    Arg::with_name("commit")
                        .help("Commit to start at.")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("status").about("Displays status of the repository."))
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds file contents to the index.")
                .arg(
                    Arg::with_name("file")
                        .help("Files to add to the index.")
                        .takes_value(true),
                ),
        )
        // .subcommand(
        //     SubCommand::with_name("commit").about(""), // TODO
        // )
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Runs the given sub command.
fn run(matches: ArgMatches) -> Result<(), ApplicationError> {
    info!("Running the app");

    // NOTE: `clone` will also be an exception here
    if let Some("init") = matches.subcommand_name() {
        Repository::init(&matches.subcommand_matches("init").unwrap())?;
        return Ok(());
    }

    let repository = Repository::new()?;
    match matches.subcommand() {
        ("init", _) => panic!("Internal error: We've already executed init command!"),
        // `hash-object` and `cat-file` is mainly for debug purpose.
        // Git also has them but no one knows/needs.
        ("hash-object", Some(m)) => repository.hash_object(m)?,
        ("cat-file", Some(m)) => repository.cat_file(m)?,
        ("log", Some(m)) => repository.log(m)?,
        ("status", Some(m)) => repository.status(m)?,
        ("add", Some(_m)) => unimplemented!(),
        ("commit", Some(_m)) => unimplemented!(),
        _ => unimplemented!(),
    };
    Ok(())
}
