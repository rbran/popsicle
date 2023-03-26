use std::process::Command;

fn main() {
    let path_in = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path_out = std::env::var("OUT_DIR").unwrap();
    let simple_in = format!("{}/src/simple.s", path_in);
    let simple_out = format!("{}/simple.elf", path_out);
    Command::new("arm-none-eabi-gcc")
        .args(&["-nostdlib", &simple_in, "-o", &simple_out])
        .status()
        .unwrap()
        .success()
        .then_some(())
        .unwrap();
}
