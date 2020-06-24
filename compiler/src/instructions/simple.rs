use super::*;

pub struct CliInstruction;
impl Instruction for CliInstruction {
    fn byte_len(&self) -> u16 { 1 }
    fn compile(&self, _: &HashMap::<String, u16>) -> Vec<u8> {
        vec![0xFA]
    }
}

pub struct HltInstruction;
impl Instruction for HltInstruction {
    fn byte_len(&self) -> u16 { 1 }
    fn compile(&self, _: &HashMap::<String, u16>) -> Vec<u8> {
        vec![0xF4]
    }
}

// Only handles the no-operand scenario
pub struct LodsbInstruction;
impl Instruction for LodsbInstruction {
    fn byte_len(&self) -> u16 { 1 }
    fn compile(&self, _: &HashMap::<String, u16>) -> Vec<u8> {
        vec![0xAC]
    }
}
