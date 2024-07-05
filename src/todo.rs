use tokio_postgres::Client;
use crate::manage;
use crate::manage::{add_task_to_db, remove_task, update_task};
use crate::todo::task::Task;

pub mod task;
pub struct Todo {
    pub tasks: Vec<Task>,
    pub client: Client
}

impl Todo{
    pub fn new(client: Client) -> Self{
        Self{
            tasks: Vec::new(),
            client
        }
    }

    pub async fn get_from_db(&mut self) -> Result<(), tokio_postgres::Error> {
        let rows = self.client.query("SELECT * FROM tasks",&[]).await?;
        for row in rows.iter() {
            let task = Task{ text: row.get(1), status: row.get::<usize, bool>(2).into()};
            self.tasks.push(task);
        }
        Ok(())
    }

    pub fn list_tasks(&self) {
        for task in self.tasks.iter() {
            println!("{}", task);
        }
    }

    pub async fn add_task(&mut self, text: &String) {
        let task = Task::new(text.to_string());
        let temp = add_task_to_db(&task, &self.client).await;
        print!("Произошло добавление");
        if let Err(e) = temp {
            eprintln!("{}", e);
        }
        self.tasks.push(task);
    }

    pub async fn edit_task(&mut self,task: &Task,status: bool) {
        for i in 0..self.tasks.len() {
            if self.tasks[i].text == task.text {
                self.tasks[i].status = status;
                let temp = update_task(&self.tasks[i], &self.client).await;
                if let Err(e) = temp {
                    eprintln!("{}", e);
                }
            }
        }
    }

    pub async fn remove_task(&mut self, task: &Task) {
        for i in 0..self.tasks.len() {
            if self.tasks[i].text == task.text {
                let temp = remove_task(&self.tasks[i], &self.client).await;
                self.tasks.remove(i);
                break;
            }
        }
    }

    pub async fn clear(&mut self) {
        self.tasks = Vec::new();
        let temp = manage::clear_db(&self.client).await;
        if let Err(e) = temp {
            eprintln!("{}", e);
        }
    }
}