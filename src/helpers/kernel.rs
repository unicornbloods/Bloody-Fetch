use uname::uname;

// Split this off so isn't directly accessed by main
pub fn kernel() -> String {
    let info = uname().unwrap();

    // println!("KERNEL: {}", info.release);
    return info.release;
}