use actix_web::web;
use crate::model::blog::{Blog, BlogCollection};
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, SearchParts};
use elasticsearch::indices::{IndicesCreateParts, IndicesExistsParts};
use serde_json::{json, Value};
use reqwest;
use crate::api::blog::{BlogRequest, BlogTitleRequest};

pub struct RepositoryBuilder;

impl RepositoryBuilder {
    pub fn build() -> Repository {
        let uri_base = "http://localhost";
        let port = ":9200";
        let uri = format!("{}{}", uri_base, port);
        let transport = Transport::single_node(uri.as_str()).unwrap();
        let client = Elasticsearch::new(transport);
        Repository { client }
    }
}

pub struct Repository {
    client: Elasticsearch,
}

impl Repository {
    pub async fn init_blogpost_index(&self) -> String {
        let exists_val = self.check_if_blogpost_index_exists().await;
        if exists_val {
            return "Exists".to_string();
        }

        self.create_blogpost_index().await
    }

    async fn create_blogpost_index(&self) -> String {
        let response = self.client
            .indices()
            .create(IndicesCreateParts::Index("blogpost"))
            .body(json!({
                "mappings" : {
                    "properties" : {
                        "blog_id" : { "type" : "integer" },
                        "blog_title" : { "type" : "text" },
                        "blog_name" : { "type" : "text" },
                    }
                }
            }))
            .send()
            .await;

        match response {
            Ok(res) => {
                match res.status_code() {
                    reqwest::StatusCode::OK => {
                        let body = res.text().await.unwrap();
                        println!("Body:\n{}", body);
                        body
                    }
                    reqwest::StatusCode::UNAUTHORIZED => "Get a new token".to_string(),
                    _ => format!("bad res status {:?}", res.status_code())
                }
            }
            Err(e) => e.to_string(),
        }
    }

    async fn check_if_blogpost_index_exists(&self) -> bool {
        let does_exist = self.client
            .indices()
            .exists(IndicesExistsParts::Index(&["blogpost"]))
            .send()
            .await;

        let exists_val = match does_exist {
            Ok(res) => {
                match res.status_code() {
                    reqwest::StatusCode::OK => { true }
                    _ => false
                }
            }
            Err(_) => false,
        };
        exists_val
    }

    pub async fn blog_search_by_title(&self, blog_request: web::Json<BlogTitleRequest>) -> BlogCollection {
        let search_response = self.client
            .search(SearchParts::Index(&["blogpost"]))
            .from(blog_request.from.clone() as i64)
            .size(blog_request.size.clone() as i64)
            .body(json!({
                "query": {
                    "match": {
                        "blog_title": blog_request.blog_title.clone()
                    }
                }
            }))
            .send()
            .await;

        let mut blog_collection = BlogCollection::init();
        let _search_response_match: bool = match search_response {
            Ok(res) => {
                match res.status_code() {
                    reqwest::StatusCode::OK => {
                        let body = res.text().await.unwrap();
                        println!("Body:\n{}", body);
                        let body_json: Value = serde_json::from_str(body.as_str()).expect("unable to parse");
                        let total_hits = body_json["hits"]["total"]["value"].clone().as_f64().unwrap();
                        if total_hits <= 0.0 { return blog_collection; }
                        for (index, _all_question_mark) in body_json["hits"]["hits"].as_array().iter().enumerate() {
                            if body_json["hits"]["hits"][index]["_source"].is_object() {
                                blog_collection.blogs.push(
                                    Blog {
                                        blog_id: None,
                                        blog_id_str: Some(body_json["hits"]["hits"][index]["_source"]["blog_id"].to_string()),
                                        blog_title: body_json["hits"]["hits"][index]["_source"]["blog_title"].to_string(),
                                        blog_name: body_json["hits"]["hits"][index]["_source"]["blog_name"].to_string()
                                    }
                                )
                            }
                        }
                        return blog_collection;
                    },
                    _ => { blog_collection.errors.push("Bad request status code: ".to_string() + res.status_code().clone().as_str()); return blog_collection; }
                }
            },
            Err(e) => { blog_collection.errors.push(e.to_string()); return blog_collection; }
        };
    }

    // had to make a request here since adding with post not yet implemented in elasticsearch 8.5.0-alpha.1
    // post is the one where you dont have to specify id
    pub async fn blog_add(&self, blog_request: web::Json<BlogRequest>) -> String {
        let post_doc_link = "http://localhost:9200/blogpost/_doc";
        let blog_json_request = json!({
            "blog_id": blog_request.blog_id.clone(),
            "blog_name": blog_request.blog_name.clone(),
            "blog_title": blog_request.blog_title.clone(),
        });
        let client = reqwest::Client::new();
        let post_res = client.post(post_doc_link)
            .body(blog_json_request.to_string())
            .header("Content-Type", "application/json")
            .send()
            .await;

        match post_res {
            Ok(res) => {
                println!("Status: {}", res.status());
                println!("Headers:\n{:#?}", res.headers());
                match res.status() {
                    reqwest::StatusCode::CREATED => {
                        let body = res.text().await.unwrap();
                        println!("Body:\n{}", body);
                        let body_json: Value = serde_json::from_str(body.as_str()).unwrap();
                        if body_json["result"] == "created" {
                            return body_json["_id"].to_string();
                        }
                        "could not create".to_string()
                    }
                    _ => format!("bad res status {:?}: {:?}", res.status(), res.text().await.unwrap())
                }
            },
            Err(_) => "could not find the server".to_string()
        }
    }

    pub async fn blog_list(&self) -> BlogCollection {
        return BlogCollection::from_vec(vec![
            Blog { blog_id: Some(1), blog_id_str: None, blog_name: "blog_name_2".to_string(), blog_title: "blog_title".to_string() },
            Blog { blog_id: Some(2), blog_id_str: None, blog_name: "blog_name_4".to_string(), blog_title: "blog_title".to_string() },
            Blog { blog_id: Some(3), blog_id_str: None, blog_name: "blog_name_3".to_string(), blog_title: "blog_title".to_string() },
        ]);
    }
}
