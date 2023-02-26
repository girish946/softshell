use pyo3::prelude::*;

use std::env;
use sys_info;
use which::which;

/// returns the package manager for macos.
fn get_macos_info() -> String {
    match which("brew"){
        Ok(_brew) => return String::from("macos:brew"),
        Err(_) => {
            println!("brew not found. Please visit https://brew.sh/ to install it.");
            return String::from("macos");
        }
    }
}

/// returns the package info about linux distro
fn get_linux_info() -> String {
    let dist_release_info = sys_info::linux_os_release().unwrap();
    let dist_id = dist_release_info.id.unwrap();
    println!("os release is :{dist_id}");
    if dist_id.eq("fedora") {
        match which("dnf") {
            Ok(_dnf) => return dist_id + ":dnf",
            Err(_) => return dist_id
        };
    }
    if dist_id.eq("ubuntu") {
        return dist_id + ":apt";
    }
    dist_id
}

/// returns the package manager for windows 
fn get_windows_info()-> String {
    match which("winget.exe"){
        Ok(_winget) => return String::from("windows:winget"),
        Err(_) => {
            println!("winget not found.");
            println!("Please visit https://github.com/microsoft/winget-cli, to install it.");
            return String::from("windows");
        }
    }
}

/// Returns the distribution name along with package manager
#[pyfunction]
fn get_dist_info() -> PyResult<String> {
    match env::consts::OS {
        "linux" => Ok(get_linux_info()),
        "macos" => Ok(get_macos_info()),
        "windows" => Ok(get_windows_info()),
        _ => Ok(String::from("unsupported distro")),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn softsh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_dist_info, m)?)?;
    Ok(())
}
