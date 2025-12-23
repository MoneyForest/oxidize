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
│   │   │   ├── otel/         # OpenTelemetry setup
│   │   │   ├── registry.rs   # DI Registry
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   └── app/              # Application entry point
│       ├── src/
│       │   └── main.rs       # Bootstrap
│       └── Cargo.toml
│
├── schema/
│   └── proto/            # Protocol Buffer definitions
├── db/
│   └── migrations/       # SQL migrations
├── docker/
│   └── Dockerfile        # Multi-stage build
└── docker-compose.yml
```

## Where Should I Put This Code?

| コードの種類 | 配置先 | 例 |
|-------------|--------|-----|
| ビジネスエンティティ | `domain/model/` | `Tenant`, `Staff` |
| ID型, Value Object | `domain/model/` | `TenantId`, `StaffRole` |
| Repository インターフェース | `domain/repository/` | `trait TenantRepository` |
| ユースケース実装 | `usecase/interactor/` | `TenantInteractor` |
| API入出力の型 | `usecase/input/`, `output/` | `CreateTenantInput` |
| DB操作の実装 | `infrastructure/database/` | `TenantRepositoryImpl` |
| DI登録 | `infrastructure/registry.rs` | `Registry` |
| gRPCサービス | `infrastructure/grpc/` | `TenantServiceImpl` |
| HTTPハンドラ | `infrastructure/http/` | `list_tenants` |
| Proto定義 | `schema/proto/` | `tenant.proto` |
| DBマイグレーション | `db/migrations/` | `create_tenants.sql` |

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

# Run HTTP server
cargo run -- http-server --port 8080

# Run gRPC server
cargo run -- grpc-server --port 50051

# Run migrations
cargo run -- migrate

# Format
cargo fmt

# Lint
cargo clippy
```

## Development Flow

新機能を追加する際の開発フロー:

### 1. Domain Layer

```rust
// 1-1. Model定義 (crates/domain/src/model/order.rs)
pub struct Order { ... }
pub struct OrderId(String);

// 1-2. Repository trait定義 (crates/domain/src/repository/order.rs)
#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn get(&self, id: &OrderId) -> Result<Option<Order>>;
    async fn create(&self, order: &Order) -> Result<()>;
}
```

### 2. Usecase Layer

```rust
// 2-1. Input/Output定義 (crates/usecase/src/input/order.rs)
pub struct CreateOrderInput { ... }

// 2-2. Interactor実装 (crates/usecase/src/interactor/order.rs)
pub struct OrderInteractor<R: OrderRepository> {
    repository: Arc<R>,
}
```

### 3. Infrastructure Layer

```rust
// 3-1. Repository実装 (crates/infrastructure/src/database/order.rs)
pub struct OrderRepositoryImpl { pool: PgPool }

impl OrderRepository for OrderRepositoryImpl { ... }

// 3-2. Registryに登録 (crates/infrastructure/src/registry.rs)
pub struct Registry {
    pub order_interactor: OrderInteractor<OrderRepositoryImpl>,  // 追加
}

// 3-3. HTTP/gRPCハンドラ追加
// - crates/infrastructure/src/http/handlers.rs
// - crates/infrastructure/src/grpc/order_service.rs
```

### 4. Proto定義 (gRPCの場合)

```protobuf
// schema/proto/order/order.proto
service OrderService {
    rpc CreateOrder(CreateOrderRequest) returns (CreateOrderResponse);
}
```

### Summary

```
Domain (white-box)     → Usecase              → Infrastructure
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. Model定義           → 2. Input/Output      → 3. Repository実装
   Repository trait       Interactor             Registry登録
                                                 HTTP/gRPC handlers
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
