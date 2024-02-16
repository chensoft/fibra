use crate::config::*;
use crate::consts::*;
use crate::kernel::*;

pub struct Public {
    pub folder: PathBuf,
    pub config: Static,
}

impl Public {
    pub fn new(folder: PathBuf, config: Option<Static>) -> Self {
        Self {folder, config: config.unwrap_or_default()}
    }
}

#[async_trait]
impl Handler for Public {
    async fn handle(&self, ctx: Context) -> Result<Context> {
        todo!()
    }
}