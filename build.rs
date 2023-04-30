use std::process::Command;

fn compile_arm(path_in: &str, path_out: &str) {
    const COMPILER: &str = "arm-none-eabi-gcc";
    let asm_file = format!("{}/src/arm-simple.s", path_in);
    let elf_file = format!("{}/arm-simple.elf", path_out);
    Command::new(COMPILER)
        .arg("-nostdlib")
        .arg("-static")
        .arg(asm_file)
        .arg("-o")
        .arg(elf_file)
        .status()
        .unwrap()
        .success()
        .then_some(())
        .unwrap();
}

fn compile_mips(path_in: &str, path_out: &str) {
    const COMPILER: &str = "/home/rbran/src/my-uled/openwrt-toolchain-22.03.2-ath79-generic_gcc-11.2.0_musl.Linux-x86_64/toolchain-mips_24kc_gcc-11.2.0_musl/bin/mips-openwrt-linux-gcc";
    let asm_file = format!("{}/src/mips-simple.s", path_in);
    let elf_file = format!("{}/mips-simple.elf", path_out);
    Command::new(COMPILER)
        .arg("-nostdlib")
        .arg("-static")
        .arg(asm_file)
        .arg("-o")
        .arg(elf_file)
        .status()
        .unwrap()
        .success()
        .then_some(())
        .unwrap();
}

fn main() {
    let path_in = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path_out = std::env::var("OUT_DIR").unwrap();
    compile_arm(&path_in, &path_out);
    compile_mips(&path_in, &path_out);
}
