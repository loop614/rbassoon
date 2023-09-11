mod api;
mod model;
mod persistence;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::blog::{blog_add, blog_list, es_init, blog_search_by_title};

use crate::persistence::repository::{Repository, RepositoryBuilder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    HttpServer::new(move || {
        let elastic: Repository = RepositoryBuilder::build();
        let elastic_data: Data<Repository> = Data::new(elastic);
        let logger: Logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(elastic_data)
            .service(es_init)
            .service(blog_add)
            .service(blog_list)
            .service(blog_search_by_title)

    })
    .workers(1)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
