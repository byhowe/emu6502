use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register
{
    A = 0,
    X = 1,
    Y = 2,
}

#[derive(Debug, Clone, Copy)]
pub struct Registers(pub [u8; 3]);

impl Registers
{
    pub const fn new() -> Self
    {
        Self([0; 3])
    }
}

impl ops::Index<Register> for Registers
{
    type Output = u8;

    fn index(&self, index: Register) -> &Self::Output
    {
        &self.0[index as usize]
    }
}

impl ops::IndexMut<Register> for Registers
{
    fn index_mut(&mut self, index: Register) -> &mut Self::Output
    {
        &mut self.0[index as usize]
    }
}
