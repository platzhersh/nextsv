use std::error::Error;

use autocfg::AutoCfg;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");
    let ac = AutoCfg::new()?;
    // Test that we meet a minimum of 1.65 for let else
    if ac.probe_rustc_version(1, 65) {
        autocfg::emit("let_else")
    }

    Ok(())
}
