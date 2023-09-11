use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlogCollection {
    pub blogs: Vec<Blog>,
    pub errors: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Blog {
    pub blog_id: Option<u64>,
    pub blog_id_str: Option<String>,
    pub blog_name: String,
    pub blog_title: String,
}

impl BlogCollection {
    pub fn init() -> BlogCollection {
        return BlogCollection { blogs: vec![], errors: vec![] };
    }

    pub fn from_vec(tasks_vec: Vec<Blog>) -> BlogCollection {
        let mut task_collection = BlogCollection::init();
        for task_vec in tasks_vec {
            task_collection.blogs.push(task_vec);
        }

        return task_collection;
    }
}
