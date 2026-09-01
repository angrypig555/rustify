use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

const WARN: &str = "[\x1b[33mWARN\x1b[0m]";
const OK: &str = "[\x1b[32mOK\x1b[0m]";
const FAIL: &str = "[\x1b[31mFAIL\x1b[0m]";

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.iter().count() < 3 {
        panic!("{FAIL} Not enough arguments\nUsage: rustify [c_source_file] [output_file_name]");
    }
    println!("rustify - Licensed under the MIT license. Copyright (c) 2026 angrypig555");
    let file_raw = match File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            panic!("{FAIL} Failed to open file! {e}");
        }
    };
    println!("{OK} Opened file {}", &args[1]);
    let file_rs_raw = match File::create(&args[2]) {
        Ok(f) => f,
        Err(e) => {
            panic!("{FAIL} Failed to create file! {e}");
        }
    };
    println!("{OK} Created file {}", &args[2]);
    let file_cc = BufReader::new(file_raw);
    let mut file_rs = BufWriter::new(file_rs_raw);
    for (num, line) in file_cc.lines().enumerate() {
        let data = line?;
        match data.as_str() {
            "#include <iostream>" => {
                println!("{OK} File imports iostream, nothing to do");
            }
            "int main() {" => {
                println!("{OK} Found main function");
                writeln!(file_rs, "fn main() {{")?;
            }
            _ => {
                println!("{WARN} Unknown function detected, skipping");
            }
        }
    }
    Ok(())
}
