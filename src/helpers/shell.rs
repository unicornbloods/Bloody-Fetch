use std::{process};

use crate::utils;

pub fn shell() -> String {
    // readlink /proc/$$/exe
    let pid = process::id();
    let file_path = format!("/proc/{pid}/exe");
    let active_shell = utils::readlink(file_path);

    return active_shell;
}