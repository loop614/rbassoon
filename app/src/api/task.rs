use actix_web::{
    get,
    post,
    web,
};

use serde::{Serialize, Deserialize};
use crate::model::task::{Task, TaskCollection};
use crate::repository::spacetime::SpacetimeRepository;

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_id: u64,
}

#[derive(Deserialize, Serialize)]
pub struct TaskRequest {
    task_name: String,
}

#[get("/task/list")]
pub async fn task_list(
    repository: web::Data<SpacetimeRepository>
) -> web::Json<String> {
    let task_list: TaskCollection = repository.task_list().await;
    let tasks_string = serde_json::to_string(&task_list).unwrap();

    return web::Json(tasks_string);
}


#[get("/task/spacetime")]
pub async fn task_spacetimeping(
    repository: web::Data<SpacetimeRepository>
) -> web::Json<String> {
    let ping: String = repository.ping().await.unwrap();

    return web::Json(ping);
}


#[get("/task/spacetime_hello")]
pub async fn task_spacetimehello(
    repository: web::Data<SpacetimeRepository>
) -> web::Json<String> {
    let ping: String = repository.say_hello().await.unwrap();

    return web::Json(ping);
}

#[post("/task/add")]
pub async fn task_add(
    repository: web::Data<SpacetimeRepository>,
    request: web::Json<TaskRequest>
) -> web::Json<String> {
    let task: Task = repository.task_add(request.task_name.clone()).await;

    return web::Json(task.task_id.to_string());
}
