use clap::Parser;
use oxidize_infrastructure::{
    otel, run_grpc_server, run_http_server, Cli, Commands, Environment, Registry,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let env = Environment::from_env();
    let _provider = otel::init(&env)?;

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::HttpServer { port } => {
            let registry = Registry::new(&env.database_url).await?;
            run_http_server(port, registry).await
        }
        Commands::GrpcServer { port } => {
            let registry = Registry::new(&env.database_url).await?;
            run_grpc_server(port, registry).await
        }
        Commands::Migrate => {
            tracing::info!("Running migrations...");
            let pool = oxidize_infrastructure::create_pool(&env.database_url).await?;
            sqlx::migrate!("../../db/migrations").run(&pool).await?;
            tracing::info!("Migrations completed");
            Ok(())
        }
    };

    otel::shutdown(_provider);
    result
}
