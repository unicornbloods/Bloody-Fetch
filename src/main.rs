// Author: Dakotah Beckmann
// Started: 2022 / 10 / 21
// Description: neofetch like program written in rust, my take for fun.

// TODO: User, wm, Pixel Art, Flgs like -V

mod helpers;
mod utils;

fn main() {

    // println!("USER: {}", $UID);
    let distro = utils::determine_distro();
    println!("OS: {}", distro);

    let kernel = helpers::kernel::kernel();
    let packages = helpers::packages::packages(distro);
    // let shell = helpers::shell::shell(); // Currently just returns the executable's directory


    println!("KERNEL: {}", kernel);
    println!("PACKAGES: {}", packages);
    // println!("Shell: {}", shell);


}