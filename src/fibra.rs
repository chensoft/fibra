use crate::types::*;
use crate::inner::*;

pub struct Fibra {
    mounted: Package,
    sockets: Vec<socket2::Socket>,
}

impl Fibra {
    pub fn route(&mut self, pattern: &'static str, handler: impl Handler) -> FibraResult<&mut Routine> {
        self.force::<Matcher>().add(pattern, handler)
    }

    pub fn group(&mut self, pattern: &'static str) -> FibraResult<&mut Fibra> {
        Ok(self.route(pattern, Fibra::default())?.trust())
    }

    pub fn limit(&mut self) -> &mut Limiter {
        self.force()
    }

    pub fn mount<T: Handler>(&mut self, handler: T) -> &mut T {
        self.mounted.insert(handler)
    }

    pub fn force<T: Handler + Default>(&mut self) -> &mut T {
        self.mounted.ensure()
    }

    pub fn catch(&mut self, handler: impl Fn(&Catcher, FibraError) -> Response<Body> + Send + Sync + 'static) {
        match self.mounted.first::<Catcher>() {
            Some(obj) => obj.handler = Box::new(handler),
            _ => unreachable!()
        }
    }

    pub fn visit(&self) -> Iter<BoxHandler> {
        self.mounted.bundle.iter()
    }

    pub fn bind(&mut self, addr: impl ToSocketAddrs) -> FibraResult<&mut socket2::Socket> {
        self.sockets.push(StdTcpListener::bind(addr)?.into());
        Ok(self.sockets.last_mut().unwrap_or_else(|| unreachable!()))
    }

    pub async fn run(mut self) -> FibraResult<()> {
        use hyper::Server;
        use hyper::server::conn::AddrStream;
        use hyper::service::{make_service_fn, service_fn};

        let mut sockets = std::mem::take(&mut self.sockets);
        let appself = Arc::new(self);
        let service = make_service_fn(|conn: &AddrStream| {
            let appself = appself.clone();
            let address = (conn.local_addr(), conn.remote_addr());

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let appself = appself.clone();
                    let context = Context::new(appself.clone(), address.0, address.1, req);

                    async move { appself.handle(context).await }
                }))
            }
        });

        let mut servers = vec![];

        while let Some(socket) = sockets.pop() {
            servers.push(Server::from_tcp(socket.into())?.serve(service));
        }

        futures::future::join_all(servers).await;

        Ok(())
    }
}

impl Default for Fibra {
    fn default() -> Self {
        Self { mounted: Package::new(vec![Catcher::default()]), sockets: vec![] }
    }
}

#[async_trait]
impl Handler for Fibra {
    async fn handle(&self, ctx: Context) -> FibraResult<Response<Body>> {
        self.mounted.handle(ctx).await
    }
}