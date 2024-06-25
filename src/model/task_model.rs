use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: Uuid,
    user_id: Uuid,
    title: String,
    done: bool,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tasks(BTreeMap<Uuid, TaskInput>);

impl Tasks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, task: TaskInput) {
        self.0.insert(Uuid::new_v4(), task);
    }

    pub fn remove_specific_task_by_id(&mut self, user_id: Uuid, id: Uuid) -> Option<TaskInput> {
        let task = self.0.get(&id);
        match task {
            Some(t) => {
                if t.user_id == user_id {
                    self.0.remove(&id)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn get_all_tasks_by_user_id(&self, user_id: Uuid) -> Vec<Task> {
        let mut task_list_by_user_id = Vec::new();
        for (k, v) in &self.0 {
            if v.user_id == user_id {
                task_list_by_user_id.push(Task {
                    id: *k,
                    user_id,
                    title: v.title.to_string(),
                    done: v.done,
                })
            }
        }
        task_list_by_user_id
    }
    pub fn get_specific_tasks_by_user_id(&self, user_id: Uuid, task_id: Uuid) -> Option<TaskInput> {
        let data = self.0.get(&task_id);
        match data {
            Some(t) => {
                if t.user_id == user_id {
                    Some(t.clone())
                } else {
                    None
                }
            }
            None => None,
        }
    }
    pub fn update_specific_tasks_by_user_id(
        &mut self,
        user_id: Uuid,
        task_id: Uuid,
        task: TaskInput,
    ) -> Option<Uuid> {
        match self.0.get_mut(&task_id) {
            Some(data) => {
                if data.user_id == user_id {
                    *data = task;
                    Some(task_id)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TaskInput {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub title: String,
    pub user_id: Uuid,
    pub done: bool,
}
