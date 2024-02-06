#[macro_export]
macro_rules! serve {
    ($($listen:literal),+; $($pattern:literal => $handler:expr),+) => {{
        let mut app = $crate::Veloce::new(None);
        $(app.route($pattern, $crate::route!($handler));)+
        $(app.bind($listen).await?;)+
        app.run().await
    }};
}

#[macro_export]
macro_rules! route {
    ($handler:expr) => {{
        $crate::addons::Func::new($handler)
    }};
}