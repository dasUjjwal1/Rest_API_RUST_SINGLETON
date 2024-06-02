use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: Uuid,
    user_id: Uuid,
    title: String,
    done: bool,
}

impl Task {
    pub fn new(title: String, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            done: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tasks(Vec<Task>);

impl Tasks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, task: Task) {
        self.0.push(task);
    }

    pub fn remove_specific_task_by_id(&mut self, user_id: Uuid, id: Uuid) -> Option<Uuid> {
        let index = self
            .0
            .iter()
            .position(|task| task.id == id && task.user_id == user_id);
        match index {
            Some(idx) => {
                self.0.remove(idx);
                Some(id)
            }
            None => None,
        }
    }

    pub fn get_all_tasks_by_user_id(&self, user_id: Uuid) -> Vec<Task> {
        self.0
            .iter()
            .filter(|task| task.user_id == user_id)
            .cloned()
            .collect()
    }
    pub fn get_specific_tasks_by_user_id(&self, user_id: Uuid, task_id: Uuid) -> Option<Task> {
        let data = self
            .0
            .iter()
            .find(|task| task.id == task_id && task.user_id == user_id)
            .cloned();
        data
    }
    pub fn update_specific_tasks_by_user_id(
        &mut self,
        user_id: Uuid,
        task_id: Uuid,
        task: TaskInput,
    ) -> Option<Uuid> {
        let index = self
            .0
            .iter()
            .position(|task| task.id == task_id && task.user_id == user_id);
        match index {
            Some(idx) => {
                self.0[idx].title = task.title;
                Some(task_id)
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInput {
    pub title: String,
}
