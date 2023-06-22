// Released under MIT License.
// Copyright (c) 2022-2023 Ladislav Bartos

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::process;

use clap::Parser;
use colored::Colorize;

use termion::{clear, cursor};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Concatenates XTC trajectories from simulations that directly follow each other. `xtcat` removes the first frame of each subsequent trajectory as it matches the last frame of the previous trajectory but does not renumber the frames or remove duplicate frames."
)]
struct Args {
    /// XTC files to concatenate
    #[clap(short = 'f', long = "files", num_args = 1.., value_delimiter = ' ', required = true)]
    input_files: Vec<String>,
    /// Name of the output file to save the concatenated trajectory to
    #[clap(short = 'o', long = "output", required = true)]
    output: String,
    /// Do not print anything to standard output.
    #[clap(short = 's', long = "silent")]
    silent: bool,
}

/// Read an integer from an xdr file.
fn read_xdr_int(file: &mut File) -> Result<i32, io::Error> {
    let mut buffer = [0u8; 4];

    file.read_exact(&mut buffer)?;

    let result: i32 = (buffer[0] as i32) << 24
        | (buffer[1] as i32) << 16
        | (buffer[2] as i32) << 8
        | (buffer[3] as i32);

    Ok(result)
}

/// Append an xtc file to the end of open output file.
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
    let mut input_file_contents: Vec<u8> =
        Vec::with_capacity((file_len as usize) - (start as usize));
    input.read_to_end(&mut input_file_contents)?;

    // write the contents of the file to the output
    output.write_all(&input_file_contents)?;

    Ok(())
}

/// Clear the terminal screen.
fn clear_progress(len: u16) {
    print!("{}", cursor::Up(len));
    print!("{}", clear::AfterCursor);
}

/// Print the current progress with reading input files.
fn print_progress(input_files: &Vec<String>, current_index: usize, success: bool) {
    for (i, file) in input_files.iter().enumerate() {
        if !success && i == current_index - 1 {
            println!("{}", file.red());
            continue;
        }

        if i < current_index {
            println!("{}", file.green());
        } else {
            println!("{}", file);
        };
    }
}

fn main() {
    let args = Args::parse();

    if !args.silent {
        println!("{}", "\nXTCAT v0.3.0\n".bold());
    }

    // open output file
    let mut output = match File::create(&args.output) {
        Ok(file) => file,
        Err(_) => {
            let error = format!(
                "Error. Output file '{}' could not be opened for writing.",
                &args.output
            );
            eprintln!("{}", error.red().bold());
            process::exit(1);
        }
    };

    // print input file
    if !args.silent {
        println!(
            "Concatenating {} files into '{}'...",
            &args.input_files.len(),
            &args.output.green()
        );
        print_progress(&args.input_files, 0, true);
    }

    // read all input files
    let mut remove_first_frame = false;
    for (i, file) in args.input_files.iter().enumerate() {
        match add_xtc(&file, &mut output, remove_first_frame) {
            Ok(_) => {
                if !args.silent {
                    clear_progress(args.input_files.len() as u16);
                    print_progress(&args.input_files, i + 1, true);
                }
            }

            Err(_) => {
                if !args.silent {
                    clear_progress(args.input_files.len() as u16);
                    print_progress(&args.input_files, i + 1, false);
                }
                let error = format!("Error. Could not read file '{}'. Aborting...", file);
                println!("\n{}", error.red().bold());

                process::exit(2);
            }
        }
        remove_first_frame = true;
    }

    drop(output);

    if !args.silent {
        let result = format!("\nSuccessfully written output file '{}'.", &args.output);
        println!("{}", result.green().bold());
    }
}
