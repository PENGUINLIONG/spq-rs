pub mod bin;
pub mod instr;

pub use bin::{SpirvBinary, SpirvHeader};
pub use instr::{Instr, Instrs, Instruction, InstructionBuilder, Operands};
