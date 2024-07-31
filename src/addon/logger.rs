//! Logger Middleware
use crate::route::*;
use crate::types::*;

/// Logger Middleware
pub struct Logger {
    logger: logkit::Logger,
    level: String,
}

impl Logger {
    /// Create a new object
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    /// app.mount(addon::Logger::new());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { logger: logkit::Logger::new(Some(&logkit::StderrTarget)), level: "info".to_string() }
    }

    /// Get the inner logger
    #[inline]
    pub fn logger(&self) -> &logkit::Logger {
        &self.logger
    }

    /// Get the inner logger
    #[inline]
    pub fn logger_mut(&mut self) -> &mut logkit::Logger {
        &mut self.logger
    }

    /// Set write target of the logger
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    /// app.mount(addon::Logger::new().route(logkit::StdoutTarget));
    /// ```
    #[inline]
    pub fn route(mut self, target: impl logkit::Target) -> Self {
        self.logger.route(target);
        self
    }

    /// Set our log's default level
    ///
    /// # Examples
    ///
    /// ```
    /// use fibra::*;
    ///
    /// let mut app = Fibra::new();
    /// app.mount(addon::Logger::new().level(logkit::LEVEL_DEBUG));
    /// ```
    #[inline]
    pub fn level(mut self, level: logkit::Level) -> Self {
        self.level = logkit::level_to_str(level).map(|v| v.to_string()).unwrap_or(level.to_string());
        self
    }
}

impl Default for Logger {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Service for Logger {
    async fn invoke(&self, ctx: Context) -> FibraResult<Response> {
        let begin = ctx.created().duration_since(UNIX_EPOCH)?;
        let reqid = ctx.header("x-request-id").to_string();

        // request log
        let mut record = self.logger.spawn(0, logkit::Source::default()).unwrap_or_else(|| unreachable!());
        record.append("time", &begin.as_millis());
        record.append("level", &self.level);
        record.append("id", &reqid);
        record.append("kind", &"req");
        record.append("method", &ctx.method().as_str());
        record.append("path", &ctx.path());
        record.append("query", &ctx.req().query());
        record.append("host", &ctx.host());
        record.append("ip", &ctx.realip().to_string());

        self.logger.flush(record);

        // call service
        let result = ctx.next().await;
        let finish = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let offset = (finish.as_nanos() as f64 - begin.as_nanos() as f64) / 1_000_000_000.0;
        let status = result.as_ref().map_or(0, |v| v.status_ref().as_u16());

        // response log
        let mut record = self.logger.spawn(0, logkit::Source::default()).unwrap_or_else(|| unreachable!());
        record.append("time", &finish.as_millis());
        record.append("level", &self.level);
        record.append("id", &reqid);
        record.append("kind", &"res");
        record.append("status", &status);
        record.append("elapsed", &offset);

        self.logger.flush(record);

        result
    }
}