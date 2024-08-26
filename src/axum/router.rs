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
macro_rules! router {
    ($body:expr) => {
        pub fn router() -> axum::Router {
            $body
        }
    };
    ($body:expr; $state:ty) => {
        pub fn router() -> axum::Router<$state> {
            $body
        }
    };
    ($body:expr; $state:ident: $($bound:tt),*) => {
        pub fn router<$state: $($bound+)* 'static>() -> axum::Router<$state> {
            $body
        }
    };
    ($body:expr; $generic:ident: $($bound:tt),* -> $state:ty) => {
        pub fn router<$generic: $($bound+)* 'static>() -> axum::Router<$state<$generic>> {
            $body
        }
    };
    ($route:expr, $router:expr) => {
        router!(axum::Router::new().nest($route, $router));
    };
    ($route:expr, $router:expr, $state:ty) => {
        router!(axum::Router::new().nest($route, $router); $state);
    };
    ($route:expr, $router:expr, $state:ident: $($bound:tt),*) => {
        router!(axum::Router::new().nest($route, $router); $state: $($bound),*);
    };
      ($route:expr, $router:expr, $generic:ident: $($bound:tt),* -> $state:ty) => {
        router!(axum::Router::new().nest($route, $router); $generic: $($bound),* -> $state);
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
macro_rules! routes {
    ($($method:ident $route:expr => $func:expr),* $(,)?) => {
        axum::Router::new()
            $(
                .route($route, axum::routing::$method($func))
            )*
    };
}

#[macro_export]
macro_rules! join_routes {
    ($($route:expr),* $(,)?) => {
        axum::Router::new()$(
         .merge($route)
        )*
    };
}

#[cfg(test)]
mod tests {
    use axum::extract::State;
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
    fn test_nested_router_with_state() {
        router!(
            "/simplify",
            routes!(
                get "/:exp" => || async {},
                get "/table/:exp" => |_state: State<String>| async {}
            ),
            String
        );
    }

    #[test]
    fn test_nested_router_with_generic_state() {
        router!(
            "/simplify",
            routes!(
                get "/:exp" => || async {},
                get "/table/:exp" => |_state: State<T>| async {}
            ),
            T: Clone, Send, Sync
        );
    }

    #[test]
    fn test_routes() {
        let _router: Router = routes!(
            get "/" => index,
            post "/" => || async {}
        );
    }

    #[test]
    fn test_join_routes() {
        let _router: Router = join_routes![Router::new(), Router::new()];
    }
}
