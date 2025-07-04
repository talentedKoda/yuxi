use api::{setup_config, setup_db, setup_router};
use app::config::Config;

async fn worker(config: Config, listener: std::net::TcpListener) {
    tracing::info!("Worker started");

    let conn = setup_db(&config.db_url).await;

    utils::migrate(&conn).await.expect("Migration failed!");

    let router = setup_router(config, conn);
    let listener = tokio::net::TcpListener::from_std(listener).expect("bind to port");
    axum::serve(listener, router).await.expect("start server");
}

fn run_in_tokio_runtime() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let config = setup_config();
        let listener = std::net::TcpListener::bind(config.get_server_url()).expect("bind to port");
        listener.set_nonblocking(true).expect("non blocking failed");
        tracing::debug!("listening on http://{}", listener.local_addr().unwrap());

        worker(config, listener).await;
    });
}

pub fn run() {
    run_in_tokio_runtime();
}
