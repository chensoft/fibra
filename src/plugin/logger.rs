use crate::consts::*;
use crate::kernel::*;

pub struct Logger {
    pub logger: logkit::Logger,
    pub format: chrono::SecondsFormat,
}

impl Logger {
    pub fn from_millis() -> Self {
        let mut obj = Self {
            logger: logkit::Logger::new(Some(&logkit::StderrTarget)),
            format: chrono::SecondsFormat::Millis,
        };
        obj.logger.mount(logkit::LevelPlugin);
        obj
    }
}

#[async_trait]
impl Handler for Logger {
    async fn handle(&self, ctx: Context) -> Result<Context> {
        let beg = chrono::Local::now();
        let ret = ctx.next().await;
        let end = chrono::Local::now();

        // todo still need recover to construct a valid Context with error response
        match ret {
            Ok(ctx) => {
                logkit::info!(time = beg.to_rfc3339_opts(self.format, false), method = ctx.req.method().as_str(), path = ctx.req.uri().path(), query = ctx.req.uri().query().unwrap_or(""), status = ctx.res.status().as_u16(), elapsed = (end - beg).num_milliseconds());
                Ok(ctx)
            }
            Err(err) => {
                // logkit::info!(time = beg.to_rfc3339_opts(self.format, false), method = ctx.req.method().as_str(), path = ctx.req.uri().path(), query = ctx.req.uri().query().unwrap_or(""), status = 0, elapsed = (end - beg).num_milliseconds());
                Err(err)
            }
        }
    }
}