use reqwest;
use crate::model::task::{Task, TaskCollection};

pub struct SpacetimeRepository {
    db_uri: String
}

impl SpacetimeRepository {
    pub fn init() -> SpacetimeRepository {
        let uri_base = "http://rbassoon_spacetime";
        let port = ":80";
        let uri = format!("{}{}/database", uri_base, port);
        SpacetimeRepository{db_uri: uri}
    }

    pub async fn ping(&self) -> Result<String, String> {
        let database_ping = self.db_uri.clone() + "/ping";
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
                    _ => Err(format!("bad res status {:?}", res.status()))
                }
            },
            Err(_) => Err("could not find the server".to_string())
        };
    }

    pub async fn say_hello(&self) -> Result<String, String> {
        let client = reqwest::Client::new();
        let say_hello_uri = self.db_uri.clone() + "call/repository/say_hello";
        return match client.post(say_hello_uri).send().await {
            Ok(res) => {
                println!("Status: {}", res.status());
                println!("Headers:\n{:#?}", res.headers());
                match res.status() {
                    reqwest::StatusCode::OK => {
                        let body = res.text().await.unwrap();
                        println!("Body:\n{}", body);
                        Ok(body)
                    },
                    _ => Err(format!("bad res status {:?}", res.status()))
                }
            },
            Err(_) => Err("could not find the server".to_string())
        };
    }

    pub async fn task_list(&self) -> TaskCollection {
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
