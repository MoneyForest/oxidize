# Go vs Rust 比較ガイド

rapid-go から oxidize への移植で使う対応表。

## 基本構文

### 変数定義

```go
// Go
var name string = "foo"
name := "foo"
```

```rust
// Rust
let name: String = String::from("foo");
let name = "foo".to_string();  // 型推論
let name = String::from("foo"); // 同じ
```

### 構造体

```go
// Go
type User struct {
    ID   string
    Name string
}

func NewUser(id, name string) *User {
    return &User{ID: id, Name: name}
}

func (u *User) FullName() string {
    return u.Name
}
```

```rust
// Rust
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    pub fn full_name(&self) -> &str {
        &self.name
    }
}
```

### インターフェース / トレイト

```go
// Go
type UserRepository interface {
    Get(ctx context.Context, id string) (*User, error)
    Create(ctx context.Context, user *User) error
}
```

```rust
// Rust
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get(&self, id: &str) -> Result<Option<User>>;
    async fn create(&self, user: &User) -> Result<()>;
}
```

### エラーハンドリング

```go
// Go
func FindUser(id string) (*User, error) {
    user, err := repo.Get(ctx, id)
    if err != nil {
        return nil, err
    }
    if user == nil {
        return nil, errors.UserNotFoundErr
    }
    return user, nil
}
```

```rust
// Rust
async fn find_user(&self, id: &str) -> Result<User> {
    let user = self.repo.get(id).await?;  // ? = if err return err
    user.ok_or_else(|| errors::user_not_found())
}
```

### Nullable / Optional

```go
// Go (using guregu/null)
type User struct {
    Name     string
    Nickname null.String  // nullable
}

if user.Nickname.Valid {
    println(user.Nickname.String)
}
```

```rust
// Rust
pub struct User {
    pub name: String,
    pub nickname: Option<String>,  // None or Some(String)
}

if let Some(nick) = &user.nickname {
    println!("{}", nick);
}
// or
user.nickname.as_ref().map(|n| println!("{}", n));
```

## 非同期処理

```go
// Go - goroutine
go func() {
    result := doSomething()
    ch <- result
}()
```

```rust
// Rust - tokio
tokio::spawn(async move {
    let result = do_something().await;
    tx.send(result).await;
});
```

## 依存性注入

```go
// Go
type UserInteractor struct {
    repo UserRepository
}

func NewUserInteractor(repo UserRepository) *UserInteractor {
    return &UserInteractor{repo: repo}
}
```

```rust
// Rust
pub struct UserInteractor {
    repo: Arc<dyn UserRepository>,  // Arc = 参照カウント付きスマートポインタ
}

impl UserInteractor {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}
```

## メモリ管理

| Go | Rust | 説明 |
|----|------|------|
| `*T` | `&T` | 不変参照（借用） |
| `*T` | `&mut T` | 可変参照（借用） |
| `new(T)` | `Box::new(T)` | ヒープ確保 |
| GC | 所有権システム | メモリ解放 |
| - | `Arc<T>` | 参照カウント（複数所有） |
| - | `Rc<T>` | 参照カウント（シングルスレッド） |

## よく使うクレート対応

| Go | Rust | 用途 |
|----|------|------|
| `context.Context` | `tokio::sync::*` | コンテキスト伝搬 |
| `go.uber.org/zap` | `tracing` | ログ |
| `github.com/spf13/cobra` | `clap` | CLI |
| `google.golang.org/grpc` | `tonic` | gRPC |
| `github.com/volatiletech/sqlboiler` | `sqlx` | DB |
| `github.com/go-playground/validator` | `validator` | バリデーション |
| `github.com/stretchr/testify` | 標準 + `mockall` | テスト |

## コンパイル時 vs ランタイム

**Go**: ランタイムでエラーが発覚することが多い
```go
user.Name = nil  // コンパイル通るけどnilポインタ参照でパニック
```

**Rust**: コンパイル時に多くのエラーを検出
```rust
user.name = None;  // コンパイルエラー（nameはOption<String>じゃない）
```
