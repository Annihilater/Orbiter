# Orbiter Backend

基于 Actix-web 的 Rust Web 后端项目，实现了用户认证和个人中心功能。

## 功能特性

- 用户注册
- 用户登录
- 个人中心
- JWT 认证
- 数据库集成

## 技术栈

- Rust
- Actix-web
- PostgreSQL
- SQLx
- JWT

## 环境要求

- Rust 1.75+
- PostgreSQL 12+
- SQLx CLI

## 项目设置

1. 克隆项目：

```bash
git clone https://github.com/yourusername/orbiter-backend.git
cd orbiter-backend
```

2. 设置环境变量：

创建 `.env` 文件并添加以下内容：

```
DATABASE_URL=postgres://postgres:postgres@localhost/orbiter
JWT_SECRET=your_jwt_secret_key
RUST_LOG=debug
```

3. 创建数据库：

```bash
createdb orbiter
```

4. 运行数据库迁移：

```bash
sqlx database create
sqlx migrate run
```

5. 运行项目：

```bash
cargo run
```

## API 接口

### 认证接口

#### 注册用户

```
POST /api/auth/register
Content-Type: application/json

{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
}
```

#### 用户登录

```
POST /api/auth/login
Content-Type: application/json

{
    "username": "testuser",
    "password": "password123"
}
```

### 用户接口

#### 获取个人信息

```
GET /api/users/me
Authorization: Bearer your_jwt_token
```

## 开发说明

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 进行代码检查
- 使用 `cargo test` 运行测试 