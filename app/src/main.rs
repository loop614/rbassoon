mod api;
mod model;
mod repository;

use api::task::{
    task_list,
    task_add,
    task_spacetimeping
};

use crate::repository::spacetime::SpacetimeRepository;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let spacetime: SpacetimeRepository = SpacetimeRepository::init();
        let spacetime_data: Data<SpacetimeRepository> = Data::new(spacetime);
        let logger: Logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(spacetime_data)
            .service(task_spacetimeping)
            .service(task_add)
            .service(task_list)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
