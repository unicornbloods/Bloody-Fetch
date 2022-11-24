// Author: Dakotah Beckmann
// Started: 2022 / 10 / 21
// Description: neofetch like program written in rust, my take for fun.

// TODO: User, wm, Pixel Art, Flgs like -V

use crate::utils::determine_user;

mod helpers;
mod utils;

fn main() {

    let user_data = determine_user();
    let shell = &user_data[1];
    let user = &user_data[0];

    println!("USER: {}", user);
    let distro = utils::determine_distro();
    println!("OS: {}", distro);

    let kernel = helpers::kernel::kernel();
    let packages = helpers::packages::packages(distro);



    println!("KERNEL: {}", kernel);
    println!("PACKAGES: {}", packages);
    println!("Shell: {}", shell);


}