use reqwest;
use crate::model::task::{Task, TaskCollection};

pub struct SpacetimeRepository {}

impl SpacetimeRepository {
    pub fn init() -> SpacetimeRepository {
        SpacetimeRepository{}
    }

    pub async fn ping(&self) -> Result<String, String> {
        let database_ping = "http://localhost:6868/database/ping";
        return match reqwest::get(database_ping).await {
            Ok(res) => {
                println!("Status: {}", res.status());
                println!("Headers:\n{:#?}", res.headers());
                match res.status() {
                    reqwest::StatusCode::OK => {
                        let body = res.text().await.unwrap();
                        println!("Body:\n{}", body);
                        Ok(body)
                    },
                    reqwest::StatusCode::UNAUTHORIZED => Err("Get a new token".to_string()),
                    _ => Err(format!("bad res status {:?}", res.status()))
                }
            },
            Err(_) => Err("could not find the server".to_string())
        };
    }

    pub async fn task_list(&self) -> TaskCollection {
        println!("Hardcoded");
        return TaskCollection::from_vec(vec![
            Task{task_id: 1, task_name: "task_name_2".to_string()},
            Task{task_id: 2, task_name: "task_name_4".to_string()},
            Task{task_id: 3, task_name: "task_name_3".to_string()},
        ]);
    }

    pub async fn task_add(&self, task_name: String) -> Task {
        return Task{task_id: 1, task_name };
    }
}
