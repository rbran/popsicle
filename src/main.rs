fn main() {
    // Setup the CPU state for the target triple
    let cpu_config =
        icicle_vm::cpu::Config::from_target_triple("armv8-unknown-unknown");
    let mut vm = icicle_vm::build(&cpu_config).unwrap();

    // Setup an environment to run inside of.
    let mut env = icicle_vm::env::build_auto(&mut vm).unwrap();
    // Load a binary into the environment.
    let binary = format!("{}/simple.elf", env!("OUT_DIR"));
    env.load(&mut vm.cpu, binary.as_bytes()).unwrap();
    let entry = env.lookup_symbol("_start").unwrap();
    vm.env = env;

    let vm: *mut _ = &mut vm;
    unsafe {
        for i in (entry..).into_iter().step_by(4).take(2) {
            (*vm).hook_address(i, move |cpu, addr| {
                println!("{:#08X}: {:?}", addr, (*vm).get_disasm(addr));
                for reg_name in ["r0", "r1", "pc"] {
                    let reg = cpu
                        .arch
                        .sleigh
                        .get_reg(reg_name)
                        .map(|reg| reg.var)
                        .unwrap();
                    print!("{} {:#08X}  ", reg_name, cpu.read_reg(reg));
                }
                println!("\n");
            });
        }

        // Run 2 steps
        let exit = (*vm).step(2);
        println!("exit: {exit:?}");
    }
}
