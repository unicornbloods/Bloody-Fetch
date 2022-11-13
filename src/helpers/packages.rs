use walkdir::WalkDir;

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

    // // Need to replicate "ls -d /var/db/pkg/*/*" for gentoo
    let total_packages = 
    WalkDir::new("/var/db/pkg/")
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_dir())
        // .inspect(|v| eprintln!("{v:?}")) // For debug
        .count();

    return total_packages as u16;
}

