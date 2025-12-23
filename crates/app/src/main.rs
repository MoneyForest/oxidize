use clap::Parser;
use oxidize_infrastructure::{otel, run_grpc_server, run_http_server, Cli, Commands, Environment};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let env = Environment::from_env();
    let _provider = otel::init(&env)?;

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::HttpServer { port } => run_http_server(port, &env.database_url).await,
        Commands::GrpcServer { port } => run_grpc_server(port, &env.database_url).await,
        Commands::Migrate => {
            tracing::info!("Running migrations...");
            let pool = oxidize_infrastructure::create_pool(&env.database_url).await?;
            sqlx::migrate!("../../migrations").run(&pool).await?;
            tracing::info!("Migrations completed");
            Ok(())
        }
    };

    otel::shutdown(_provider);
    result
}
