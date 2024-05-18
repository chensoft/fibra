//! Logger Middleware
use crate::route::*;
use crate::types::*;

/// Logger Middleware
///
/// # Examples
///
/// ```
/// use fibra::*;
///
/// let mut app = Fibra::new();
/// app.mount(addon::Logger::default());
/// ```
pub struct Logger {
    logger: logkit::Logger,
    level: String,
}

impl Logger {
    /// Get the inner logger
    pub fn logger(&self) -> &logkit::Logger {
        &self.logger
    }

    /// Get the inner logger
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
    /// app.mount(addon::Logger::default().route(logkit::StdoutTarget));
    /// ```
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
    /// app.mount(addon::Logger::default().level(logkit::LEVEL_DEBUG));
    /// ```
    pub fn level(mut self, level: logkit::Level) -> Self {
        self.level = logkit::level_to_str(level).map(|v| v.to_string()).unwrap_or(level.to_string());
        self
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self { logger: logkit::Logger::new(Some(&logkit::StderrTarget)), level: "info".to_string() }
    }
}

#[async_trait]
impl Handler for Logger {
    async fn handle(&self, ctx: Context) -> FibraResult<Response> {
        let begin = ctx.created().duration_since(UNIX_EPOCH)?.as_millis();
        let reqid = ctx.reqid();

        // request log
        let mut record = self.logger.spawn(0, logkit::Source::default()).unwrap_or_else(|| unreachable!());
        record.append("time", &begin);
        record.append("level", &self.level);
        record.append("kind", &"req");
        record.append("reqid", &reqid);
        record.append("method", &ctx.method().as_str());
        record.append("path", &ctx.path());
        record.append("query", &ctx.req().query());
        record.append("ip", &ctx.remote().ip().to_string());

        self.logger.flush(record);

        // call handler
        let result = ctx.next().await;
        let finish = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let offset = finish - begin;
        let status = match &result {
            Ok(res) => res.status_ref().as_u16(),
            Err(FibraError::PathNotFound) => Status::NOT_FOUND.as_u16(),
            _ => 0,
        };

        // response log
        let mut record = self.logger.spawn(0, logkit::Source::default()).unwrap_or_else(|| unreachable!());
        record.append("time", &begin);
        record.append("level", &self.level);
        record.append("kind", &"res");
        record.append("reqid", &reqid);
        record.append("status", &status);
        record.append("elapsed", &offset);

        self.logger.flush(record);

        result
    }
}