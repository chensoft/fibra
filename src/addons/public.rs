use crate::consts::*;
use crate::kernel::*;

pub struct Public {
    pub folder: PathBuf,
}

impl Public {
    pub fn new(folder: PathBuf) -> Self {
        Self {folder}
    }
}

#[async_trait]
impl Handler for Public {
    async fn handle(&self, _ctx: &mut Context) -> Result<()> {
        todo!()
    }
}