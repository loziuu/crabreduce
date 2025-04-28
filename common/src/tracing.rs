pub fn init() {
    tracing_subscriber::fmt()
        // Only during development?
        .with_max_level(tracing::Level::INFO)
        .compact()
        .with_target(false)
        .with_thread_ids(true)
        .init();
}
