use tracing::info;

/// Read environment variables
pub fn config(package_name: &str) -> config::Config {
    // Load global .env file
    dotenvy::dotenv().ok();

    // Build configuration
    let config = config::Config::builder()
        .add_source(config::Environment::with_prefix(package_name))
        .add_source(config::Environment::with_prefix("AGENT_CONFIG"))
        .build()
        .unwrap();

    info!("{:?}", config);

    config
}