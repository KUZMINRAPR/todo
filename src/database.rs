use tokio_postgres::{Client, Row};
use tokio_postgres::types::ToSql;
use crate::todo::task::Task;
pub(crate) struct Database{
    client: Client
}
impl Database {
    pub fn new(client: Client) -> Self {Self{client}}
    pub(crate) async fn add_task_to_db(&self,task: &Task) -> Result<(), tokio_postgres::Error> {
        self.client.query("insert into tasks (task, status) values ($1, $2)", &[&task.text,
            &task.status]).await?;
        Ok(())
    }

    pub(crate) async fn update_task(&self,task: &Task) -> Result<(), tokio_postgres::Error> {
        self.client.query("update tasks set status = $1 where task = $2",
                     &[&task.status, &task.text]).await?;
        Ok(())
    }

    pub(crate) async fn remove_task(&self,task: &Task) -> Result<(), tokio_postgres::Error> {
        self.client.query("delete from tasks where task = $1", &[&task.text]).await?;
        Ok(())
    }

    pub(crate) async fn clear_db(&self) -> Result<(), tokio_postgres::Error> {
        self.client.query("delete from tasks", &[]).await?;
        Ok(())
    }

    pub(crate) async fn query(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, tokio_postgres::Error>
    {
        let rows = self.client.query(query,params).await?;
        Ok(rows)
    }

}