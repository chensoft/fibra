use crate::types::*;
use crate::route::*;

pub struct Logger {
    pub logger: logkit::Logger,
    pub level: logkit::Level,
    pub precision: chrono::SecondsFormat,
}

impl Logger {
    pub fn new(target: impl logkit::Target, level: logkit::Level, precision: chrono::SecondsFormat) -> Self {
        let mut logger = logkit::Logger::new(None);
        logger.mount(logkit::LevelPlugin);
        logger.route(target);

        Self {logger, level, precision}
    }

    pub fn level(mut self, level: logkit::Level) -> Self {
        self.level = level;
        self
    }

    pub fn precision(mut self, precision: chrono::SecondsFormat) -> Self {
        self.precision = precision;
        self
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(logkit::StderrTarget, logkit::LEVEL_INFO, chrono::SecondsFormat::Millis)
    }
}

#[async_trait]
impl Handler for Logger {
    async fn handle(&self, ctx: Context) -> FibraResult<Response<Body>> {
        let launch = chrono::Local::now();
        let remote = ctx.peer.to_string();

        let mut record = self.logger.spawn(self.level).unwrap_or_else(|| unreachable!());
        record.append("time", &launch.to_rfc3339_opts(self.precision, false));
        record.append("method", &ctx.method().as_str());
        record.append("path", &ctx.path());
        record.append("query", &ctx.query());

        let result = ctx.next().await;
        let finish = chrono::Local::now();
        let status = match &result {
            Ok(res) => res.status().as_u16(),
            Err(FibraError::StatusCode(status)) => status.as_u16(),
            _ => 0,
        };

        let offset = finish - launch;
        let offset = match self.precision {
            chrono::SecondsFormat::Secs => offset.num_seconds(),
            chrono::SecondsFormat::Millis => offset.num_milliseconds(),
            chrono::SecondsFormat::Micros => offset.num_microseconds().unwrap_or(offset.num_milliseconds() * 1000),
            _ => offset.num_nanoseconds().unwrap_or(offset.num_milliseconds() * 1000000),
        };

        record.append("status", &status);
        record.append("elapsed", &offset);
        record.append("peer", &remote);

        self.logger.flush(record);

        result
    }
}