use std::env;

mod cli;
mod commands;

use cli::parse_args;
use commands::{
    Command,
    init::handle_init,
    history::handle_history,
    save::handle_save
};
fn main() {
    let args: Vec<String> = env::args().collect();

    let command = parse_args(&args);

    let results = match command {
        Command::Init => handle_init(),
        // Command::Save { message } => handle_save(message),
        // Command::History => handle_history(),
        _ => Ok(())
    };
}
