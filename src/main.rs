use crate::todo::Todo;
use std::io::{self, stdout};
use tokio_postgres::{NoTls};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{ExecutableCommand,
                terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}},
    Terminal
};
mod todo;
mod manage;
mod console;


#[tokio::main]
async fn main() -> io::Result<()>{
    stdout().execute(EnterAlternateScreen)?; //Execute alternate screen
    enable_raw_mode()?; // Raw mode - мод для того, чтобы текс выводился не в обычный терминал
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=Zdfpdhq3", NoTls)
            .await.expect("could not connect to database");
    tokio::spawn(async move { // Drop connection for checking connection
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut todo = Todo::new(client);

    loop {
        terminal.draw(|frame| {

        })?;
    }


    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}