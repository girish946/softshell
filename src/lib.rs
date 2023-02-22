use pyo3::prelude::*;
use sys_info;

/// Returns the distribution name along with package manager
#[pyfunction]
fn get_dist_info() -> PyResult<String> {
    let dist_release_info = sys_info::linux_os_release().unwrap();
    let dist_id = dist_release_info.id.unwrap(); 
    println!("os release is :{dist_id}");
    if dist_id.eq("fedora"){
        return Ok(dist_id + "dnf");
    }
    Ok(dist_id)
}

/// A Python module implemented in Rust.
#[pymodule]
fn softsh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_dist_info, m)?)?;
    Ok(())
}
