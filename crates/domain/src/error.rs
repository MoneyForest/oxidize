//! ドメインエラー定義
//!
//! # Go vs Rust エラーハンドリング
//!
//! ## Go
//! ```go
//! func FindUser(id string) (*User, error) {
//!     user, err := repo.Get(ctx, id)
//!     if err != nil {
//!         return nil, err  // エラーを返す
//!     }
//!     return user, nil
//! }
//! ```
//!
//! ## Rust
//! ```rust
//! fn find_user(&self, id: &str) -> Result<User> {
//!     let user = self.repo.get(id)?;  // ? でエラー時は早期リターン
//!     Ok(user)
//! }
//! ```
//!
//! `?` 演算子 = `if err != nil { return err }` の1文字版

use thiserror::Error;

/// エラーカテゴリ（HTTP/gRPCステータスコードにマップ）
///
/// Go の ErrorCategory と同等
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    BadRequest,      // 400
    Unauthorized,    // 401
    Forbidden,       // 403
    NotFound,        // 404
    Conflict,        // 409
    Internal,        // 500
}

/// ドメインエラー
///
/// # thiserror クレート
///
/// `#[derive(Error)]` マクロで `std::error::Error` トレイトを自動実装
/// `#[error("...")]` でエラーメッセージのフォーマットを指定
///
/// Go だと:
/// ```go
/// type DomainError struct {
///     Code     string
///     Category ErrorCategory
///     Message  string
/// }
/// func (e *DomainError) Error() string { return e.Message }
/// ```
#[derive(Error, Debug)]
#[error("{message}")]  // Display トレイトの実装（エラーメッセージ）
pub struct DomainError {
    pub code: &'static str,      // 'static = プログラム全体で有効なライフタイム
    pub category: ErrorCategory,
    pub message: String,
}

impl DomainError {
    /// コンストラクタ
    ///
    /// `impl Into<String>` = String に変換できる任意の型を受け付ける
    /// Go でいう interface{} だが、コンパイル時に型チェックされる
    pub fn new(code: &'static str, category: ErrorCategory, message: impl Into<String>) -> Self {
        Self {
            code,
            category,
            message: message.into(),
        }
    }

    // ファクトリメソッド群
    // Go の NewBadRequestError, NewNotFoundError 等に相当

    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(code, ErrorCategory::BadRequest, message)
    }

    pub fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(code, ErrorCategory::NotFound, message)
    }

    pub fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(code, ErrorCategory::Internal, message)
    }
}

/// Result 型エイリアス
///
/// Go: `func Foo() (*T, error)`
/// Rust: `fn foo() -> Result<T>` (= `Result<T, DomainError>`)
///
/// 毎回 `Result<T, DomainError>` と書くのは冗長なのでエイリアス化
pub type Result<T> = std::result::Result<T, DomainError>;

/// 事前定義エラー
///
/// Go の `var TenantNotFoundErr = NewNotFoundError(...)` に相当
/// 関数にしているのは、毎回新しいインスタンスを返すため
/// （Go だとグローバル変数だが、Rust では関数が一般的）
pub mod errors {
    use super::*;

    pub fn internal() -> DomainError {
        DomainError::internal("E100001", "Internal error")
    }

    pub fn invalid_argument() -> DomainError {
        DomainError::bad_request("E100002", "Invalid argument")
    }

    pub fn tenant_not_found() -> DomainError {
        DomainError::not_found("E200101", "Tenant not found")
    }

    pub fn staff_not_found() -> DomainError {
        DomainError::not_found("E200201", "Staff not found")
    }
}

// ============================================================
// テスト
// ============================================================

#[cfg(test)]  // テスト時のみコンパイル
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = errors::tenant_not_found();
        assert_eq!(err.code, "E200101");
        assert_eq!(err.category, ErrorCategory::NotFound);
    }

    #[test]
    fn test_result_with_question_mark() {
        // ? 演算子のテスト
        fn may_fail(should_fail: bool) -> Result<String> {
            if should_fail {
                return Err(errors::internal());
            }
            Ok("success".to_string())
        }

        fn caller() -> Result<String> {
            let result = may_fail(false)?;  // Ok なので result に値が入る
            Ok(result)
        }

        assert!(caller().is_ok());
    }
}
