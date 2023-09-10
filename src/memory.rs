use std::ops;

pub struct Memory
{
    data: [u8; Self::MAX_MEM],
}

impl Memory
{
    pub const MAX_MEM: usize = 1024 * 64;

    pub const fn new() -> Self
    {
        Self {
            data: [0; Self::MAX_MEM],
        }
    }

    pub fn write16(&mut self, cycles: &mut u32, value: u16, addr: u32)
    {
        self.data[addr as usize] = (value & 0xFF) as u8;
        self.data[addr as usize + 1] = (value >> 8) as u8;
        *cycles -= 1;
    }
}

impl ops::Index<u32> for Memory
{
    type Output = u8;

    fn index(&self, index: u32) -> &Self::Output
    {
        &self.data[index as usize]
    }
}

impl ops::IndexMut<u32> for Memory
{
    fn index_mut(&mut self, index: u32) -> &mut Self::Output
    {
        &mut self.data[index as usize]
    }
}
