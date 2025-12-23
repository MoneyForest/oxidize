# Oxidize

Rust API boilerplate with Clean Architecture.

**Go版**: [rapid-go](https://github.com/abyssparanoia/rapid-go) の Rust移植

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Presentation Layer                        │
│  (gRPC handlers, HTTP handlers, CLI)                        │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                     Usecase Layer                            │
│  (Interactors - orchestrate domain logic)                   │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                     Domain Layer                             │
│  (Entities, Repository traits, Domain services)             │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                  Infrastructure Layer                        │
│  (DB, External APIs, gRPC server implementation)            │
└─────────────────────────────────────────────────────────────┘
```

## Tech Stack

| Category | Go (rapid-go) | Rust (oxidize) |
|----------|---------------|----------------|
| gRPC | grpc-go | tonic |
| HTTP | grpc-gateway | axum |
| ORM | sqlboiler | sqlx |
| Auth | Firebase/Cognito | jsonwebtoken |
| Logging | zap | tracing |
| CLI | cobra | clap |
| DI | Manual | Manual |

## Project Structure

```
oxidize/
├── crates/
│   ├── domain/          # Domain layer
│   │   ├── model/       # Entities
│   │   ├── repository/  # Repository traits
│   │   ├── service/     # Domain services
│   │   └── error.rs     # Domain errors
│   │
│   ├── usecase/         # Usecase layer
│   │   ├── input/       # Input DTOs
│   │   ├── output/      # Output DTOs
│   │   └── interactor/  # Interactor traits & impl
│   │
│   ├── infrastructure/  # Infrastructure layer
│   │   ├── database/    # sqlx implementations
│   │   ├── grpc/        # tonic handlers
│   │   └── config/      # Environment config
│   │
│   └── app/             # Application entry point
│       └── main.rs      # CLI (clap)
│
├── schema/
│   └── proto/           # Protocol Buffer definitions
│
├── db/
│   └── mysql/
│       └── migrations/  # SQL migrations
│
└── docker/              # Dockerfiles
```

## Rust Learning Points

このプロジェクトで学べるRustの概念:

1. **所有権 & 借用** - GoのポインタとRustの参照の違い
2. **Result & Option** - Goの`(T, error)`との違い
3. **トレイト** - Goのinterfaceとの違い
4. **非同期処理** - tokio, async/await
5. **ライフタイム** - 参照の有効期間
6. **マクロ** - derive, proc-macro

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run server
cargo run -- http-server run

# Format
cargo fmt

# Lint
cargo clippy
```

## PRs (Learning Steps)

| PR | Content | Rust Concepts |
|----|---------|---------------|
| #1 | Project setup | Cargo workspace, modules |
| #2 | Domain models | struct, enum, impl |
| #3 | Error handling | Result, thiserror, ? operator |
| #4 | Repository traits | trait, async_trait |
| #5 | Usecase layer | generics, Arc |
| #6 | Database (sqlx) | async, connection pool |
| #7 | gRPC (tonic) | protobuf, streaming |
| #8 | CLI & HTTP | clap, axum |
