use crate::{Operator, RuntimeError, Stack, Bytecode};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use bellman::pairing::Engine;
use ff::Field;
use crate::stack::Primitive;

/// Removes two elements from the stack and pushes their sum.
pub struct Add;

impl<E, CS> Operator<E, CS> for Add where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>,
        _bytecode: &mut Bytecode)
        -> Result<(), RuntimeError>
    {
        let left = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let right = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let mut sum = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                let mut sum = a;
                sum.add_assign(&b);
                Some(sum)
            }
            _ => None
        };

        let sum_var = cs.alloc(
            || "sum",
            || sum.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + left.variable + right.variable,
            |lc| lc + CS::one(),
            |lc| lc + sum_var
        );

        stack.push(Primitive { value: sum, variable: sum_var });

        Ok(())
    }
}
