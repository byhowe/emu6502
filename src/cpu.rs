use std::any::type_name;
use std::fmt;
use std::mem;
use std::ops;

use crate::memory::Memory;
use crate::processor_status::ProcessorStatus;
use crate::registers::Register;
use crate::registers::Registers;

#[derive(Clone, Copy)]
pub struct Cpu
{
    program_counter: u16,
    stack_pointer: u16,
    registers: Registers,
    processor_status: ProcessorStatus,
}

impl ops::Index<Register> for Cpu
{
    type Output = u8;

    fn index(&self, index: Register) -> &Self::Output
    {
        &self.registers[index]
    }
}

impl ops::IndexMut<Register> for Cpu
{
    fn index_mut(&mut self, index: Register) -> &mut Self::Output
    {
        &mut self.registers[index]
    }
}

impl Cpu
{
    pub const INS_LDA_IM: u8 = 0xA9;
    pub const INS_LDA_ZP: u8 = 0xA5;
    pub const INS_LDA_ZPX: u8 = 0xB5;
    pub const INS_JSR: u8 = 0x20;

    pub fn new() -> Self
    {
        let mut cpu = mem::MaybeUninit::<Self>::uninit();
        unsafe {
            cpu.assume_init_mut().reset();
            cpu.assume_init()
        }
    }

    pub fn reset(&mut self)
    {
        self.program_counter = 0xFFFC;
        self.stack_pointer = 0x0100;
        self.registers = Registers::new();
        self.processor_status = ProcessorStatus::empty();
    }

    #[inline(always)]
    fn fetch8(&mut self, cycles: &mut u32, mem: &Memory) -> u8
    {
        let data = mem[self.program_counter as u32];
        self.program_counter += 1;
        *cycles -= 1;
        data
    }

    #[inline(always)]
    fn fetch16(&mut self, cycles: &mut u32, mem: &Memory) -> u16
    {
        // 6502 is little endian. Assuming the host platform is also little endian.
        self.fetch8(cycles, mem) as u16 | (self.fetch8(cycles, mem) as u16) << 8
    }

    fn read_byte(&self, cycles: &mut u32, address: u8, mem: &Memory) -> u8
    {
        let data = mem[address as u32];
        *cycles -= 1;
        data
    }

    fn lda_set_status(&mut self)
    {
        self.processor_status
            .set(ProcessorStatus::Zero, self[Register::A] == 0);
        self.processor_status
            .set(ProcessorStatus::Negative, (self[Register::A] & 1 << 7) > 0)
    }

    pub fn execute(&mut self, mut cycles: u32, mem: &mut Memory)
    {
        while cycles > 0 {
            let instruction = self.fetch8(&mut cycles, mem);
            match instruction {
                Self::INS_LDA_IM => {
                    let value = self.fetch8(&mut cycles, mem);
                    self[Register::A] = value;
                    self.lda_set_status();
                },
                Self::INS_LDA_ZP => {
                    let zero_page_addres = self.fetch8(&mut cycles, mem);
                    self[Register::A] = self.read_byte(&mut cycles, zero_page_addres, mem);
                    self.lda_set_status();
                },
                Self::INS_LDA_ZPX => {
                    let zero_page_addres = self.fetch8(&mut cycles, mem) + self[Register::X];
                    cycles -= 1;
                    self[Register::A] = self.read_byte(&mut cycles, zero_page_addres, mem);
                    self.lda_set_status();
                },
                Self::INS_JSR => {
                    let sub_addr = self.fetch16(&mut cycles, mem);
                    mem.write16(&mut cycles, self.program_counter - 1, self.stack_pointer as u32);
                    self.stack_pointer += 1;
                    self.program_counter = sub_addr;
                    cycles -= 1;
                },
                _ => println!("unknown instruction: {:#0X}", instruction),
            }
        }
    }
}

impl fmt::Debug for Cpu
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct(type_name::<Self>())
            .field("program_counter", &format!("{:#06X}", self.program_counter))
            .field("stack_pointer", &format!("{:#06X}", self.stack_pointer))
            .field(
                "registers",
                &self.registers.0.map(|r| format!("{:#04X}", r)),
            )
            .field(
                "processor_status",
                &format!("{:#010b}", self.processor_status.bits()),
            )
            .finish()
    }
}
