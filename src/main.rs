use cpu::Cpu;

use crate::memory::Memory;

pub mod cpu;
pub mod memory;
pub mod processor_status;
pub mod registers;

fn main()
{
    let mut mem = Memory::new();

    // inline a little program
    mem[0xFFFC] = Cpu::INS_JSR;
    mem[0xFFFD] = 0x42;
    mem[0xFFFE] = 0x42;
    mem[0x4242] = Cpu::INS_LDA_IM;
    mem[0x4243] = 0x84;

    let mut cpu = Cpu::new();
    cpu.execute(7, &mut mem);

    println!("{:#?}", cpu);
}
