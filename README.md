# Oxidize

Rust API boilerplate with Clean Architecture + DDD.

**Go版**: [rapid-go](https://github.com/abyssparanoia/rapid-go) の Rust移植

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                      Usecase Layer                            │
│  (Interactors - orchestrate domain logic)                    │
└──────────────────────────┬───────────────────────────────────┘
                           │
┌──────────────────────────▼───────────────────────────────────┐
│                      Domain Layer                             │
│  (Entities, Value Objects, Repository traits, Services)      │
└──────────────────────────┬───────────────────────────────────┘
                           │
┌──────────────────────────▼───────────────────────────────────┐
│                   Infrastructure Layer                        │
│  (DB, External APIs, gRPC/HTTP handlers, CLI)                │
└──────────────────────────────────────────────────────────────┘
```

## Layer Responsibilities

### Domain Layer (`crates/domain`)

ビジネスロジックの中核。**外部に一切依存しない**。

| ディレクトリ | 内容 | 配置するもの |
|-------------|------|-------------|
| `model/` | Entity, Value Object | `User`, `Tenant`, `Email`, `UserId` |
| `repository/` | Repository trait | `trait UserRepository` (インターフェースのみ) |
| `service/` | Domain Service | 複数エンティティにまたがるビジネスロジック |
| `error.rs` | Domain Error | `DomainError`, ビジネスルール違反エラー |

**判断基準**: 「DBやAPIが変わっても変更不要か？」→ Yes なら Domain

### Usecase Layer (`crates/usecase`)

アプリケーション固有のビジネスルール。ユースケースを実現する。

| ディレクトリ | 内容 | 配置するもの |
|-------------|------|-------------|
| `interactor/` | Interactor | `CreateUserInteractor`, `GetTenantInteractor` |
| `input/` | Input DTO | Interactor の入力データ構造 |
| `output/` | Output DTO | Interactor の出力データ構造 |

**判断基準**: 「ユーザー操作やAPIコールの単位か？」→ Yes なら Usecase

### Infrastructure Layer (`crates/infrastructure`)

外部システムとの接続。Domain の trait を実装する。

| ディレクトリ | 内容 | 配置するもの |
|-------------|------|-------------|
| `database/` | Repository 実装 | `UserRepositoryImpl` (sqlx) |
| `grpc/` | gRPC handlers | tonic サーバー実装 |
| `http/` | HTTP handlers | axum ルーター |
| `external/` | 外部API client | Firebase, AWS SDK |
| `config/` | 環境設定 | 環境変数読み込み |
| `cli/` | CLI | clap コマンド |

**判断基準**: 「DBやフレームワーク固有のコードか？」→ Yes なら Infrastructure

## Dependency Rule

```
Infrastructure → Usecase → Domain
     ↓              ↓         ↓
   具体実装      オーケストレータ   純粋なビジネスロジック
```

- **Domain** は何にも依存しない
- **Usecase** は Domain のみに依存
- **Infrastructure** は Domain と Usecase に依存

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
│   ├── domain/           # Domain layer (white-box)
│   │   ├── src/
│   │   │   ├── model/        # Entity, Value Object
│   │   │   ├── repository/   # Repository traits (interface)
│   │   │   ├── service/      # Domain services
│   │   │   ├── error.rs      # Domain errors
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── usecase/          # Usecase layer
│   │   ├── src/
│   │   │   ├── interactor/   # Interactor implementations
│   │   │   ├── input/        # Input DTOs
│   │   │   ├── output/       # Output DTOs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── infrastructure/   # Infrastructure layer
│   │   ├── src/
│   │   │   ├── database/     # Repository implementations (sqlx)
│   │   │   ├── grpc/         # gRPC handlers (tonic)
│   │   │   ├── http/         # HTTP handlers (axum)
│   │   │   ├── cmd/          # CLI commands (clap)
│   │   │   ├── environment/  # Environment config
│   │   │   ├── otel/         # OpenTelemetry setup
│   │   │   ├── registry.rs   # Centralized DI container
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   └── app/              # Application entry point
│       ├── src/
│       │   └── main.rs       # Bootstrap & server startup
│       └── Cargo.toml
│
├── schema/
│   └── proto/            # Protocol Buffer definitions
├── db/
│   └── migrations/       # SQL migrations
└── docker/               # Dockerfiles
```

## Where Should I Put This Code?

| コードの種類 | 配置先 | 例 |
|-------------|--------|-----|
| ビジネスエンティティ | `domain/model/` | `User`, `Tenant` |
| ID型, Email型 | `domain/model/` | `UserId`, `Email` (Value Object) |
| Repository インターフェース | `domain/repository/` | `trait UserRepository` |
| 複数エンティティのロジック | `domain/service/` | `AuthorizationService` |
| ユースケース実装 | `usecase/interactor/` | `CreateUserInteractor` |
| API入出力の型 | `usecase/input/`, `output/` | `CreateUserInput` |
| DB操作の実装 | `infrastructure/database/` | `UserRepositoryImpl` |
| gRPCハンドラ | `infrastructure/grpc/` | `UserServiceServer` |
| HTTPハンドラ | `infrastructure/http/` | `create_user_handler` |
| 環境変数読み込み | `infrastructure/environment/` | `Environment` |
| CLIコマンド | `infrastructure/cmd/` | `Cli`, `Commands` |
| DI設定 | `infrastructure/registry.rs` | `Registry` (Interactor組み立て) |

## Rust Learning Points

このプロジェクトで学べるRustの概念:

1. **所有権 & 借用** - GoのポインタとRustの参照の違い
2. **Result & Option** - Goの`(T, error)`との違い
3. **トレイト** - Goのinterfaceとの違い
4. **非同期処理** - tokio, async/await
5. **ライフタイム** - 参照の有効期間
6. **マクロ** - derive, proc-macro

## Development Flow

新しいエンティティ（例: `Order`）を追加する場合の開発フロー:

### 1. Domain Layer

```bash
# model定義
crates/domain/src/model/order.rs
crates/domain/src/model/mod.rs  # pub mod order; を追加

# repositoryトレイト定義
crates/domain/src/repository/order.rs
crates/domain/src/repository/mod.rs  # pub mod order; を追加
```

### 2. Usecase Layer

```bash
# input/output定義
crates/usecase/src/input/order.rs
crates/usecase/src/output/order.rs

# interactor実装
crates/usecase/src/interactor/order.rs

# mod.rsに追加
crates/usecase/src/input/mod.rs
crates/usecase/src/output/mod.rs
crates/usecase/src/interactor/mod.rs
crates/usecase/src/lib.rs  # re-export
```

### 3. Infrastructure Layer

```bash
# repository実装
crates/infrastructure/src/database/order.rs
crates/infrastructure/src/database/mod.rs

# gRPCサービス (必要な場合)
schema/proto/order.proto
crates/infrastructure/src/grpc/order_service.rs
crates/infrastructure/src/grpc/mod.rs
crates/infrastructure/src/grpc/server.rs  # サービス追加

# HTTPハンドラ (必要な場合)
crates/infrastructure/src/http/handlers.rs  # ハンドラ追加
crates/infrastructure/src/http/router.rs    # ルート追加
```

### 4. DI設定

```bash
# RegistryにInteractorを追加
crates/infrastructure/src/registry.rs
```

### 5. マイグレーション

```bash
# SQLマイグレーション作成
db/migrations/YYYYMMDDHHMMSS_create_orders.sql
```

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
