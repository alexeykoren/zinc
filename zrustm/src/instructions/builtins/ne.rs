extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Ne;

impl<E, O> VMInstruction<E, O> for Ne
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.memory()?.pop()?;
        let right = vm.memory()?.pop()?;

        let ne = vm.get_operator().ne(left, right)?;

        vm.memory()?.push(ne)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_ne() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(Ne)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 2.into() })
            .add(Ne)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 1.into() })
            .add(Ne)
            .test(&[1, 0, 1])
    }
}
