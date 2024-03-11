use actix_web::{ App, HttpServer};
use std::sync::Arc;
use infrastructure::db::Pg;
use std::env;

/// Start the server
#[cfg(not(tarpaulin_include))]
pub async fn start_server(pg: Arc<Pg>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .app_data::<infrastructure::state::AppState>(infrastructure::state::initialize())
        .configure(|cfg| crate::application::configure(Arc::clone(&pg),cfg))
    })
    .bind((env::var("HOST").unwrap(), 8080))?
    .run()
    .await
}
