use std::fs::{File, create_dir};
use std::io::{self, Write};
use std::process::Command;

const MAIN_C_FILE_CONTENT: &'static str =
    "#include <stdio.h>\n\nint main()\n{\n\tprintf(\"Hello, World!\");\n}";
// const MAKEFILE_CONTENT: &'static str = "SRC = main.c\nCC = gcc\nBIN = main\n\nall:\n\t$(CC) $(SRC) -o bin/$(BIN)";

pub enum Commands {
    New,
    Init,
    Run,
}

pub fn match_command(cmd: &str) -> Option<Commands> {
    match cmd {
        "new" => Some(Commands::New),
        "run" => Some(Commands::Run),
        "init" => Some(Commands::Init),
        _ => None,
    }
}

pub fn new_project(project_name: String) -> io::Result<()> {
    let bin_dir: String = format!("{project_name}/bin");
    let main_file: String = format!("{project_name}/main.c");
    // let makefile: String = format!("{project_name}/makefile");
    create_dir(project_name)?;
    create_dir(bin_dir)?;

    let mut mainc: File = File::create_new(main_file)?;
    mainc.write_all(MAIN_C_FILE_CONTENT.as_bytes())?;

    // let mut mkfl = File::create_new(makefile)?;
    // mkfl.write_all(MAKEFILE_CONTENT.as_bytes())?;

    /*Didn't used a makefile since it kills the use of `run`*/

    Ok(())
}

pub fn run_project() -> io::Result<()> {
    let cmd = Command::new("gcc")
        .arg("main.c")
        .arg("-o")
        .arg("./bin/main")
        .output()?;

    io::stdout().write_all(&cmd.stdout)?;
    io::stderr().write_all(&cmd.stderr)?;
    let bin = Command::new("./bin/main").output()?;

    io::stdout().write_all(&bin.stdout)?;
    io::stderr().write_all(&bin.stderr)?;

    Ok(())
}

pub fn init_project() -> io::Result<()> {
    create_dir("bin")?;
    let mut mainc: File = File::create_new("main.c")?;
    mainc.write_all(MAIN_C_FILE_CONTENT.as_bytes())?;

    Ok(())
}