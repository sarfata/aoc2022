use std::collections::HashSet;
use std::env;
use std::fs;

// 6a start-of-packet
// const prefix_size: usize = 4;
// 6b start-of-message
const PREFIX_SIZE: usize = 14;

fn process_buf(buf: &str) -> Result<usize, &'static str> {
    for i in PREFIX_SIZE..buf.len() {
        let set: HashSet<char> = buf[i - PREFIX_SIZE..i].chars().collect();
        if set.len() == PREFIX_SIZE {
            return Ok(i);
        }
    }
    Err("Did not find four distinct letters in a row.")
}

fn main() -> Result<(), &'static str> {
    let args = env::args().collect::<Vec<String>>();
    let filepath_arg = args.get(1);

    if let Some(filepath) = filepath_arg {
        let content = fs::read_to_string(filepath);
        match content {
            Ok(buf) => {
                let index = process_buf(&buf)?;
                println!("BEEP BEEP - Data starts at {index}");
                return Ok(());
            }
            Err(_) => Err::<(), &str>(&"Error reading file content."),
        }
    } else {
        Err("Please provide one argument with the file containing the data.")
    }
}
