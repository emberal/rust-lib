#[macro_export]
#[cfg(feature = "axum")]
macro_rules! create_app {
    ($router:expr) => {
        $router
    };
    ($router:expr, $($layer:expr),* $(,)?) => {
        $router$(.layer($layer))*
    };
}

#[cfg(all(test, feature = "axum"))]
mod tests {
    use axum::Router;

    #[test]
    fn test_create_app_router_only() {
        let _app: Router<()> = create_app!(Router::new());
    }
}
