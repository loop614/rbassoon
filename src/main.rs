mod api;
mod model;
mod repository;

use api::task::{
    task_list,
    task_add
};

use repository::ddb::DDBRepository;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let repository: DDBRepository = DDBRepository::init();
        let ddb_data: Data<DDBRepository> = Data::new(repository);
        let logger: Logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(task_list)
            .service(task_add)
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await
}
