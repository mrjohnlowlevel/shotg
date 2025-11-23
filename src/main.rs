use std::env::args;
use std::process::exit;
use shotg::util::{ match_command, new_project, Commands };
use std::io::Result as IOResult;

fn main() -> IOResult<()> {
    let argv: Vec<String> = args().collect();
    let argc: usize = argv.len();

    if argc < 2 {
        println!("Usage: {} <COMMAND>", argv[0]);
        exit(1);
    }

    let command_opt: Option<Commands> = match_command(argv[1].clone());

    match command_opt {
        Some(cmd) => match cmd {

            Commands::New => {
                if argc < 3 {
                    println!("Project Name Required: {} new <NAME>", argv[0]);
                    exit(1);
                }

                new_project(argv[2].clone())?;
            },

            Commands::Run => println!("this is run"),
            Commands::Init => println!("this is init"),
        },
        None => println!("none here"),
    }

    Ok(())
}
