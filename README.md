# rust-port-checker 🦀

[![Rust CI](https://github.com/yoona333/rust-port-checker/actions/workflows/rust.yml/badge.svg)](https://github.com/yoona333/rust-port-checker/actions/workflows/rust.yml)

一个轻量、零第三方依赖的 **Rust TCP 端口连通性检测工具**。

A lightweight, dependency-free TCP connectivity checker written in Rust.

## Features / 功能

- 检测单个或多个 TCP 端口是否可建立连接
- 支持端口范围，如 `8000-8005`
- 支持域名、IPv4 和 IPv6 地址解析
- 自定义超时时间，最多支持 1024 个不同端口
- 提供 Rust 单元测试和 GitHub Actions CI
- 仅依赖 Rust 标准库，无需外部 crates

> 注意：本工具用于自己管理的服务器或已获授权的主机的连通性排查。`CLOSED` 表示本次连接没有成功，不一定代表端口没有服务，也可能是网络或防火墙限制。

## Requirements / 环境

- Rust stable (2021 edition)
- Cargo

安装 Rust: https://rustup.rs/

## Quick start / 快速开始

```bash
git clone https://github.com/yoona333/rust-port-checker.git
cd rust-port-checker
cargo run -- 127.0.0.1 22,80,443
```

检测端口范围：

```bash
cargo run -- 127.0.0.1 8000-8003 --timeout-ms 1000
```

查看帮助：

```bash
cargo run -- --help
```

输出示例（结果根据实际环境而定）：

```text
OPEN    127.0.0.1:8080
CLOSED  127.0.0.1:8081
```

退出码：`0` 表示所有指定端口均可连接；`1` 表示至少一个端口无法连接；`2` 表示输入或网络解析错误。

## Tests / 测试

```bash
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets -- -D warnings
```

## Project structure / 项目结构

```text
rust-port-checker/
├── .github/workflows/rust.yml  # CI
├── .gitignore
├── Cargo.toml
├── LICENSE
├── README.md
└── src/
    ├── lib.rs                  # Port parsing and TCP connectivity
    └── main.rs                 # CLI
```

## Roadmap / 后续计划

- [ ] CSV/JSON 格式输出
- [ ] 并发检测，提高多端口扫描效率
- [ ] 提供发布二进制包的 Release 工作流

## Contributing / 参与贡献

欢迎提交 Issue 与 Pull Request 改进文档、错误处理、测试与兼容性。

## License / 开源协议

MIT © 2026 Yoona
