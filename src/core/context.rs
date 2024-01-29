pub struct Context {
    // request
    // response
    // cookie
    // storage
}

impl Drop for Context {
    fn drop(&mut self) {
        // todo reuse
    }
}

impl Context {
    pub fn next(&mut self) {}
    pub fn abort(&mut self) {}
    pub fn param(&mut self) {}
    pub fn rewrite(&mut self) {}
    pub fn redirect(&mut self) {}
}