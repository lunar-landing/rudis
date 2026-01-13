use rudis_server::args::Args;
use rudis_server::server::Server;
use rudis_server::web::WebServer;
use rudis_server::store::db_manager::DatabaseManager;
use std::process::id;
use std::sync::Arc;

#[tokio::main]
async fn main() {

    let args = Arc::new(Args::load());
    unsafe { std::env::set_var("RUST_LOG", &args.loglevel); };
    env_logger::init();

    server_info(args.clone());
    let db_manager = Arc::new(DatabaseManager::new(args.clone()));
    let web_server = WebServer::new(args.clone(), db_manager.clone());
    let mut server = Server::new(args.clone(), db_manager);

    tokio::select! {
        _ = web_server.start(args.webport) => {
            log::error!("Web server stopped unexpectedly");
        }
        _ = server.start() => {
            log::error!("server stopped unexpectedly");
        }
    }
}

fn server_info(args: Arc<Args>) {
    let pid = id();
    let version = env!("CARGO_PKG_VERSION");
    let role = if args.is_slave() { "slave" } else { "master" };
    let pattern = format!(r#"
         /\_____/\
        /  o   o  \          Rudis {}
       ( ==  ^  == )
        )         (          Bind: {} PID: {}
       (           )
      ( (  )   (  ) )        Role: {}
     (__(__)___(__)__)

    Rudis is a high-performance in memory database.

    Web UI: http://127.0.0.1:{}
    "#, version, args.port, pid, role, args.webport);
    println!("{}", pattern);
}