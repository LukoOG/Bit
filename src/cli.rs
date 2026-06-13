use crate::commands::Command;

pub fn parse_args(args: &[String]) -> Command {
    let length = args.len();
    if length < 2 {
        println!("No arguements provided!");
        std::process::exit(1)
    };

    let arg = args[1].as_str();

    match arg {
        "init" => Command::Init,
        "history" => Command::History,
        "save" => {
            if let Some(message) = args.get(2) {
                return Command::Save { message: message.clone() }
            } else {
                eprintln!("Message not provided!");
                std::process::exit(1)
            }

        }
        _ => Command::Init,
    }
}
