pub fn init() {
    let homedir = home::home_dir().unwrap();

    let config = log4rs::Config::builder()
        .appender(log4rs::config::Appender::builder().build(
            "stdout",
            Box::new(log4rs::append::console::ConsoleAppender::builder().build()),
        ))
        .appender(
            log4rs::config::Appender::builder().build(
                "file",
                Box::new(
                    log4rs::append::file::FileAppender::builder()
                        .build(homedir.join(".skar").join("skar.log"))
                        .unwrap(),
                ),
            ),
        )
        .build(
            log4rs::config::Root::builder()
                .appender("stdout")
                .appender("file")
                .build(get_default_log_level()),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn get_default_log_level() -> log::LevelFilter {
    if cfg!(debug_assertions) {
        return log::LevelFilter::Debug;
    }

    match std::env::var("RUST_LOG") {
        Ok(level) => level.parse().unwrap(),
        Err(_) => log::LevelFilter::Info,
    }
}
