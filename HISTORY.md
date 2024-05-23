## [0.0.1] - 2024-05-xx

### Added

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
- Stream support
- Varied responses
- Dual-stack support
- Multiple listeners

## [Todo]

### Performance

- form decode, use decoder
- reuse ctx, req, res, con
- lru cache serve hottest path, map to handler directly, record handlers call chain
- automatically adjust backlog value according to traffic

### Improve

- less unreachable!
- embed in another framework
- sync callback support
- handle HEAD req without write resp body
- regex do not support {} inside
- matcher use entry or_insert
- Scheme check tls socket, scheme is none when self comes from hyper connection

### Features

- Response: add file fn, auto detect file mime, chunk transfer, stream wrap attachment header
- tls support with tls info, ver, sni
- h2 support: extension, server push, stream priority, RST_STREAM cancel request in flight, res push support via addon HTTP Link Headers
- h3 support
- content type, referer, user agent, is keepalive, content encoding, accept, accept-encoding, accept-language, length, type...
- addon: realip, port, caching, Cache-Control If-Modified-Since, cors, limiter, Accepts, logger customize
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
- context: temp storage, save data into file
- trailing headers after the body in h1.1
- test: curl --http2 --parallel -v http://localip.cc:3000/first http://localip.cc:3000/second
- test: curl --http2-prior-knowledge --parallel -v http://localip.cc:3000/first http://localip.cc:3000/second
- graceful shutdown