mod cli;
mod config;
mod grpc;
mod http;
mod telemetry;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = config::Config::from_env();
    let _provider = telemetry::init_telemetry(&config)?;

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::HttpServer { port } => http::run_server(port).await,
        Commands::GrpcServer { port } => grpc::run_grpc_server(port).await,
        Commands::Migrate => {
            tracing::info!("Running migrations...");
            let pool = oxidize_infrastructure::create_pool(&config.database_url).await?;
            sqlx::migrate!("../../migrations").run(&pool).await?;
            tracing::info!("Migrations completed");
            Ok(())
        }
    };

    telemetry::shutdown_telemetry(_provider);
    result
}
