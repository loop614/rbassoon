use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TaskCollection {
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub task_id: u64,
    pub task_name: String
}

impl TaskCollection {
    pub fn init() -> TaskCollection {
        return TaskCollection{ tasks: vec![] };
    }

    pub fn from_vec(tasks_vec: Vec<Task>) -> TaskCollection {
        let mut task_collection = TaskCollection::init();
        for task_vec in tasks_vec {
            task_collection.tasks.push(task_vec);
        }

        return task_collection;
    }
}
