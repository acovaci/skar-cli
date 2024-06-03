mod cli;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    skar::core::log::init();

    match cli::run().await {
        Ok(_) => (),
        Err(e) => {
            log::error!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
