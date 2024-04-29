## [Unreleased] - 2024-05-xx

### Added

- Powerful router
- Async-ready API
- Flex middleware
- Named params
- Regex matching
- Glob support
- URL rewrite
- URL redirect
- Subdomain support
- Subrouter support
- Cache storage
- Multiple listener

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

- fibra config
- tls support with tls info, ver, sni
- h2 support
- content type, referer, user agent, is keepalive, content encoding, accepts, length, type...
- addon: realip, port, caching, Cache-Control If-Modified-Since, cors, limiter, Accepts
- limits: conn's num, conn's num per ip, reqs per conn, req read timeout, handler timeout, max header count, max body size, keepalive time
- websocket support
- cookie support
- session support
- Swagger & OpenAPI
- bench: https://www.techempower.com/benchmarks/#section=intro&test=fortune