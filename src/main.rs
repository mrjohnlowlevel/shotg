use shotg::util::{Commands, init_project, match_command, new_project, run_project};
use std::env::args;
use std::io;
use std::process::exit;

fn main() -> io::Result<()> {
    let argv: Vec<String> = args().collect();
    let argc: usize = argv.len();

    if argc < 2 {
        println!("Usage: {} <COMMAND>", argv[0]);
        exit(1);
    }

    let command_opt: Option<Commands> = match_command(argv[1].as_str());

    match command_opt {
        Some(cmd) => match cmd {
            Commands::New => {
                if argc < 3 {
                    println!("Project Name Required: {} new <NAME>", argv[0]);
                    exit(1);
                }

                new_project(argv[2].as_str())?;
                println!("Created project \"{}\"", argv[2].clone());
            }

            Commands::Run => {
                run_project()?;
            }

            Commands::Init => {
                init_project()?;
                println!("Initialized project in current directory");
            }
        },

        None => {
            println!("Command `{}` doesn't exist", argv[1].as_str());
            exit(1);
        }
    }

    Ok(())
}
