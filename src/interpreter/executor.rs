//!
//! The executor.
//!

use num_bigint::BigInt;
use num_traits::Num;
use num_traits::One;
use num_traits::Zero;

use crate::interpreter::Error;
use crate::interpreter::Field;
use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::lexical::Literal;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Type;

pub struct Executor {
    stack: Vec<Field>,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            stack: Vec::with_capacity(1024),
        }
    }
}

impl Executor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&mut self, expression: Expression) -> Result<Field, Error> {
        for element in expression.elements.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => self.stack.push(match operand {
                    ExpressionOperand::Literal(Literal::Boolean(BooleanLiteral::False)) => {
                        Field::new(BigInt::zero(), Type::Bool)
                    }
                    ExpressionOperand::Literal(Literal::Boolean(BooleanLiteral::True)) => {
                        Field::new(BigInt::one(), Type::Bool)
                    }
                    ExpressionOperand::Literal(Literal::Integer(IntegerLiteral::Decimal {
                        value,
                    })) => {
                        let value = BigInt::from_str_radix(value.as_str(), 10)
                            .expect("Integer literal parsing bug");
                        Field::new(value, Type::Field)
                    }
                    ExpressionOperand::Literal(Literal::Integer(IntegerLiteral::Hexadecimal {
                        value,
                    })) => {
                        let value = BigInt::from_str_radix(value.as_str(), 16)
                            .expect("Integer literal parsing bug");
                        Field::new(value, Type::Field)
                    }
                    ExpressionOperand::Identifier(_identifier) => unimplemented!(),
                    ExpressionOperand::Type(r#_type) => unimplemented!(),
                }),
                ExpressionObject::Operator(operator @ ExpressionOperator::Addition) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");

                    if !operand_1.value_type.can_be_first_operand(operator) {
                        return Err(Error::first_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_1,
                        ));
                    }
                    if !operand_2.value_type.can_be_second_operand(operator) {
                        return Err(Error::second_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_2,
                        ));
                    }
                    if operand_1.value_type != operand_2.value_type {
                        return Err(Error::operand_type_mismatch(
                            element.token.location,
                            operand_2.value_type,
                            operand_1.value_type,
                        ));
                    }

                    let result = operand_1.value + operand_2.value;
                    self.stack.push(Field::new(result, Type::Field));
                }
                ExpressionObject::Operator(operator @ ExpressionOperator::Subtraction) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");

                    if !operand_1.value_type.can_be_first_operand(operator) {
                        return Err(Error::first_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_1,
                        ));
                    }
                    if !operand_2.value_type.can_be_second_operand(operator) {
                        return Err(Error::second_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_2,
                        ));
                    }
                    if operand_1.value_type != operand_2.value_type {
                        return Err(Error::operand_type_mismatch(
                            element.token.location,
                            operand_2.value_type,
                            operand_1.value_type,
                        ));
                    }

                    let result = operand_1.value - operand_2.value;
                    self.stack.push(Field::new(result, Type::Field));
                }
                ExpressionObject::Operator(operator @ ExpressionOperator::Multiplication) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");

                    if !operand_1.value_type.can_be_first_operand(operator) {
                        return Err(Error::first_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_1,
                        ));
                    }
                    if !operand_2.value_type.can_be_second_operand(operator) {
                        return Err(Error::second_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_2,
                        ));
                    }
                    if operand_1.value_type != operand_2.value_type {
                        return Err(Error::operand_type_mismatch(
                            element.token.location,
                            operand_2.value_type,
                            operand_1.value_type,
                        ));
                    }

                    let result = operand_1.value * operand_2.value;
                    self.stack.push(Field::new(result, Type::Field));
                }
                ExpressionObject::Operator(operator @ ExpressionOperator::Division) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");

                    if !operand_1.value_type.can_be_first_operand(operator) {
                        return Err(Error::first_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_1,
                        ));
                    }
                    if !operand_2.value_type.can_be_second_operand(operator) {
                        return Err(Error::second_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_2,
                        ));
                    }
                    if operand_1.value_type != operand_2.value_type {
                        return Err(Error::operand_type_mismatch(
                            element.token.location,
                            operand_2.value_type,
                            operand_1.value_type,
                        ));
                    }

                    let result = operand_1.value / operand_2.value;
                    self.stack.push(Field::new(result, Type::Field));
                }
                ExpressionObject::Operator(operator @ ExpressionOperator::Remainder) => {
                    let operand_2 = self.stack.pop().expect("Stack bug");
                    let operand_1 = self.stack.pop().expect("Stack bug");

                    if !operand_1.value_type.can_be_first_operand(operator) {
                        return Err(Error::first_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_1,
                        ));
                    }
                    if !operand_2.value_type.can_be_second_operand(operator) {
                        return Err(Error::second_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand_2,
                        ));
                    }
                    if operand_1.value_type != operand_2.value_type {
                        return Err(Error::operand_type_mismatch(
                            element.token.location,
                            operand_2.value_type,
                            operand_1.value_type,
                        ));
                    }

                    let result = operand_1.value % operand_2.value;
                    self.stack.push(Field::new(result, Type::Field));
                }
                ExpressionObject::Operator(operator @ ExpressionOperator::Negation) => {
                    let operand = self.stack.pop().expect("Stack bug");

                    if !operand.value_type.can_be_first_operand(operator) {
                        return Err(Error::first_operand_operator_not_available(
                            element.token.location,
                            operator,
                            operand,
                        ));
                    }

                    let result = -operand.value;
                    self.stack.push(Field::new(result, Type::Field));
                }
                _ => unimplemented!(),
            }
        }

        Ok(self.stack.pop().expect("Stack bug"))
    }
}
