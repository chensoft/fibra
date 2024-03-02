// use crate::addons;
use crate::consts::*;
use crate::kernel::*;

pub struct Veloce {
    cached: Vec<Box<dyn Handler>>,
    routes: Arc<Vec<Box<dyn Handler>>>,
    listen: Vec<StdTcpListener>,
}

impl Veloce {
    pub fn mount(&mut self, handler: impl Handler) {
        self.cached.push(Box::new(handler));
    }

    pub fn freeze(&mut self) {
        for handler in &mut self.cached {
            if let Some(veloce) = handler.as_any_mut().downcast_mut::<Veloce>() {
                veloce.freeze();
            }
        }

        self.routes = Arc::new(std::mem::take(&mut self.cached));
    }

    pub fn bind(&mut self, addr: impl ToSocketAddrs) -> Result<()> {
        Ok(self.take(StdTcpListener::bind(addr)?))
    }

    pub fn take(&mut self, tcp: StdTcpListener) {
        self.listen.push(tcp);
    }

    pub async fn run(mut self) -> Result<()> {
        use hyper::Server;
        use hyper::server::conn::AddrStream;
        use hyper::service::{make_service_fn, service_fn};

        self.freeze();

        let mut sockets = std::mem::take(&mut self.listen);
        let appself = Arc::new(self);
        let service = make_service_fn(|conn: &AddrStream| {
            let appself = appself.clone();
            let address = (conn.local_addr(), conn.remote_addr());

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let appself = appself.clone();
                    let mut context = Context::new(appself.clone(), req.into(), Address::new(address.0, address.1));

                    async move {
                        match appself.handle(&mut context).await {
                            Ok(_) => Ok::<_, Infallible>(context.res),
                            Err(_) => unreachable!()
                        }
                    }
                }))
            }
        });

        let mut servers = vec![];

        while let Some(socket) = sockets.pop() {
            servers.push(Server::from_tcp(socket)?.serve(service));
        }

        futures::future::join_all(servers).await;

        Ok(())
    }
}

impl Veloce {
    pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        // self.router.add(pattern, handler);
    }

    pub fn group(&mut self, pattern: impl Into<Pattern>, initial: impl Fn(&mut Veloce)) {
        // let mut veloce = Veloce::default();
        // initial(&mut veloce);
        // self.route(pattern, veloce);
    }

    pub fn public(&mut self) {
    }

    pub fn reject(&mut self) {
    }

    pub fn rewrite(&mut self) {
    }

    pub fn redirect(&mut self) {
    }

    pub fn catch(&mut self) {
        // todo replace default catcher if user call this fn
    }
}

impl Default for Veloce {
    fn default() -> Self {
        // Self {addons: vec![Arc::new(addons::Catcher::default())], listen: vec![]}
        todo!()
    }
}

#[async_trait]
impl Handler for Veloce {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        // match self.router.get(ctx.search.as_str()) {
        //     Some(val) => ctx.routes.push_front(val),
        //     None => return Err(StatusCode::NOT_FOUND.into_error()),
        // };
        // 
        // // todo subrouter
        // ctx.parent = ctx.search.clone();
        // ctx.search = "/".to_string();
        // 
        // for handler in self.addons.iter().rev() {
        //     ctx.routes.push_front(handler.clone());
        // }
        // 
        // ctx.next().await
        todo!()
    }
}