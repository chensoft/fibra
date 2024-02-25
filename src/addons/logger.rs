use crate::consts::*;
use crate::kernel::*;

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
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(logkit::StderrTarget, logkit::LEVEL_INFO, chrono::SecondsFormat::Millis)
    }
}

#[async_trait]
impl Handler for Logger {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        let beg = chrono::Local::now();
        let ret = ctx.next().await;
        let end = chrono::Local::now();

        let status = match &ret {
            Ok(_) => ctx.res.status().as_u16(),
            Err(err) => match err.downcast_ref::<Error>() {
                Some(Error::HttpStatusCode(status)) => status.as_u16(),
                _ => 0,
            }
        };

        let offset = end - beg;
        let offset = match self.precision {
            chrono::SecondsFormat::Secs => offset.num_seconds(),
            chrono::SecondsFormat::Millis => offset.num_milliseconds(),
            chrono::SecondsFormat::Micros => offset.num_microseconds().unwrap_or(offset.num_milliseconds() * 1000),
            _ => offset.num_nanoseconds().unwrap_or(offset.num_milliseconds() * 1000000),
        };

        logkit::record!(
            self.logger,
            self.level,
            time = beg.to_rfc3339_opts(self.precision, false),
            method = ctx.req.method().as_str(),
            path = ctx.req.uri().path(),
            query = ctx.req.uri().query().unwrap_or(&""),
            status = status,
            elapsed = offset,
        );

        ret
    }
}