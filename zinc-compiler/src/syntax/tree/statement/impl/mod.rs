//!
//! The impl statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
    pub statements: Vec<ImplementationLocalStatement>,
}

impl Statement {
    pub fn new(
        location: Location,
        identifier: Identifier,
        statements: Vec<ImplementationLocalStatement>,
    ) -> Self {
        Self {
            location,
            identifier,
            statements,
        }
    }
}
