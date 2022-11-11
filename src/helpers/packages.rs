use scan_dir::*;

pub fn packages(distro: String) -> u16{
    // Mutable so I can add together multiple package manager packages.
    // Looking at you bedrock linux.
    let mut total_packages: u16 = 0;
    
    // TODO: check if system is even gentoo first
    if distro == "Gentoo" {
        total_packages += gentoo_emerge();
    }

    // In future, maybe add support to other systems

    // println!("PACKAGES: {}", total_packages);
    return total_packages;
}

// Calculate the packages for emerge
fn gentoo_emerge() -> u16 {
    let mut total_packages: u16 = 0;

    // Need to replicate "ls -d /var/db/pkg/*/*" for gentoo
    ScanDir::dirs().read("/var/db/pkg", |iter| {
        for (entry, _name) in iter {
            ScanDir::dirs().read(entry.path() , |iter| {
                for _entry in iter {
                    total_packages += 1;
                }
            }).unwrap(); // End second layer depth

        }
    }).unwrap(); // End first layer depth

    return total_packages;
}

