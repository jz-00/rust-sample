mod context;
mod image;
mod mutation;
mod query;
mod user;

use juniper::{EmptySubscription, RootNode};

pub use context::{Context, ContextManager};
use mutation::Mutation;
use query::Query;

pub mod schema {
    use super::{EmptySubscription, Mutation, Query, RootNode};
    use crate::context::Context;

    pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

    pub fn create() -> Schema {
        Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
    }
}

pub use schema::Schema;

// pub async fn test(context: Context, value: Option<&str>) -> anyhow::Result<serde_json::Value> {
//     db::test(&context.db_pool, value).await
// }
