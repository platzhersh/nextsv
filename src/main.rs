fn main() {
    println!("Hello World");
    match nextsv_lib::get_latest_version_tag() {
        Ok(version) => println!("Latest version is: {}", version),
        Err(e) => println!("Error: {}", e),
    };
}
