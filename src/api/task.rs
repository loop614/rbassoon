use actix_web::{
    get,
    post,
    HttpRequest,
    web,
};

use serde::{Serialize, Deserialize};
use crate::model::task::{Task, TaskCollection};
use crate::repository::ddb::DDBRepository;

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_id: u64,
}

#[derive(Deserialize, Serialize)]
pub struct TaskRequest {
    task_name: String,
}

#[post("/{name}")]
async fn index(req: HttpRequest) -> web::Json<String> {
    web::Json(req.match_info().get("name").unwrap().to_owned())
}

#[get("/task/list")]
pub async fn task_list(
    repository: web::Data<DDBRepository>
) -> web::Json<String> {
    let task_list: TaskCollection = repository.task_list().await;
    let tasks_string = serde_json::to_string(&task_list).unwrap();

    return web::Json(tasks_string);
}

#[post("/task/add")]
pub async fn task_add(
    repository: web::Data<DDBRepository>,
    request: web::Json<TaskRequest>
) -> web::Json<String> {
    let task: Task = repository.task_add(request.task_name.clone()).await;

    return web::Json(task.task_id.to_string());
}
