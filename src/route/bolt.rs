use crate::route::*;
use crate::types::*;

pub struct Bolt {
    // mounted: Package,
    sockets: Vec<Socket>,
}

impl Bolt {
    // pub fn get(&mut self, )

    // todo add get, post, ... like ctx
    // todo remove BoltResult
    // pub fn route(&mut self, pattern: &'static str, handler: impl Handler) -> BoltResult<&mut Routine> {
    //     self.force::<Matcher>().add(pattern, handler)
    // }
    // 
    // pub fn group(&mut self, pattern: &'static str) -> BoltResult<&mut Router> {
    //     Ok(self.route(pattern, Router::default())?.trust())
    // }
    // 
    // pub fn limit(&mut self) -> &mut Limiter {
    //     self.force()
    // }
    // 
    // pub fn mount<T: Handler>(&mut self, handler: T) -> &mut T {
    //     self.mounted.insert(handler)
    // }
    // 
    // pub fn force<T: Handler + Default>(&mut self) -> &mut T {
    //     self.mounted.ensure()
    // }
    // 
    // pub fn catch(&mut self, handler: impl Fn(&Catcher, RouterError) -> Response + Send + Sync + 'static) {
    //     match self.mounted.first::<Catcher>() {
    //         Some(obj) => obj.handler = Box::new(handler),
    //         _ => unreachable!()
    //     }
    // }
    // 
    // pub fn visit(&self) -> Iter<BoxHandler> {
    //     self.mounted.bundle.iter()
    // }

    pub fn bind(&mut self, addr: impl ToSocketAddrs) -> BoltResult<&mut Socket> {
        let last = self.sockets.len();
        self.sockets.push(StdTcpListener::bind(addr)?.into());
        Ok(&mut self.sockets[last])
    }

    pub async fn run(mut self) -> BoltResult<()> {
        use hyper::Server;
        use hyper::server::conn::AddrStream;
        use hyper::service::{make_service_fn, service_fn};

        let mut sockets = std::mem::take(&mut self.sockets);
        let app = Arc::new(self);
        let srv = make_service_fn(|conn: &AddrStream| {
            let app = app.clone();
            let con = Arc::new(Connection::from((conn.local_addr(), conn.remote_addr())));

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    // simply incr because no concurrent requests on one connection
                    con.count_ref().fetch_add(1, atomic::Ordering::Relaxed);

                    // construct our own context object for each request
                    let ctx = Context::from((app.clone(), con.clone(), Request::from(req)));

                    // processing the request from the ctx's next method
                    async move { Ok::<_, BoltError>(ctx.next().await?.into()) }
                }))
            }
        });

        let mut servers = vec![];

        while let Some(socket) = sockets.pop() {
            servers.push(Server::from_tcp(socket.into())?.serve(srv));
        }

        futures::future::join_all(servers).await;

        Ok(())
    }
}

impl Default for Bolt {
    fn default() -> Self {
        // Self { mounted: Package::new(vec![Catcher::default()]), sockets: vec![] }
        todo!()
    }
}

#[async_trait]
impl Handler for Bolt {
    async fn handle(&self, ctx: Context) -> BoltResult<Response> {
        // self.mounted.handle(ctx).await
        todo!()
    }

    fn select(&self, idx: usize) -> Option<&BoxHandler> {
        // match idx == 0 {
        //     true => ,
        //     false => {}
        // }
        todo!()
    }
}