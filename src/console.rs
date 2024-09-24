use std::io::Stdout;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, List, ListState};
use tokio::io;
use crate::todo::Todo;
use ratatui::prelude::Text;
use crate::database::Database;

pub async fn write_to_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>, mut todo: Todo, db: &Database) -> io::Result<()>
{
    while !todo.should_exit {
        terminal.draw(|frame| frame.render_widget(&mut todo, frame.size()))?;
        if let Event::Key(key) = event::read()? {
            handle_key(&mut todo,key, db).await;
        };
    }
    Ok(())
}

async fn handle_key(todo:&mut Todo, key: KeyEvent,db: &Database) {
    if (key.kind != KeyEventKind::Press) {return};
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => todo.should_exit = true,
        KeyCode::Char('w') | KeyCode::Up => todo.state.select_previous(),
        KeyCode::Char('s') | KeyCode::Down => todo.state.select_next(),
        KeyCode::Char('a') | KeyCode::Char('d') | KeyCode::Left | KeyCode::Right => todo.state.select(None),
        KeyCode::Enter => todo.switch_status(db).await,
        KeyCode::Char('f') | KeyCode::Char('1') => todo.state.select_first(),
        _ => {}
    }
}