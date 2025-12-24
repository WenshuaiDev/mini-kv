#[derive(Debug)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Help,
}

#[derive(Debug)]
pub enum CommandError {
    InvalidCommand,
    MissingArgument,
}

use std::env;

impl Command {
    pub fn parse() -> Result<Command, CommandError> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err(CommandError::InvalidCommand);
        }

        match args[1].as_str() {
            "set" => {
                if args.len() != 4 {
                    return Err(CommandError::MissingArgument);
                }
                Ok(Command::Set {
                    key: args[2].clone(),
                    value: args[3].clone(),
                })
            }
            "get" => {
                if args.len() != 3 {
                    return Err(CommandError::MissingArgument);
                }
                Ok(Command::Get {
                    key: args[2].clone(),
                })
            }
            "delete" => {
                if args.len() != 3 {
                    return Err(CommandError::MissingArgument);
                }
                Ok(Command::Delete {
                    key: args[2].clone(),
                })
            }
            "-h" | "--help" => Ok(Command::Help),
            _ => Err(CommandError::InvalidCommand)
        }
    }

    pub fn print_help() {
        println!(
            r#"mini-kv: a simple key-value store

Usage:
    mini-kv set <key> <value>    # set key
    mini-kv get <key>            # get key
    mini-kv del <key>            # delete key
    mini-kv -h | --help          # show this help
"#
        );
    }
}