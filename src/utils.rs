use std::fs::{File, self};
use std::io::{BufRead, BufReader};

use nix::libc::{getuid};

pub fn determine_distro() -> String {

    let mut distro_name = read_file("/etc/os-release", 1);

    distro_name = rem_chars(distro_name, 5);

    // DEBUG
    // println!("{}", distro_name);
    return distro_name;
}

pub fn determine_user() -> String {
    // let uid = getuid();
    


    return "turtle".to_string();
}

pub fn rem_chars(value: String, int_removed: i8) -> String {
    let mut chars = value.chars();
    // Skips a char
    for _ in 0..int_removed {
        chars.next();
    };
    return chars.as_str().to_string();
}

// Plan to make this configurable for number of lines read or which line read
fn read_file(filename: &str, line_num: u16) -> String {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let output;


    if line_num == 1 {
        // Just read the first line.
        output = reader.lines().next().unwrap().unwrap();
    } else if line_num == 0 {
        // Loop through all lines.
        // Honestly don't know why I would need this in this function right now.

        output = "taco".to_string(); // just here to stop errors;
        for (index, line) in reader.lines().enumerate() {
            let line= line.unwrap(); // unwrapping the result
            println!("{}. {}", index + 1, line);
        }

    } else {
        // This section is just for if you have a specific number of lines to go through.
        // Probably won't be useful ever
        output = "taco".to_string();
        
        for (index, line) in reader.lines().enumerate() {
            let line= line.unwrap(); // unwrapping the result
            println!("{}. {}", index + 1, line);

            // Stop looping through after making it to the sought after line
            if index + 1 == line_num.into() { break; }
        }

    }


    // With a loop
    // let mut output = None;
    // for (index, line) in reader.lines().enumerate() {
    //     let line= line.unwrap(); // unwrapping the result
    //     println!("{}. {}", index + 1, line);
    //     output = Some(line);
    //     break;
    // }
    // let output = output.unwrap(); // unwrapping the option

    return output;
}

// Probably should just output the pathbuf then convert to string later
pub fn readlink(filename: String) -> String {
    let path = fs::read_link(filename);
    return path.unwrap().into_os_string().into_string().unwrap();
}