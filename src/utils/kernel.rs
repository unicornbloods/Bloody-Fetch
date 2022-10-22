use uname::uname;

pub fn kernel() {
    print_kernel();
}

// Split this off so isn't directly accessed by main
fn print_kernel() {
    let info = uname().unwrap();

    println!("KERNEL: {}", info.release);
}