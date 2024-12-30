use clap::{App, Arg, SubCommand};

mod functions;

use crate::functions::{create_schema, alter_schema, select_schema};


fn main() {

    let matches = App::new("NorthDB")
        .version("0.2.1")
        .author("Adam Seelye")
        .about("NorthDB")
        .subcommand(
            SubCommand::with_name("alter_schema")
                .long_flag("alter")
                .short_flag('a')
                .about("Alter a schema")
                .arg(
                    Arg::with_name("schema_name")
                    .help("Name of schema to alter")
                    .required(true),
                )
                .arg(
                    Arg::with_name("data")
                        .short('d')
                        .long("data")
                        .help("Input data")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("create_schema")
                .long_flag("create")
                .short_flag('c')
                .about("Create a new schema")
                .arg(
                    Arg::with_name("schema_name")
                        .help("Name of schema to create")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("select_schema")
                .long_flag("select")
                .short_flag('s')
                .about("Select and read from a schema")
                .arg(
                    Arg::with_name("schema_name")
                        .help("Name of schema to select")
                        .required(true),
                ),
        )
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("alter_schema") {
        if let Some(schema_name) = matches.value_of("schema_name") {
            if let Some(data) = matches.value_of("data") {
                if let Err(e) = alter_schema(schema_name, data) {
                    eprintln!("Error: {}", e);
                }
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("create_schema") {
        if let Some(schema_name) = matches.value_of("schema_name") {
            if let Err(e) = create_schema(schema_name) {
                eprintln!("Error: {}", e);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("select_schema") {
        if let Some(schema_name) = matches.value_of("schema_name") {
            if let Err(e) = select_schema(schema_name) {
                eprintln!("Error: {}", e);
            }
        }
    } else {
        println!("Please specify a command. Use --help for more help.");
    }
}

