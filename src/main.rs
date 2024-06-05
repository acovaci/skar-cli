mod cli;

#[tokio::main]
async fn main() {
    init_env();

    skar::core::log::init();

    match cli::run().await {
        Ok(_) => (),
        Err(e) => {
            log::error!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn init_env() {
    if cfg!(debug_assertions) {
        dotenv::dotenv().ok();
    }
}
