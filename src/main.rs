mod command;
mod error;
mod store;

use command::Command;
use crate::error::AppError;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
    }
}

fn run() -> Result<(), AppError> {
    let cmd = Command::parse();

    let mut store = store::Store::load("data.json")?;

    match cmd {
        Ok(command) => match command {
            Command::Set {key, value} => {
                store.set(key, value);
                store.save("data.json")?;
                println!("set success");
                Ok(())
            }
            Command::Get { key} => match store.get(&key) {
                Ok(value) => {println!("Value: {}", value); Ok(())}
                Err(store::StoreError::KeyNotFound) => {println!("Key not found"); Ok(())}
                Err(e) => Err(e.into()),
            }
            Command::Delete { key} => match store.delete(&key) {
                Ok(_) => {
                    store.save("data.json")?;
                    println!("delete success");
                    Ok(())
                }
                Err(store::StoreError::KeyNotFound) => {println!("Key not found"); Ok(())}
                Err(e) => Err(e.into()),
            }
            Command::Help => {Command::print_help(); Ok(())}
        }
        Err(error) => Err(error.into()),
    }
}