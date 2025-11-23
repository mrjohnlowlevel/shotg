use std::env::args;
use std::io;
use std::process::exit;

enum Commands {
    New,
    Init,
    Run,
}

fn match_command(cmd: String) -> Option<Commands> {
    if cmd == "new".to_string() {
        Some(Commands::New)
    }
    else if cmd == "init".to_string() {
        Some(Commands::Init)
    }
    else if cmd == "run".to_string() {
        Some(Commands::Run)
    }
    else {
        None
    }
}

fn new_project() -> Result<(), String> {
    Ok(())
}

fn main() -> io::Result<()> {
    let argv: Vec<String> = args().collect();
    let argc: usize = argv.len();

    if argc < 2 {
        println!("Usage: {} <COMMAND>", argv[0]);
        exit(1);
    }

    let command_opt: Option<Commands> = match_command(argv[1].clone());

    match command_opt {
        Some(cmd) => match cmd {
            Commands::New => println!("this is new"),
            Commands::Run => println!("this is run"),
            Commands::Init => println!("this is init"),
        },
        None => println!("none here"),
    }

    Ok(())
}
