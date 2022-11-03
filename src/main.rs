// Author: Dakotah Beckmann
// Started: 2022 / 10 / 21
// Description: neofetch like program written in rust, my take for fun.

// TODO: User, wm, Pixel Art

mod helpers;

fn main() {

    // println!("Bloody Fetch");
    println!("OS: Gentoo Linux");
    helpers::packages::packages();
    helpers::kernel::kernel();
}