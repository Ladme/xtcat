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

    println!("Start: {}", start);

    // get length of the file
    let file_len = input.seek(SeekFrom::End(0))?;

    // jump to the position in input file from which it should be read
    input.seek(SeekFrom::Start(start as u64))?;

    // load the contents of the input file
    let mut input_file_contents: Vec<u8> = Vec::with_capacity((file_len as usize) - (start as usize));
    println!("{} {} {}", file_len, start, (file_len as usize) - (start as usize));
    let total_read = input.read_to_end(&mut input_file_contents)?;
    println!("Total read: {} bytes", total_read);

    // write the contents of the file to the output
    output.write_all(&input_file_contents)?;

    Ok(())

}

fn main() {
    let args: Vec<String> = env::args().collect();

    // sanity check the arguments
    if args.len() < 2 {
        eprintln!("Incorrect number of arguments.");
        println!("Usage: {} -f XTC_FILE1 XTC_FILE2 ... -o OUTPUT_XTC", args[0]);
        process::exit(1);
    }

    // open output file
    let mut output = match File::create("output.xtc") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error. Output file {} could not be opened for writing [{}]", "output.xtc", error);
            process::exit(2);
        }
    };

    let mut remove_first_frame = false;
    for file in args[1..args.len()].iter() {
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
