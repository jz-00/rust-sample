use juniper::{graphql_object, FieldResult};

use crate::context::Context;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    /// This is a placeholder.
    fn placeholder(_context: &Context) -> FieldResult<bool> {
        Ok(true)
    }
}
