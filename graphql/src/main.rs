fn main() {
    println!("{}", graphql::schema::create().as_schema_language());
}
