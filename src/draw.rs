use crate::connections::Model;
use crate::drawing_utils::*;
use crate::events::{Event, EventFunctions};
use crate::utils::fmt_time;
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Paragraph, TableState},
    Terminal,
};

use mpd::Song;

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Songs,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Songs => 1,
        }
    }
}

pub struct QueueTable {
    state: TableState,
    items: Vec<Song>
}

impl QueueTable {
    fn new() -> QueueTable {
        QueueTable {
            state: TableState::default(),
            items: vec![]
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0
        };
        self.state.select(Some(i));
    }
    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0
        };
        self.state.select(Some(i));
    }
}

pub fn draw() -> Result<(), Box<dyn std::error::Error>> {
    let eventfunc = EventFunctions::new(200);
    let mut current_total_time = String::new();
    let mut active_menu_item = MenuItem::Home;
    enable_raw_mode()?;
    let stdout_x = stdout();
    let backend = CrosstermBackend::new(stdout_x);
    let mut terminal = Terminal::new(backend)?;
    execute!(stdout(), EnterAlternateScreen)?;
    terminal.hide_cursor()?;
    let mut table_info = QueueTable::new();
    let mut client = Model::create_conn("127.0.0.1", "6600");
    loop {
        terminal.draw(|rect| {
            let size = rect.size();

            // Splitting the entire window into 5
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Min(3),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(size);

            let (title, _ok) = match active_menu_item {
                MenuItem::Home => ("Playlist", true),
                MenuItem::Songs => ("Browse", true),
            };

            // Rendering the top portion, i.e header
            let volume = client.get_volume();
            render_header(chunks[0], rect, title, volume);

            // Rendering info part
            // render_info(chunks[1], rect);

            let queue = client.queue();

            let is_empty = queue.is_empty();
            table_info.items = queue.clone();

            let table = render_songs(&table_info.items);
            rect.render_stateful_widget(table, chunks[1], &mut table_info.state);
            if is_empty {
                return;
            }

            let bottom = client.get_current();
            let x = if bottom.total == 0 {
                0 as f32
            } else {
                bottom.elapsed as f32 / bottom.total as f32
            };
            let y = (size.width as f32 * x) as usize;
            if y != 0 {
                rect.render_widget(
                    Paragraph::new("=".repeat(y - 1) + ">")
                        .style(Style::default().fg(Color::Green)),
                    chunks[3],
                );
            }
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(10),
                        Constraint::Percentage(70),
                        Constraint::Length(20),
                    ]
                    .as_ref(),
                )
                .split(chunks[2]);
            rect.render_widget(
                create_title(&format!("{}:", bottom.state), false, Alignment::Left, true),
                bottom_chunks[0],
            );
            rect.render_widget(
                create_title(&bottom.title, false, Alignment::Left, false),
                bottom_chunks[1],
            );
            let val = if bottom.is_change {
                fmt_time(bottom.total)
            } else {
                (0, String::new())
            };
            current_total_time = val.1;
            let count = val.0;
            let mut elapsed_res = Vec::new();
            let mut elapsed_new = bottom.elapsed;
            for _ in 0..count {
                let stuff = elapsed_new % 60;
                if stuff >= 10 {
                    elapsed_res.push(stuff.to_string());
                } else {
                    elapsed_res.push(format!("0{}", stuff.to_string()));
                }
                elapsed_new /= 60;
            }
            elapsed_res.reverse();
            let current_time = if current_total_time.is_empty() {
                String::new()
            } else {
                format!("[{}/{}]", elapsed_res.join(":"), current_total_time)
            };
            rect.render_widget(
                create_title(&current_time, false, Alignment::Right, true),
                bottom_chunks[2],
            );
        })?;
        match eventfunc.get()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(stdout(), LeaveAlternateScreen)?;
                    break;
                }
                KeyCode::Down => {
                    table_info.next();
                }
                KeyCode::Up => {
                    table_info.prev();
                }
                KeyCode::Char('1') => active_menu_item = MenuItem::Home,
                KeyCode::Char('2') => active_menu_item = MenuItem::Songs,
                _ => {}
            },
            Event::Tick => {}
        }
    }
    Ok(())
}
