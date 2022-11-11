use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn determine_distro() -> String {

    let mut distro_name = read_file("/etc/os-release");

    distro_name = rem_chars(distro_name, 5);

    // DEBUG
    // println!("{}", distro_name);
    return distro_name;
}

fn rem_chars(value: String, int_removed: i8) -> String {
    let mut chars = value.chars();
    // Skips a char
    for _ in 0..int_removed {
        chars.next();
    };
    return chars.as_str().to_string();
}

// Plan to make this configurable for number of lines read or which line read
fn read_file(filename: &str) -> String {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let output = reader.lines().next().unwrap().unwrap();

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