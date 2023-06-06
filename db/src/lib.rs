//
// # Bootstrapping
//
// Enable the following extensions in the template database:
//
//      `psql -d template1 -c "CREATE EXTENSION fuzzystrmatch;"`
//      `psql -d template1 -c "CREATE EXTENSION postgis;"`
//
// Create/Recreate the database:
//      `sqlx database drop; sqlx database create; sqlx migrate run`

extern crate sqlx;

#[macro_use]
mod macros;

mod error;
mod json;
mod models;
mod pool;
mod validation;

pub mod prelude {
    pub use crate::json::{ToJson, ToOptionJson};
}
