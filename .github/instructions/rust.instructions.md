---
name: Rust 编码规范
description: Rust 文件的编码规范
applyTo: '**/*.rs'
---

# Rust 编码规范

## 基本原则

- 优先遵循 Rust 官方规范（rustfmt + clippy）
- 代码应当 安全、可读、可维护
- 避免过度设计，优先简单直接的实现
- 明确区分：
  - 业务逻辑
  - 错误处理
  - 数据结构
- 基于 Rust 2024 Edition 编写代码，利用最新的语言特性和改进

## 风格

### 严格遵守

- 使用 `cargo fmt` 统一格式
- 使用 `cargo clippy`，在无合理说明时，不忽略警告
- 避免使用 `unsafe`，除非有充分理由，并且必须如下示例添加详细注释说明原因和安全保证

  ```rust
  // SAFETY: 这是一个示例，说明为什么需要使用 unsafe，以及如何保证安全
  unsafe {
      // 这里是 unsafe 代码块的实现
  }
  ```

- 避免滥用 `unwrap` 与 `expect`，若使用则必须提供清晰的错误信息

  ```rust
  let value = some_option.unwrap_or_else(|| panic!("Expected a value, but got None"));
  ```

### 推荐实践

- 使用 `Result<T, E>` 与 `?` 进行错误传播
- 使用 `Option<T>` 来表示可能缺失的值
- 使用 `match` 或 `if let` 来处理枚举类型
- 保持函数短小，单一职责，避免过长的函数和过多的参数
- 函数参数超过五个就应当考虑使用结构体来传递参数

### 应该避免

- 全局可变状态 `static mut`，考虑采用 `OnceLock`、`LazyLock`、`Mutex` 等替代方案
- 逻辑嵌套超过 3 层
- 无意义的 `clone`
- 无意义的魔法值

## 命名

### 命名风格

| 类型     | 规范                 | 示例                                            |
| :------- | :------------------- | :---------------------------------------------- |
| 变 量    | snake_case           | `let user_name: String;`                        |
| 函 数    | snake_case           | `fn calculate_area(radius: f64) -> f64 { ... }` |
| 枚 举    | CamelCase            | `enum Color { Red, Green, Blue }`               |
| 常 量    | SCREAMING_SNAKE_CASE | `const MAX_SIZE: usize = 100;`                  |
| 模 块    | snake_case           | `mod user_profile;`                             |
| 特 征    | CamelCase            | `trait Drawable { ... }`                        |
| 结 构 体 | CamelCase            | `struct UserProfile { ... }`                    |
| 宏       | snake_case!          | `macro_rules! my_macro { ... }`                 |

### 命名建议

- 变量和函数名应当具有描述性，能够清晰表达其用途和功能
- 避免使用缩写，除非是广泛认可的缩写（如 `id`、`url`）
- 对于布尔变量与返回布尔的函数，建议使用 `is`、`has`、`can` 等前缀来提高可读性

## 错误处理

- 除非在初始化逻辑中，否则应避免使用 `panic!` 进行错误处理
- 使用 `Result<T, E>` 与 `?` 来表示可能失败的操作
- 在函数签名中明确错误类型，避免使用 `Box<dyn Error>`
- 提供有意义的错误信息，避免使用过于笼统的错误描述
- 自定义的错误类型应当实现 `std::error::Error` trait，以便与 Rust 的错误处理生态系统兼容
- 推荐使用 `thiserror` 或 `anyhow` 等库来简化错误定义和处理
- 应当在每个逻辑分支与关键节点提供对应的日志记录，以便后续排查问题，例如：

  ```rust
  fn request_payment_example(payload: Payload) -> Result<(), MyError> {
      log::info!("请求支付, 订单号 {}, 用户ID {}", payload.order_id, payload.user_id);

      log::info!("获取用户付款信息")
      // ...
      log::info!("请求支付接口")

      match user.billing_info.card_type {
          CardType::Visa => {
              log::info!("Visa 卡支付")
              // ...
          },
          CardType::MasterCard => {
              log::info!("MasterCard 卡支付")
              // ...
          },
          _ => {
              log::error!("不支持的卡类型: {}", user.billing_info.card_type);
              return Err(MyError::UnsupportedCardType(user.billing_info.card_type));
          }
      }
      log::info!("订单 {} 支付请求完成", payload.order_id);

      Ok(())
  }
  ```

- 日志级别建议：
  - `log::trace!`：用于非常详细的调试信息，通常只在开发过程中使用
  - `log::debug!`：用于调试信息，帮助开发者理解程序的运行状态
  - `log::info!`：用于重要的运行时事件，如请求开始、请求完成等
  - `log::warn!`：用于潜在的问题或非预期的情况，但不影响程序正常运行
  - `log::error!`：用于错误事件，表示程序无法继续正常运行

## 模块组织

- 每个模块应当有明确的职责，避免过于庞大和复杂
- 使用 `mod` 来组织代码，避免过度嵌套
- 将公共接口暴露在模块的根部，避免过多的内部细节暴露给外部
- 默认私有，仅暴露必要的公共接口
- 使用 `pub(crate)` 来限制模块内部的可见性
- 避免循环依赖，保持模块之间的清晰边界
- 使用 `mod_name.rs` 来定义模块，保持文件结构清晰
- 对于大型模块，应当使用子模块来进一步组织代码

## 注释

- 代码应当自解释，注释仅用于解释复杂的逻辑或设计决策
- 所有的 `pub` 对象都必须使用文档注释 `///` 来提供文档说明
- 注释应该包含功能说明，参数说明，返回值说明，以及可能的错误情况
- 避免使用过时的注释，保持注释与代码同步更新
- 对于复杂的算法或逻辑，建议使用内联注释来解释关键步骤
- 使用 `TODO` 注释来标记需要改进或未完成的部分，但应当定期检查和清理这些注释
- 使用 `FIXME` 注释来标记已知的错误或问题，并提供足够的信息以便后续修复
- 使用 `HACK` 注释来标记临时代码或不理想的解决方案，并提供改进建议
- 使用 `SAFETY` 注释来标记 `unsafe` 代码块，并详细说明安全保证和使用理由
- 使用 `NOTE` 注释来提供额外的信息或上下文，帮助理解代码的设计和实现

## 测试

- 关键逻辑必须且应当有单元测试
- 集成测试应当覆盖主要功能和边界情况
- 使用 `#[cfg(test)]` 模块来组织测试代码，保持测试与生产代码分离
- 测试函数应当具有描述性名称，能够清晰表达测试的目的和预期结果
- 使用 `assert!`、`assert_eq!` 等宏来验证测试结果，并提供有意义的错误信息
- 使用 `mock` 或 `stub` 来模拟外部依赖，确保测试的独立性和可靠性
- 允许使用 `unwrap` 和 `expect` 在测试代码中，以简化测试实现，但应当提供清晰的错误信息
- 定期运行测试，保持测试覆盖率，并及时修复测试失败的问题

## 性能与安全

- 优先使用安全的 Rust 代码，避免使用 `unsafe`，除非有充分理由，并且必须添加详细注释说明原因和安全保证
- 先实现，再谈论优化，避免早期的过度优化
- 除非有明确的性能瓶颈，否则应当优先考虑代码的可读性和可维护性
