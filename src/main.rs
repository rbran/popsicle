use icicle_vm::cpu::mem::{perm, AllocLayout, Mapping};

fn main() {
    let cpu_config = icicle_vm::cpu::Config {
        triple: "armv8-unknown-unknown".parse().unwrap(),
        enable_shadow_stack: false,
        ..icicle_vm::cpu::Config::default()
    };
    let mut vm = icicle_vm::build(&cpu_config).unwrap();
    let pc = vm.cpu.arch.sleigh.get_reg("pc").unwrap().var;

    let code = vm
        .cpu
        .mem
        .alloc_memory(
            AllocLayout {
                addr: None,
                size: 16,
                align: 4,
            },
            Mapping {
                perm: perm::ALL,
                value: 0x00,
            },
        )
        .unwrap();
    vm.cpu
        .mem
        .write_bytes(code, &[
         0x00, 0x00, 0xa0, 0xe1, //nop
         0x00, 0x00, 0xa0, 0xe1, //nop
        ], perm::ALL)
        .unwrap();

    vm.cpu.write_reg(pc, code);
    vm.add_breakpoint(code + 4);
    vm.add_breakpoint(code + 8);
    let exit = vm.run();
    println!("exit: {exit:?}");
    vm.remove_breakpoint(code + 4);
    vm.remove_breakpoint(code + 8); // subtraction with overflow
    println!("pc {:#X}", vm.cpu.read_reg(pc));
}
