//! Macros to build routes

/// Macros to build routes
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate fibra;
///
/// let app = fibra!{
///     get("/api/v1/user") => "user1",
///     get("/api/v2/user") => "user2",
///     post("/api/v3/user") => "user3",
/// };
///
/// assert_eq!(app.handlers().len(), 1); // only one Matcher here
/// ```
#[macro_export]
macro_rules! fibra {
    ( $($method:ident($path:literal) => $handler:expr),* $(,)? ) => {{
        let mut app = $crate::Fibra::new();
        $( app.$method($path, $handler).expect(format!("path invalid {}", $path).as_str()); )*
        app
    }};
}