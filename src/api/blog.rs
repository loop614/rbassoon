use actix_web::{get, post, web};

use crate::model::blog::{BlogCollection};
use crate::persistence::repository::Repository;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BlogRequest {
    pub blog_id: u64,
    pub blog_name: String,
    pub blog_title: String,
}


#[derive(Deserialize, Serialize)]
pub struct BlogTitleRequest {
    pub blog_title: String,
    pub from: u64,
    pub size: u8,
}

#[get("/blog/list")]
pub async fn blog_list(
    repository: web::Data<Repository>
) -> web::Json<String> {
    let blog_collection: BlogCollection = repository.blog_list().await;
    let blogs_string = serde_json::to_string(&blog_collection).unwrap();

    return web::Json(blogs_string);
}

#[get("/blog/search_by_title")]
pub async fn blog_search_by_title(
    repository: web::Data<Repository>,
    request: web::Json<BlogTitleRequest>
) -> web::Json<String> {
    let blog_collection: BlogCollection = repository.blog_search_by_title(request).await;
    let blogs_string = serde_json::to_string(&blog_collection).unwrap();

    return web::Json(blogs_string);
}

#[post("/blog/add")]
pub async fn blog_add(
    repository: web::Data<Repository>,
    request: web::Json<BlogRequest>,
) -> web::Json<String> {
    let res: String = repository.blog_add(request).await;

    return web::Json(res.to_string());
}

#[get("/es_init")]
pub async fn es_init(
    repository: web::Data<Repository>,
) -> web::Json<String> {
    let res: String = repository.init_blogpost_index().await;

    return web::Json(res);
}
