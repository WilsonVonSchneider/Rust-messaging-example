use super::domain::SlackMessages;
use super::infrastructure::PgService;
use actix_web::web;
use super::http::*;
use infrastructure::db::Pg;
use std::sync::Arc;


pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = SlackMessages {
        service: PgService { pg_pool },
    };

    cfg.app_data(web::Data::new(service));

    cfg.route(
        "/",
        web::get().to(index)
    );
    cfg.route(
        "/api/{version}/slack_message",
        web::post().to(handle_create::<SlackMessages<PgService>>)
    );
}

