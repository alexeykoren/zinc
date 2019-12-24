//!
//! The static statement.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Static {
    pub location: Location,
    pub identifier: Identifier,
    pub r#type: Type,
    pub expression: Expression,
}

impl Static {
    pub fn new(
        location: Location,
        identifier: Identifier,
        r#type: Type,
        expression: Expression,
    ) -> Self {
        Self {
            location,
            identifier,
            r#type,
            expression,
        }
    }
}