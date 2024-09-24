use crate::todo::Todo;
use std::io::{self, stdout, Stdout};
use dialoguer::Error;
use tokio_postgres::{NoTls};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{ExecutableCommand,
                terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}},
    Terminal
};

mod todo;
mod database;
mod console;

use database::Database;
use crate::todo::task::Task;

fn start_temrinal() -> io::Result<()> {
    stdout().execute(EnterAlternateScreen)?; //Execute alternate screen
    enable_raw_mode()?; // Raw mode - мод для того, чтобы текс выводился не в обычный терминал
    Ok(())
}
fn cancel_terminal() -> io::Result<()>
{
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
#[tokio::main]
async fn main() -> io::Result<()>{
    start_temrinal().expect("Terminal don't start");
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=Asdfgqwerzxcv dbname=postgres", NoTls)
            .await.expect("could not connect to database");
    tokio::spawn(async move { // Drop connection for checking connection
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut todo = Todo::new();
    let db = Database::new(client);
    todo.get_from_db(&db).await;

    console::write_to_terminal(terminal, todo, &db).await;


    cancel_terminal().expect("Terminal don't cancel");
    Ok(())
}