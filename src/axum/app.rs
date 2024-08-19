use {
    axum::{
        extract::Request, handler::Handler, response::IntoResponse, routing::Route, Router,
        ServiceExt,
    },
    std::{
        convert::Infallible,
        io,
        net::{IpAddr, Ipv4Addr, SocketAddr},
    },
    tokio::net::TcpListener,
    tower::{layer::Layer, Service},
    tower_http::{
        cors::CorsLayer,
        normalize_path::NormalizePathLayer,
        trace,
        trace::{HttpMakeClassifier, TraceLayer},
    },
    tracing::{info, Level},
};

// TODO trim trailing slash into macro > let _app = NormalizePathLayer::trim_trailing_slash().layer(create_app!(routes));
#[macro_export]
macro_rules! create_app {
    ($router:expr) => {
        $router
    };
    ($router:expr, $($layer:expr),* $(,)?) => {
        $router$(.layer($layer))*
    };
}

#[derive(Default)]
pub struct AppBuilder {
    router: Router,
    socket: Option<(IpAddr, u16)>,
    cors: Option<CorsLayer>,
    normalize_path: Option<bool>,
    tracing: Option<TraceLayer<HttpMakeClassifier>>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn route(mut self, route: Router) -> Self {
        self.router = self.router.merge(route);
        self
    }

    pub fn routes(mut self, routes: impl IntoIterator<Item = Router>) -> Self {
        self.router = routes.into_iter().fold(self.router, Router::merge);
        self
    }

    /// Adds a layer to the previously added routes
    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + 'static,
        L::Service: Service<Request> + Clone + Send + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        self.router = self.router.layer(layer);
        self
    }

    pub fn socket<IP: Into<IpAddr>>(mut self, socket: impl Into<(IP, u16)>) -> Self {
        let (ip, port) = socket.into();
        self.socket = Some((ip.into(), port));
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.socket = if let Some((ip, _)) = self.socket {
            Some((ip, port))
        } else {
            Some((Ipv4Addr::UNSPECIFIED.into(), port))
        };
        self
    }

    pub fn fallback<H, T>(mut self, fallback: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.fallback(fallback);
        self
    }

    pub fn cors(mut self, cors: CorsLayer) -> Self {
        self.cors = Some(cors);
        self
    }

    pub fn normalize_path(mut self, normalize_path: bool) -> Self {
        self.normalize_path = Some(normalize_path);
        self
    }

    pub fn tracing(mut self, tracing: TraceLayer<HttpMakeClassifier>) -> Self {
        self.tracing = Some(tracing);
        self
    }

    /// Build the app and start the server
    /// # Default Options
    /// - IP == 0.0.0.0
    /// - Port == 8000
    /// - Cors == None
    /// - Normalize Path == true
    /// - Tracing == Default compact
    pub async fn serve(self) -> io::Result<()> {
        let _ = fmt_trace(); // Allowed to fail
        let listener = self.listener().await?;

        if self.normalize_path.unwrap_or(true) {
            let app = NormalizePathLayer::trim_trailing_slash().layer(self.create_app());
            axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await?;
        } else {
            let app = self.create_app();
            axum::serve(listener, app.into_make_service()).await?;
        };
        Ok(())
    }

    async fn listener(&self) -> io::Result<TcpListener> {
        let addr = SocketAddr::from(self.socket.unwrap_or((Ipv4Addr::UNSPECIFIED.into(), 8000)));
        info!("Initializing server on: {addr}");
        TcpListener::bind(&addr).await
    }

    fn create_app(self) -> Router {
        let mut app = self.router;
        if let Some(cors) = self.cors {
            app = app.layer(cors);
        }
        app.layer(
            self.tracing.unwrap_or(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
    }
}

fn fmt_trace() -> Result<(), String> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .try_init()
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use axum::Router;

    use super::*;

    mod tokio_tests {
        use std::time::Duration;

        use tokio::time::sleep;

        use super::*;

        #[tokio::test]
        async fn test_app_builder_serve() {
            let handler = tokio::spawn(async {
                AppBuilder::new().serve().await.unwrap();
            });
            sleep(Duration::from_millis(250)).await;
            handler.abort();
        }

        #[tokio::test]
        async fn test_app_builder_all() {
            let handler = tokio::spawn(async {
                AppBuilder::new()
                    .socket((Ipv4Addr::LOCALHOST, 8080))
                    .routes([Router::new()])
                    .fallback(|| async { "Fallback" })
                    .cors(CorsLayer::new())
                    .normalize_path(true)
                    .tracing(TraceLayer::new_for_http())
                    .layer(TraceLayer::new_for_http())
                    .serve()
                    .await
                    .unwrap();
            });
            sleep(Duration::from_millis(250)).await;
            handler.abort();
        }
    }

    #[test]
    fn test_create_app_router_only() {
        let _app: Router<()> = create_app!(Router::new());
    }
}
