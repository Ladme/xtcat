// Released under MIT License.
// Copyright (c) 2022 Ladislav Bartos

use std::env;
use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::process;


fn read_xdr_int(file: &mut File) -> Result<i32, io::Error> {
    let mut buffer = [0u8; 4];

    file.read_exact(&mut buffer)?;

    let result: i32 = (buffer[0] as i32) << 24 | (buffer[1] as i32) << 16 | (buffer[2] as i32) << 8 | (buffer[3] as i32);

    Ok(result)
}

fn add_xtc(input_path: &str, output: &mut File, remove_first_frame: bool) -> Result<(), io::Error> {
    // try opening the input file
    let mut input = File::open(input_path)?;

    let mut start = 0;
    if remove_first_frame {
        // read the size of the first frame
        input.seek(SeekFrom::Start(88))?;
        start = read_xdr_int(&mut input)? + 92;

        // the size of the frame in bytes must be divisible by 4
        // therefore, we add some padding
        if start % 4 != 0 {
            start += 4 - (start % 4);
        }
    }

    // get length of the file
    let file_len = input.seek(SeekFrom::End(0))?;

    // jump to the position in input file from which it should be read
    input.seek(SeekFrom::Start(start as u64))?;

    // load the contents of the input file
    let mut input_file_contents: Vec<u8> = Vec::with_capacity((file_len as usize) - (start as usize));
    input.read_to_end(&mut input_file_contents)?;

    // write the contents of the file to the output
    output.write_all(&input_file_contents)?;

    Ok(())

}

fn parse_arguments(args: &Vec<String>) -> (Vec<String>, String) {
    let mut input_files = Vec::new();
    let mut output_file = String::from("output.xtc");

    let mut input_block = false;
    for mut i in 1..args.len() {
        if args[i] == "-o" {
            i += 1;
            output_file = args[i].clone();
            input_block = false;
            continue;
        }

        if input_block {
            input_files.push(args[i].clone());
            continue;
        }

        if args[i] == "-f" {
            input_block = true;
        }
    }

    (input_files, output_file)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // help option
    for arg in &args {
        if arg == "-h" || arg == "--help" {
            println!("Usage: {} -f XTC_FILE1 XTC_FILE2 ... -o OUTPUT_XTC", args[0]);
            process::exit(0);
        }
    }

    // sanity check the arguments
    if args.len() < 3 {
        eprintln!("Incorrect number of arguments.");
        println!("Usage: {} -f XTC_FILE1 XTC_FILE2 ... -o OUTPUT_XTC", args[0]);
        process::exit(1);
    }

    // get input files and output file
    let (input_files, output_file) = parse_arguments(&args);

    print!("Concatenating {} files: ", input_files.len());
    for file in &input_files {
        print!("{} ", &file);
    }
    println!("\nOutput file: {}\n", &output_file);

    // open output file
    let mut output = match File::create(&output_file) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error. Output file {} could not be opened for writing [{}]", &output_file, error);
            process::exit(1);
        }
    };

    let mut remove_first_frame = false;
    for file in &input_files {
        println!("Concatenating file {}...", &file);
        io::stdout().flush().unwrap();

        match add_xtc(&file, &mut output, remove_first_frame) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("Error. Could not read file {} [{}]", &file, error);
                process::exit(1);
            }
        }
        remove_first_frame = true;
    }

}
