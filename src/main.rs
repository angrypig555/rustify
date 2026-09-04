use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::Command;

// declare colored status indicators
const WARN: &str = "[\x1b[33mWARN\x1b[0m]";
const OK: &str = "[\x1b[32mOK\x1b[0m]";
const FAIL: &str = "[\x1b[31mFAIL\x1b[0m]";

// TODO:
// 1. Change comments regarding errors in the code to compile errors
// 2. when detecting specific keywords, set a boolean to true that signifies that it requires a library

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.iter().count() < 3 {
        panic!("{FAIL} Not enough arguments\nUsage: rustify [c_source_file] [output_file_name]");
    } // collect and count arguments
    println!("rustify - Licensed under the MIT license. Copyright (c) 2026 angrypig555");
    let file_raw = match File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            panic!("{FAIL} Failed to open file! {e}");
        }
    }; // attempt to open the c++ source file and catch errors
    println!("{OK} Opened file {}", &args[1]);
    let file_rs_raw = match File::create(&args[2]) {
        Ok(f) => f,
        Err(e) => {
            panic!("{FAIL} Failed to create file! {e}");
        }
    }; // attemp to create the rust source file
    println!("{OK} Created file {}", &args[2]);
    let file_cc = BufReader::new(file_raw);
    let mut file_rs = BufWriter::new(file_rs_raw); // setup bufreader/bufwriter for easiear reading and writing
    writeln!(file_rs, "// Automatically converted from C++ using rustify; https://github.com/angrypig555/rustify\nuse std::process::ExitCode;")?;
    for (num, line) in file_cc.lines().enumerate() { // read the c++ code line by line
        let data = line?;
        let mut words_iter = data.split_whitespace(); // collect the words inside the line

        let Some(first_word) = words_iter.next() else { // check if a line is empty (crashes if this is not here)
            continue;
        };
        //let words: Vec<&str> = words_iter.collect();
        match first_word {
            "#include" => {
                match words_iter.next() {
                    Some("<iostream>") => {
                        println!("{OK} File imports iostream, nothing to do")
                    }
                    Some(library) => { // add comment if the library is unkown
                        println!("{FAIL} File imports unknown library, requires manual intervention");
                        writeln!(file_rs, "compile_error!(\"IMPORT OF UNKNOWN LIBRARY {}\")", library)?;
                    }
                    _ => { // add comment if the c++ code is incorrect
                        println!("{FAIL} #include missing library name");
                        writeln!(file_rs, "compile_error!(\"MALFORMED C++ CODE {}\")", data)?;
                    }
                }
                
            }
            "int" => { // handling integers / functions returning integers
                println!("{OK} Found integer");
                match words_iter.next() {
                    Some(name) => { // check if its a function or an integer
                        if name.contains("()") {
                            println!("{OK} Function named {}", name);
                            writeln!(file_rs, "fn {} -> ExitCode {{", name)?;
                        } else { // if its an integer we check for the operation
                            println!("{OK} Integer named {}", name);
                            write!(file_rs, "let mut {}", name)?;
                            match words_iter.next() {
                                Some(operation) => {
                                    write!(file_rs, " {} ", operation)?;
                                    match words_iter.next() {
                                        Some(value) => {
                                            println!("{OK} Value of {} is {}", name, value);
                                            writeln!(file_rs, "{}", value)?;
                                        } // if it has an operation but no value for some reason, it comments in the code
                                        None => {
                                            println!("{FAIL} Integer has name and operation but no value");
                                            writeln!(file_rs, "compile_error!(\"INTEGER int {} {} HAS NO VALUE\")", name, operation)?;
                                        }
                                    }
                                }
                                None => { // if the operation and value are missing, it writes a comment in the code
                                    println!("{FAIL} Integer name was declared but operation was not");
                                    writeln!(file_rs, "compile_error!(\"INTEGER {} HAS NO OPERATION\")", name)?;
                                }
                            }
                        }
                    }
                    None => { // if the integer has no name (e.g just "int") then it puts a comment in the code, may possibly change these later on to a compile error
                        println!("{FAIL} Integer has no name");
                        writeln!(file_rs, "compile_error!(\"INTEGER HAS NO NAME BUT WAS DECLARED IN C++\")")?;
                    }
                }
            }
            "void" => { // handles void functions
                println!("{OK} Void function"); // no need to check if its a value or a function because voids cant return anything
                match words_iter.next() {
                    Some(name) => {
                        println!("{OK} Void function is named {}", name);
                        writeln!(file_rs, "fn {} {{", name)?;
                    }
                    None => { // again, if its incorrect it comments in the code
                        println!("{FAIL} Void function has malformed C++ code");
                        writeln!(file_rs, "compile_error!(\"VOID FUNCTION WAS DECLARED BUT HAS NO NAME\")")?;
                    }
                }
            }
            "return" => { // handles the return statement
                println!("{OK} Return statement");
                write!(file_rs, "return ")?;
                match words_iter.next() { // check if there is actually a return code there
                    Some(code) => {
                        println!("{OK} Return statement has a value of {}", code);
                        let clean_code = code.trim_end_matches(";");
                        writeln!(file_rs, "ExitCode::from({});", clean_code)?;
                    }
                    None => { // if it has no value it comments in the code
                        println!("{FAIL} Return statement has no value");
                        writeln!(file_rs, "compile_error!(\"RETURN STATEMENT HAS NO VALUE: {}\")", data)?;
                    }
                }
            }
            "}" => { // no need for error checking here because its just a closing brace
                println!("{OK} Closing brace");
                writeln!(file_rs, "}}")?;
            }
            "//" => { // again, no error checking for comments becasue there is nothing to check for
                println!("{OK} Found comment");
                write!(file_rs, "//")?;
                for word in words_iter {
                    write!(file_rs, " {}", word)?;
                }
                write!(file_rs, "\n")?;
            }
            _ => { // if there is an unknown keyword (not implemented yet or misspelled) we leave a comment in the code
                println!("{WARN} Unknown keyword detected, requires manual intervention");
                writeln!(file_rs, "compile_error!(\"UNKNOWN KEYWORD {}\")", data)?;
            }
        }
    }
    file_rs.flush()?;
    println!("{OK} Compiling converted code...");
    let out_dir = std::path::Path::new(&args[2])
        .parent()
        .unwrap_or(std::path::Path::new("."));
    let output = Command::new("rustc")
        .args([format!("{}", &args[2]), "--out-dir".to_string(), format!("{}", out_dir.display())])
        .status()?;
    println!("{OK} Compiled program can be found next to executable");
    Ok(())
}
