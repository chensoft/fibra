use crate::kernel::*;

// pub struct Domain {
// 
// }
// 
// impl Limit for Domain {
//     fn allow(&self, ctx: &Context) -> Result<bool> {
//         todo!()
//     }
// }

pub fn domain(name: impl Into<String>) -> impl Fn(&Context) -> bool {
    let name = name.into();
    move |ctx| {
        // todo
        ctx.req.uri().host() == Some(name.as_str())
    }
}

pub fn domains(names: Vec<String>) -> bool {
    todo!()
}