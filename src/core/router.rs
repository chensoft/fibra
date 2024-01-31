use crate::exts;
use super::define::*;
use super::context::*;

pub struct Router;

impl Router {
    pub fn new() -> Self { todo!() }
    pub fn sub(&mut self, _path: Cow<'static, str>) -> Self { todo!() }

    pub fn route(&mut self, _path: Cow<'static, str>, _func: impl Fn(&mut Context)) {}
    pub fn catch(&mut self, _func: impl Fn(&mut Context)) {}
    pub fn public(&mut self, _path: Cow<'static, str>, _dir: PathBuf, _conf: Option<Public>) {}
    pub fn reject(&mut self, _path: Cow<'static, str>) {}
    pub fn rewrite(&mut self, _from: Cow<'static, str>, _to: Cow<'static, str>) {}
    pub fn redirect(&mut self, _from: Cow<'static, str>, _to: Cow<'static, str>, _code: http::StatusCode) {}
    pub fn attach(&mut self, _plugin: impl exts::Extension) {}
    pub fn detach(&mut self, _delete: impl Fn(&dyn exts::Extension) -> bool) {}
}