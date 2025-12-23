//! Domain layer - core business logic
//!
//! # モジュールシステム (Go比較)
//!
//! Go: パッケージ = ディレクトリ
//! Rust: モジュール = ファイル or ディレクトリ
//!
//! ```text
//! Go:   internal/domain/errors/errors.go  → package errors
//! Rust: crates/domain/src/error.rs        → mod error
//! ```
//!
//! `pub mod` で公開、`pub use` で再エクスポート

pub mod error;

// 再エクスポート: 外部から `oxidize_domain::DomainError` でアクセス可能に
// Go でいう type alias のエクスポートに近い
pub use error::{DomainError, Result};
