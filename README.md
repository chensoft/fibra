Fibra
==========================

## ⚠️ Caution: Not Ready for Production! ⚠️

Fast and Powerful HTTP router written in Rust

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][license-badge]][license-url]
[![Documentation][document-badge]][document-url]
[![Build Status][macos-badge]][macos-url]
[![Build Status][linux-badge]][linux-url]
[![Build Status][windows-badge]][windows-url]

[crates-badge]: https://img.shields.io/crates/v/fibra.svg
[crates-url]: https://crates.io/crates/fibra
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/chensoft/fibra?tab=MIT-1-ov-file
[document-badge]: https://docs.rs/fibra/badge.svg
[document-url]: https://docs.rs/fibra
[macos-badge]: https://github.com/chensoft/fibra/actions/workflows/macos.yml/badge.svg
[macos-url]: https://github.com/chensoft/fibra/actions/workflows/macos.yml
[linux-badge]: https://github.com/chensoft/fibra/actions/workflows/linux.yml/badge.svg
[linux-url]: https://github.com/chensoft/fibra/actions/workflows/linux.yml
[windows-badge]: https://github.com/chensoft/fibra/actions/workflows/windows.yml/badge.svg
[windows-url]: https://github.com/chensoft/fibra/actions/workflows/windows.yml

## Features

- Async-ready API
- Powerful router
- Flex middlewares
- Named params
- Glob support
- Regex matching
- URL rewrite
- URL redirect
- Domain filtering
- Subdomain filtering
- Subrouter support
- Varied responses
- Multiple listeners

## Example

```Cargo.toml
[dependencies]
fibra = "0.x"
tokio = { version = "1", features = ["full"] }
```

```rust
use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();
    app.mount(addon::Logger::default());
    app.get("/", "Hello World!")?;
    app.bind("0.0.0.0:3000")?;
    app.run().await
}
```

**Refer to the examples folder for more use cases.**

## Benchmark

- MacBook Air, Apple M2 24G, Sonoma 14.5, Rust 1.78

| Name              |              Time               |
|:------------------|:-------------------------------:|

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04, Rust 1.78

| Name              |              Time               |
|:------------------|:-------------------------------:|

## Documentation

The documentation is [available here](https://docs.rs/fibra).

## License

This software is released under the [MIT License](https://github.com/chensoft/fibra?tab=MIT-1-ov-file).