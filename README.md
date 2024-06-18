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
- Stream support
- Dual-stack support
- Multiple listeners

## Example

### Start

```Cargo.toml
[dependencies]
fibra = "0.x"
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use fibra::*;

#[tokio::main]
async fn main() -> FibraResult<()> {
    let mut app = Fibra::new();
    app.get("/", "Hello World!")?;
    app.bind(3000)?;
    app.run().await
}
```

### Basic

* [Routing Patterns](https://github.com/chensoft/fibra/blob/HEAD/examples/routing.rs)
* [Subrouter Support](https://github.com/chensoft/fibra/blob/HEAD/examples/multisite.rs)
* [Varied Responses](https://github.com/chensoft/fibra/blob/HEAD/examples/response.rs)
* [Stream Support](https://github.com/chensoft/fibra/blob/HEAD/examples/stream.rs)
* [URL Rewrite](https://github.com/chensoft/fibra/blob/HEAD/examples/rewrite.rs)
* [IPv6 Support](https://github.com/chensoft/fibra/blob/HEAD/examples/ipv6.rs)

### Tools

* [Echo Server](https://github.com/chensoft/fibra/blob/HEAD/examples/echo.rs)

## Performance

- todo use two machines, wrk

## Benchmark

- MacBook Air, Apple M2 24G, Sonoma 14.5, Rust 1.79

| Name                |              Time               |
|:--------------------|:-------------------------------:|
| run_baseline        | [307.88 ns 309.69 ns 311.91 ns] |
| run_routes_1        | [898.58 ns 900.80 ns 903.07 ns] |
| run_routes_8_front  | [904.38 ns 906.53 ns 908.77 ns] |
| run_routes_8_middle | [905.60 ns 907.58 ns 909.63 ns] |
| run_routes_8_back   | [929.73 ns 932.53 ns 935.54 ns] |
| run_routes_16       | [938.56 ns 940.53 ns 942.66 ns] |
| req_empty           | [54.292 ns 54.320 ns 54.349 ns] |
| req_hyper           | [156.59 ns 156.73 ns 156.86 ns] |
| req_build           | [166.22 ns 166.29 ns 166.36 ns] |
| res_empty           | [11.436 ns 11.450 ns 11.465 ns] |
| res_full            | [119.16 ns 119.53 ns 119.98 ns] |
| res_status_body     | [15.415 ns 15.417 ns 15.420 ns] |
| res_status          | [11.562 ns 11.572 ns 11.581 ns] |
| res_body            | [15.391 ns 15.393 ns 15.397 ns] |