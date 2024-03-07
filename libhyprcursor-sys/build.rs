fn main() {
    let _lib = pkg_config::Config::new()
        .atleast_version("0.1.0")
        .cargo_metadata(true)
        .probe("hyprcursor")
        .expect("Failed to find suitable libhyprcursor version");
}
