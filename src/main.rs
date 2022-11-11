// Author: Dakotah Beckmann
// Started: 2022 / 10 / 21
// Description: neofetch like program written in rust, my take for fun.

// TODO: User, wm, Pixel Art

mod helpers;
mod utils;

fn main() {

    // println!("Bloody Fetch");
    let distro = utils::determine_distro();
    println!("OS: {}", distro);

    let packages = helpers::packages::packages(distro);
    let kernel = helpers::kernel::kernel();

    println!("PACKAGES: {}", packages);
    println!("KERNEL: {}", kernel);

}