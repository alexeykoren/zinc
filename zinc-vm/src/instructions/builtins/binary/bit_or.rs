use crate::core::{VMInstruction, VirtualMachine};
use crate::{Engine, Result};

use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::BitOr;

impl<E, CS> VMInstruction<E, CS> for BitOr
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result {
        unimplemented!()
    }
}
