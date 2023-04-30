fn emu_arm() {
    let cpu_config =
        icicle_vm::cpu::Config::from_target_triple("armv8-unknown-unknown");
    let mut vm = icicle_vm::build(&cpu_config).unwrap();

    let mut env = icicle_vm::env::build_auto(&mut vm).unwrap();
    let binary = format!("{}/arm-simple.elf", env!("OUT_DIR"));
    env.load(&mut vm.cpu, binary.as_bytes()).unwrap();
    vm.env = env;

    let exit = vm.run();
    println!("exit: {exit:?}");
    for reg_name in ["r0", "pc"] {
        let reg = vm
            .cpu
            .arch
            .sleigh
            .get_reg(reg_name)
            .map(|reg| reg.var)
            .unwrap();
        println!("{} {:#X}", reg_name, vm.cpu.read_reg(reg));
    }
}

fn emu_mips() {
    let cpu_config =
        icicle_vm::cpu::Config::from_target_triple("mips-unknown-unknown");
    let mut vm = icicle_vm::build(&cpu_config).unwrap();

    let mut env = icicle_vm::env::build_auto(&mut vm).unwrap();
    let binary = format!("{}/mips-simple.elf", env!("OUT_DIR"));
    env.load(&mut vm.cpu, binary.as_bytes()).unwrap();
    vm.env = env;

    let exit = vm.run();
    println!("exit: {exit:?}");
    for reg_name in ["v0", "pc"] {
        let reg = vm
            .cpu
            .arch
            .sleigh
            .get_reg(reg_name)
            .map(|reg| reg.var)
            .unwrap();
        println!("{} {:#X}", reg_name, vm.cpu.read_reg(reg));
    }
}

fn main() {
    println!("arm: ");
    emu_arm();
    println!("\n");
    println!("mips: ");
    emu_mips();
    println!("\n");
}
