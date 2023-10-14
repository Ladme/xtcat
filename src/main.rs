// Released under MIT License.
// Copyright (c) 2022-2023 Ladislav Bartos

use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
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
    /// Do not print anything to standard output
    #[clap(short = 's', long = "silent")]
    silent: bool,
    /// Overwrite any file sharing the name with the output file
    #[clap(long = "overwrite")]
    overwrite: bool,
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
    print!("{}", cursor::Up(len + 2));
    print!("{}", clear::AfterCursor);
}

/// Print the current progress of reading input files.
fn print_progress(input_files: &Vec<String>, current_index: usize, success: bool) {
    for (i, file) in input_files.iter().enumerate() {
        let trajectory = format!("[XTC {}]", i + 1);

        if !success && i == current_index - 1 {
            println!("{:12} {} {}", trajectory, file.red(), "✖".red());
            continue;
        }

        if i < current_index {
            println!("{:12} {} {}", trajectory, file.green(), "✓".green());
        } else if i == current_index && success {
            println!("{:12} {} {}", trajectory, file.yellow(), "⚙".yellow());
        } else {
            println!("{:12} {} ⧗", trajectory, file);
        };
    }

    if current_index == input_files.len() && success {
        println!(
            "[{}]       {}/{} files concatenated\n",
            "DONE".green(),
            current_index.to_string().green(),
            input_files.len()
        );
    } else if success {
        println!(
            "[{}]    {}/{} files concatenated\n",
            "RUNNING".yellow(),
            current_index.to_string().yellow(),
            input_files.len()
        );
    } else {
        println!(
            "[{}]     {}/{} files concatenated\n",
            "FAILED".red(),
            (current_index - 1).to_string().red(),
            input_files.len()
        );
    }
}

/// Create a suitable name for backing up a file.
fn name_for_backup(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let filename = path.file_name().unwrap().to_string_lossy();
    let mut number = 1;

    let mut new_path = parent.join(format!("#{}.{}#", filename, number));

    while new_path.exists() {
        number += 1;
        new_path = parent.join(format!("#{}.{}#", filename, number));
    }

    new_path
}

fn main() {
    let args = Args::parse();

    if !args.silent {
        println!("{}", "\n   >> XTCAT v0.3.1 <<\n".bold());
    }

    // check that the output file does not already exist
    // if it does, back it up
    if !args.overwrite {
        let path = Path::new(&args.output);
        if path.exists() {
            let new_path = name_for_backup(path);

            fs::rename(Path::new(&args.output), Path::new(&new_path))
                .expect("Panic! Could not backup file.");

            if !args.silent {
                let output = format!(
                    "Output file '{}' already exists. Backing it up as '{}'",
                    &args.output,
                    &new_path.to_str().unwrap()
                );
                println!("[OUTPUT]     {}", output.yellow());
            }
        }
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

    // print initial progress
    if !args.silent {
        print_progress(&args.input_files, 0, true);
    }

    // read all input files
    let mut remove_first_frame = false;
    for (i, file) in args.input_files.iter().enumerate() {
        match add_xtc(file, &mut output, remove_first_frame) {
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
                println!("{}", error.red().bold());

                process::exit(2);
            }
        }
        remove_first_frame = true;
    }

    drop(output);

    if !args.silent {
        let result = format!("Successfully written output file '{}'.", &args.output);
        println!("{}", result.green().bold());
    }
}
