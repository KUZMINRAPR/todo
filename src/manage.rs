use tokio_postgres::{Client};
use crate::todo::task::Task;

pub(crate) async  fn add_task_to_db(task: &Task, client: &Client) -> Result<(),tokio_postgres::Error>{
    client.query("insert into tasks (task, status) values ($1, $2)", &[&task.text,
            &task.status]).await?;
    Ok(())
}

pub(crate) async fn update_task(task: &Task, client: &Client) -> Result<(),tokio_postgres::Error> {
    client.query("update tasks set status = $1 where task = $2",
                &[&task.status, &task.text]).await?;
    Ok(())
}

pub(crate) async fn remove_task(task: &Task,client: &Client) -> Result<(),tokio_postgres::Error> {
    client.query("delete from tasks where task = $1",&[&task.text]).await?;
    Ok(())
}

pub(crate) async fn clear_db(client: &Client) -> Result<(),tokio_postgres::Error> {
    client.query("delete from tasks", &[]).await?;
    Ok(())
}
