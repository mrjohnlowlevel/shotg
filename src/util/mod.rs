use std::fs::{create_dir, File};
use std::io::{self, Write};

const MAIN_C_FILE_CONTENT: &'static str = "#include <stdio.h>\n\nint main()\n{\n\tprintf(\"Hello, World!\");\n}";
const MAKEFILE_CONTENT: &'static str = "SRC = main.c\nCC = gcc\nBIN = main\n\nall:\n\t$(CC) $(SRC) -o bin/$(BIN)";

pub enum Commands {
    New,
    Init,
    Run,
}

pub fn match_command(cmd: String) -> Option<Commands> {
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

pub fn new_project(project_name: String) -> io::Result<()> {
    let bin_dir: String = format!("{project_name}/bin");
    let main_file: String = format!("{project_name}/main.c");
    let makefile: String = format!("{project_name}/makefile");
    create_dir(project_name)?;
    create_dir(bin_dir)?;

    let mut mainc: File = File::create_new(main_file)?;
    mainc.write_all(MAIN_C_FILE_CONTENT.as_bytes())?;

    let mut mkfl = File::create_new(makefile)?;
    mkfl.write_all(MAKEFILE_CONTENT.as_bytes())?;

    Ok(())
}