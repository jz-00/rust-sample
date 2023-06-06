pub struct Context {
    pub db_pool: db::Pool,
    pub user: auth::AuthenticatedUser,
}

impl juniper::Context for Context {}

#[derive(Clone)]
pub struct ContextManager {
    db_pool: db::Pool,
}

impl ContextManager {
    pub async fn new() -> ContextManager {
        ContextManager {
            db_pool: db::create_default_pool().await,
        }
    }

    pub fn create_user_context(&self, user: auth::AuthenticatedUser) -> Context {
        Context {
            db_pool: self.db_pool.clone(),
            user,
        }
    }
}
