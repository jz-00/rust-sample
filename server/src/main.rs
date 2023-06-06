#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();
    server::run().await
}
