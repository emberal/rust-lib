/// Create an axum router function with the given body or routes.
/// # Examples
/// ```
/// use lib::router;
/// async fn index() {}
///
/// router!(
///     get "/" => index,
///     get "/openapi" => || async {}
/// );
/// ```
/// ```
/// use lib::router;
/// async fn simplify(path: axum::extract::path::Path<String>) {}
/// router!("/simplify", lib::routes!(
///     get "/:exp" => simplify,
///     get "/table/:exp" => || async {}
/// ));
/// ```
#[macro_export]
#[cfg(feature = "axum")]
macro_rules! router {
    ($body:expr) => {
        pub(crate) fn router() -> axum::Router {
            $body
        }
    };
    ($route:expr, $router:expr) => {
        router!(axum::Router::new().nest($route, $router));
    };
    ($($method:ident $route:expr => $func:expr),* $(,)?) => {
        router!($crate::routes!($($method $route => $func),*));
    };
}

/// Create a router with the given routes.
/// # Examples
/// ```
/// async fn index() {}
///
/// let _: axum::Router<()> = lib::routes!(
///     get "/" => index,
///     post "/" => || async {}
/// );
/// ```
#[macro_export]
#[cfg(feature = "axum")]
macro_rules! routes {
    ($($method:ident $route:expr => $func:expr),* $(,)?) => {
        axum::Router::new()
            $(
                .route($route, axum::routing::$method($func))
            )*
    };
}

#[macro_export]
#[cfg(feature = "axum")]
macro_rules! join_routes {
    ($($route:expr),* $(,)?) => {
        axum::Router::new()$(
         .merge($route)
        )*
    };
}

#[cfg(all(test, feature = "axum"))]
mod tests {
    use axum::Router;

    async fn index() {}

    #[test]
    fn test_router() {
        router!(
            get "/" => index,
            post "/" => || async {}
        );
    }

    #[test]
    fn test_router_with_expression() {
        router!(Router::new());
    }

    #[test]
    fn test_nested_router() {
        router!(
            "/simplify",
            routes!(
                get "/:exp" => || async {},
                get "/table/:exp" => || async {}
            )
        );
    }

    #[test]
    fn test_routes() {
        let _router: Router<()> = routes!(
            get "/" => index,
            post "/" => || async {}
        );
    }

    #[test]
    fn test_join_routes() {
        let _router: Router = join_routes![Router::new(), Router::new()];
    }
}
