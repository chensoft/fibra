use crate::consts::*;
use crate::veloce::*;
use crate::kernel::*;

pub trait Router {
    fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler);
    fn group(&mut self, pattern: impl Into<Pattern>) -> &mut Self;

    fn public(&mut self, pattern: impl Into<Pattern>, folder: PathBuf);
    fn reject(&mut self, pattern: impl Into<Pattern>, status: Option<StatusCode>);
    fn rewrite(&mut self, from: impl Into<Pattern>, to: Uri);
    fn redirect(&mut self, from: impl Into<Pattern>, to: Uri, status: Option<StatusCode>);
}

impl Router for Veloce {
    fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        todo!()
    }

    fn group(&mut self, pattern: impl Into<Pattern>) -> &mut Self {
        todo!()
    }

    fn public(&mut self, pattern: impl Into<Pattern>, folder: PathBuf) {
        todo!()
    }

    fn reject(&mut self, pattern: impl Into<Pattern>, status: Option<StatusCode>) {
        todo!()
    }

    fn rewrite(&mut self, from: impl Into<Pattern>, to: Uri) {
        todo!()
    }

    fn redirect(&mut self, from: impl Into<Pattern>, to: Uri, status: Option<StatusCode>) {
        todo!()
    }
}

// impl Veloce {
//     pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
//         // self.mount(addons::Matcher::new(pattern, handler)); // todo add or new
//     }
// 
//     pub fn group(&mut self, pattern: impl Into<Pattern>) -> &mut Veloce {
//         // let mut veloce = Veloce::default();
//         // initial(&mut veloce);
//         // self.route(pattern, veloce);
//         todo!()
//     }
// 
//     // todo define trait in addons, pub use in this file
//     pub fn public(&mut self, pattern: impl Into<Pattern>, folder: PathBuf) {
//         self.route(pattern, addons::Public::new(folder));
//     }
// 
//     pub fn reject(&mut self, pattern: impl Into<Pattern>, status: Option<StatusCode>) {
//         self.route(pattern, addons::Reject::new(status));
//     }
// 
//     pub fn rewrite(&mut self, from: impl Into<Pattern>, to: Uri) {
//         self.route(from, addons::Rewrite::new(to));
//     }
// 
//     pub fn redirect(&mut self, from: impl Into<Pattern>, to: Uri, status: Option<StatusCode>) {
//         self.route(from, addons::Redirect::new(to, status));
//     }
// }


// pub struct Public {
//     pub folder: PathBuf,
// }
// 
// impl Public {
//     pub fn new(folder: PathBuf) -> Self {
//         Self {folder}
//     }
// }
// 
// #[async_trait]
// impl Handler for Public {
//     async fn handle(&self, _ctx: &mut Context) -> Result<()> {
//         todo!()
//     }
// }

// pub struct Redirect {
//     pub to: Uri,
//     pub status: Option<StatusCode>,
// }
// 
// impl Redirect {
//     pub fn new(to: Uri, status: Option<StatusCode>) -> Self {
//         Self {to, status}
//     }
// }
// 
// #[async_trait]
// impl Handler for Redirect {
//     async fn handle(&self, ctx: &mut Context) -> Result<()> {
//         ctx.redirect(self.to.clone(), self.status.clone()).await
//     }
// }

// pub struct Reject {
//     pub status: Option<StatusCode>,
// }
// 
// impl Reject {
//     pub fn new(status: Option<StatusCode>) -> Self {
//         Self {status}
//     }
// }
// 
// #[async_trait]
// impl Handler for Reject {
//     async fn handle(&self, ctx: &mut Context) -> Result<()> {
//         ctx.reject(self.status.clone()).await
//     }
// }

// // todo universe Derive
// pub struct Rewrite {
//     pub to: Uri,
// }
// 
// impl Rewrite {
//     pub fn new(to: Uri) -> Self {
//         Self {to}
//     }
// }
// 
// #[async_trait]
// impl Handler for Rewrite {
//     async fn handle(&self, ctx: &mut Context) -> Result<()> {
//         ctx.rewrite(self.to.clone()).await
//     }
// }