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
    writeln!(file_rs, "// Automatically converted from C++ using rustify; https://github.com/angrypig555/rustify")?;
    for (num, line) in file_cc.lines().enumerate() {
        let data = line?;
        let mut words_iter = data.split_whitespace();

        let Some(first_word) = words_iter.next() else {
            continue;
        };
        //let words: Vec<&str> = words_iter.collect();
        match first_word {
            "#include" => {
                match words_iter.next() {
                    Some("<iostream>") => {
                        println!("{OK} File imports iostream, nothing to do")
                    }
                    Some(library) => {
                        println!("{FAIL} File imports unknown library, requires manual intervention");
                        writeln!(file_rs, "// IMPORT OF UNKNOWN LIBRARY {}", library)?;
                    }
                    _ => {
                        println!("{FAIL} #include missing library name");
                        writeln!(file_rs, "// MALFORMED C++ CODE {}", data)?;
                    }
                }
                
            }
            "int" => {
                println!("{OK} Found integer");
                match words_iter.next() {
                    Some(name) => {
                        if name.contains("()") {
                            println!("{OK} Function named {}", name);
                            writeln!(file_rs, "fn {} {{", name)?;
                        } else {
                            println!("{OK} Integer named {}", name);
                            write!(file_rs, "let mut {}", name)?;
                            match words_iter.next() {
                                Some(operation) => {
                                    write!(file_rs, " {} ", operation)?;
                                    match words_iter.next() {
                                        Some(value) => {
                                            println!("{OK} Value of {} is {}", name, value);
                                            writeln!(file_rs, "{}", value)?;
                                        }
                                        None => {
                                            println!("{FAIL} Integer has name and operation but no value");
                                            writeln!(file_rs, "// INTEGER int {} {} HAS NO VALUE", name, operation)?;
                                        }
                                    }
                                }
                                None => {
                                    println!("{FAIL} Integer name was declared but operation was not");
                                    writeln!(file_rs, "// INTEGER {} HAS NO OPERATION", name)?;
                                }
                            }
                        }
                    }
                    None => {
                        println!("{FAIL} Integer has no name");
                        writeln!(file_rs, "// INTEGER HAS NO NAME BUT WAS DECLARED IN C++")?;
                    }
                }
            }
            "return" => {
                println!("{OK} Return statement");
                write!(file_rs, "return ")?;
                match words_iter.next() {
                    Some(code) => {
                        println!("{OK} Return statement has a value of {}", code);
                        if !code.contains(";") {
                            println!("{WARN} Return statement didnt contain semicolon at the end");
                            writeln!(file_rs, "{};", code)?;
                        } else {
                            writeln!(file_rs, "{}", code)?;
                        }
                    }
                    None => {
                        println!("{FAIL} Return statement has no value");
                        writeln!(file_rs, "// RETURN STATEMENT HAS NO VALUE: {}", data)?;
                    }
                }
            }
            "}" => {
                println!("{OK} Closing brace");
                writeln!(file_rs, "}}")?;
            }
            _ => {
                println!("{WARN} Unknown keyword detected, requires manual intervention");
                writeln!(file_rs, "// UNKNOWN KEYWORD {}", data)?;
            }
        }
    }
    Ok(())
}
