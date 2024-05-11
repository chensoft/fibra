## [Unreleased] - 2024-05-xx

### Added

- Powerful router
- Async-ready API
- Flex middleware
- Named params
- Glob support
- Regex matching
- URL rewrite
- URL redirect
- Domain routing
- Subdomain routing
- Subrouter support
- Cookie support
- Session management
- Multiple listener
- Handy response

## [Todo]

### Performance

- form decode, use decoder
- reuse ctx, req, res, con
- lru cache serve hottest path, map to handler directly, record handlers call chain

### Improve

- less unreachable!
- embed in another framework
- sync callback support
- handle HEAD req without write resp body

### Features

- fibra config, case sensitive matching
- tls support with tls info, ver, sni
- h2 support: extension, server push, stream priority
- content type, referer, user agent, is keepalive, content encoding, accept, accept-encoding, accept-language, length, type...
- addon: realip, port, caching, Cache-Control If-Modified-Since, cors, limiter, Accepts
- limits: conn's num, conn's num per ip, reqs per conn, req read timeout, handler timeout, max header count, max body size, keepalive time
- websocket support
- cookie support
- session support
- Swagger & OpenAPI
- hook support
- bench: https://www.techempower.com/benchmarks/#section=intro&test=fortune
- example: simple downloader and indexer, httpbin-like tools
- deploy easily to serverless platform
- impl Handler for sync closure, even a File fd
- impl File for Response
- temp storage for context