use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Stylize, Widget};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Wrap};
use ratatui::crossterm::event::MouseEvent;
use ratatui::style::Color;
use tokio_postgres::Client;
use crate::database;
use crate::database::Database;
use crate::todo::task::Task;

pub mod task;
#[derive(Clone)]
pub struct Todo {
    pub tasks: Vec<Task>,
    pub state: ListState,
    pub should_exit: bool
}

impl Todo{
    pub fn new() -> Self{
        Self{
            tasks: Vec::new(),
            state: ListState::default(),
            should_exit: false
        }
    }

    pub async fn get_from_db(&mut self, db: &Database) -> Result<(), tokio_postgres::Error> {
        let rows = db.query("SELECT * FROM tasks",&[]).await?;
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

    pub async fn add_task(&mut self, text: &String, db: &Database) {
        let task = Task::new(text.to_string());
        let temp = db.add_task_to_db(&task).await;
        print!("Произошло добавление");
        if let Err(e) = temp {
            eprintln!("{}", e);
        }
        self.tasks.push(task);
    }

    pub async fn edit_task(&mut self,task: &Task,status: bool, db: &Database) {
        for i in 0..self.tasks.len() {
            if self.tasks[i].text == task.text {
                self.tasks[i].status = status;
                let temp = db.update_task(&self.tasks[i]).await;
                if let Err(e) = temp {
                    eprintln!("{}", e);
                }
            }
        }
    }

    pub async fn remove_task(&mut self, task: &Task, db: &Database) {
        for i in 0..self.tasks.len() {
            if self.tasks[i].text == task.text {
                let temp = db.remove_task(&self.tasks[i]).await;
                self.tasks.remove(i);
                break;
            }
        }
    }

    pub async fn clear(&mut self, db: &Database) {
        self.tasks = Vec::new();
        let temp = db.clear_db().await;
        if let Err(e) = temp {
            eprintln!("{}", e);
        }
    }
    fn render_list(&mut self, area: Rect,buf: &mut Buffer)
    {
        let block = Block::new()
            .title(Line::raw("TODO list").centered())
            .borders(Borders::TOP);
        let items: Vec<ListItem> = self
                                .tasks
                                .iter().map(|task| ListItem::from(task)).collect();
        let list = List::new(items)
            .block(block)
            .on_green()
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(list,area,buf,&mut self.state)
    }

    pub async fn switch_status(&mut self, db: &Database) {
        if let Some(i) = self.state.selected()
        {
            self.tasks[i].status = match self.tasks.get(i).unwrap().status {
                false => true,
                true => false
            };
            db.update_task(&self.tasks[i]).await;
        }
    }

    pub fn render_selected_task(& self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.state.selected() {
            match self.tasks.get(i).unwrap_or(&Task::new("End task".to_string())).status {
                true => format!("✓ DONE: {}", self.tasks[i].text),
                false => format!("☐ TODO: {}", self.tasks[i].text)
            }
        } else { "Nothing selected...".to_string() };

        let block = Block::new()
            .title("Todo info")
            .borders(Borders::TOP)
            .border_set(ratatui::symbols::border::EMPTY)
            .border_style(Color::Blue)
            .padding(Padding::ZERO);

        Paragraph::new(info)
            .block(block)
            .wrap(Wrap{trim:false})
            .render(area,buf);
    }
}

impl Widget for &mut Todo {
    fn render(mut self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let [main_area] = Layout::vertical([Constraint::Fill(1)]).areas(area);
        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1),Constraint::Fill(1)])
            .areas(main_area);
        self.render_list(list_area,buf);
        self.render_selected_task(item_area,buf);
    }
}

