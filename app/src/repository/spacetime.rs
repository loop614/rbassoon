use reqwest;
use crate::model::task::{Task, TaskCollection};

pub struct SpacetimeRepository {}

impl SpacetimeRepository {
    pub fn init() -> SpacetimeRepository {
        SpacetimeRepository{}
    }

    pub async fn ping(&self) -> Result<String, Box<dyn std::error::Error>> {
        let res = reqwest::get("`http://rbassoon_spacetime:80/database/ping`").await?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        let body = res.text().await?;
        println!("Body:\n{}", body);

        Ok(body)
    }

    pub async fn task_list(&self) -> TaskCollection {
        return TaskCollection::from_vec(vec![
            Task{task_id: 1, task_name: "task_name_1".to_string()},
            Task{task_id: 2, task_name: "task_name_2".to_string()},
            Task{task_id: 3, task_name: "task_name_3".to_string()},
        ]);
    }

    pub async fn task_add(&self, task_name: String) -> Task {
        return Task{task_id: 1, task_name };
    }
}
