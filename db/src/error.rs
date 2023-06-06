#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("missing required argument: {}", .0)]
    RequiredArgument(&'static str),

    #[error("{} depends on {}", .name, .depends_on)]
    DependentArgument {
        name: &'static str,
        depends_on: &'static str,
    },

    #[error("invalid value: {}: '{}'", .name, .value)]
    InvalidValue { name: &'static str, value: String },

    #[error("entity not found: {}:{} ", .table, .id)]
    NotFound {
        table: &'static str,
        id: &'static str,
    },
}
