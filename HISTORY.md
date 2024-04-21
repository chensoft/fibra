## [Todo]

- context & request & **response** reuse
- tls support
- h1 & h2 support
- cookie support
- session support
- Swagger & OpenAPI
- session keepalive
- stream support
- websocket support
- json5 support
- url builder macros
- embed in another framework
- both sync & async callback
- http proxy
- handle HEAD req without write resp body
- limits: conn's num, conn's num per ip, reqs per conn, req read timeout, handler timeout, max header count, max body size, keepalive time
- sequence id on same connection when keepalived
- addon caching, Cache-Control If-Modified-Since
- addon realip, cors, limiter, Accepts
- radix pack special merge multiple regex into one using (R1)|(R2), must ensure Rx is correct
- lru cache serve hottest path, map to handler directly

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