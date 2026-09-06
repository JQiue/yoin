# 测试指南

## 测试目录结构

```plain
tests/
├── unit/                    # 单元测试
│   ├── common/             # 共享测试工具
│   ├── entity/             # 实体测试
│   ├── handler/            # 处理器测试
│   ├── service/            # 服务层测试
│   └── mod.rs              # 模块导出
├── integration/            # 集成测试
│   ├── mod.rs              # 模块导出
│   ├── auth.rs            # 认证集成测试
│   ├── comment.rs          # 评论集成测试
│   ├── site.rs             # 站点集成测试
│   └── user.rs             # 用户集成测试
└── ...
```

## 运行测试

### 运行所有测试

```bash
cargo test
```

### 只运行单元测试

```bash
cargo test --lib
```

### 只运行集成测试

```bash
cargo test --test integration
```

### 运行特定测试

```bash
# 运行特定模块的测试
cargo test error
cargo test auth

# 运行特定文件
cargo test -- unit::service::error_tests
```

### 运行并生成覆盖率报告

```bash
# 安装 tarpaulin
cargo install tarpaulin

# 运行覆盖率测试
cargo tarpaulin --out Html
```

## 测试最佳实践

### 1. 单元测试 (Unit Tests)

- 测试单个函数或方法的行为
- 不依赖外部服务（数据库、网络等）
- 使用模拟对象（mocks）替代依赖
- 运行速度快

### 2. 集成测试 (Integration Tests)

- 测试多个组件的交互
- 使用真实的数据库（内存 SQLite）
- 测试完整的 HTTP 请求/响应周期
- 验证端到端功能

### 3. 测试命名规范

- 单元测试文件：`*_tests.rs`
- 集成测试文件：`*.rs`
- 测试函数名：`test_` 或 `should_` 前缀

### 4. 测试数据管理

- 使用 `TEST_JWT_KEY` 常量
- 每个测试应该独立，避免共享状态
- 测试后清理数据（如果需要）

### 5. 异步测试

- 使用 `#[tokio::test]` 标记异步测试
- 确保 Future 被正确 await

## 添加新测试

### 添加单元测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Arrange
        let input = "test";

        // Act
        let result = some_function(input);

        // Assert
        assert_eq!(result, "expected");
    }
}
```

### 添加集成测试示例

```rust
#[tokio::test]
async fn test_endpoint() {
    let app = test_app().await;
    let payload = json!({
        "field": "value"
    });

    let resp = post_json(&app, "/api/endpoint", payload).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let result: ApiResponse<SomeType> = read_json(resp).await;
    assert_eq!(result.code, 0);
}
```
