use std::env;
use std::fs;

mod locker;

use locker::{decrypt_file, encrypt_file};

#[derive(PartialEq, Eq)]
enum Mode {
    None,
    Lock,
    Unlock,
}

struct ProgramData {
    pub mode: Mode,
    pub file_path: String,
    pub password: String,
}

impl ProgramData {
    pub fn new() -> ProgramData {
        ProgramData {
            mode: Mode::None,
            file_path: String::new(),
            password: String::new(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program_data: ProgramData = ProgramData::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-l" => {
                if program_data.mode == Mode::None {
                    program_data.mode = Mode::Lock;
                    i += 1;
                    program_data.file_path = args[i].clone();
                } else {
                    panic!("Mode already specified")
                }
            }
            "-u" => {
                if program_data.mode == Mode::None {
                    program_data.mode = Mode::Unlock;
                    i += 1;
                    program_data.file_path = args[i].clone();
                } else {
                    panic!("Mode already specified")
                }
            }
            "-p" => {
                i += 1;
                program_data.password = args[i].clone();
            }
            _ => {
                panic!("Unexpected parameter")
            }
        }
        i += 1;
    }

    if program_data.mode == Mode::None {
        panic!("Mode not specified")
    }

    if program_data.file_path == String::default() {
        panic!("File Path not specified")
    }

    if program_data.password == String::default() {
        panic!("Password not specified")
    }

    let file = fs::read(&program_data.file_path).expect("Unable to read file");

    let key = program_data.password.as_bytes();

    let new_file: Vec<u8> = match program_data.mode {
        Mode::Lock => encrypt_file(file, key),
        Mode::Unlock => decrypt_file(file, key),
        Mode::None => encrypt_file(file, key),
    };

    let out_path = match program_data.mode {
        Mode::Lock => program_data.file_path + ".lck",
        Mode::Unlock => program_data.file_path + ".dec",
        Mode::None => program_data.file_path + ".lck",
    };
    fs::write(out_path, new_file).unwrap();
}
