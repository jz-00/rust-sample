use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::user::{UserAccount, UserId, UserProfile};

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn current_user(context: &Context) -> FieldResult<Option<UserAccount>> {
        let id = UserId(context.user.id.to_string().into());
        Ok(Some(UserAccount {
            id: id.clone(),
            profile: UserProfile::new(id, "A User"),
        }))
    }
}
