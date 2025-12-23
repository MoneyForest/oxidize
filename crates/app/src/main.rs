mod cli;
mod config;
mod http;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::HttpServer { port } => {
            http::run_server(port).await?;
        }
        Commands::Migrate => {
            tracing::info!("Running migrations...");
            let config = config::Config::from_env();
            let pool = oxidize_infrastructure::create_pool(&config.database_url).await?;
            sqlx::migrate!("../../migrations").run(&pool).await?;
            tracing::info!("Migrations completed");
        }
    }

    Ok(())
}
