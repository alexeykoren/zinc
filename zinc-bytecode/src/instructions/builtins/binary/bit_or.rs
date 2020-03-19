use crate::{Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct BitOr;

impl InstructionInfo for BitOr {
    fn to_assembly(&self) -> String {
        "bit_or".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::BitOr
    }

    fn wrap(&self) -> Instruction {
        Instruction::BitOr(self.clone())
    }
}
