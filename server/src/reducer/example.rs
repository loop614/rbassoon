use crate::transfer::task::{Task, TaskCollection};

pub struct MockedRepository;

impl MockedRepository {
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
