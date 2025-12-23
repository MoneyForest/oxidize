use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oxidize")]
#[command(about = "Rust API boilerplate with Clean Architecture")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run HTTP server
    HttpServer {
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Run gRPC server
    GrpcServer {
        #[arg(short, long, default_value = "50051")]
        port: u16,
    },
    /// Run database migrations
    Migrate,
}
