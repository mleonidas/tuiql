use crate::config::Connection;
use crossterm::event::{self, Event, KeyCode};
use std::fs::File;

use std::{
    io,
    process::Command,
    time::{Duration, Instant},
};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

struct StatefulList {
    state: ListState,
    items: Vec<Connection>,
}

impl StatefulList {
    fn with_items(items: Vec<Connection>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn enter(&mut self) -> Result<String, io::Error> {
        let _i = match self.state.selected() {
            Some(i) => {
                let conn = self.items[i].clone();
                let db_url = conn.database_url();
                let s = db_url.unwrap();
                return Ok(s);
            }
            None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "foo")),
        };
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App {
    items: StatefulList,
}

impl App {
    pub fn new(config: Vec<Connection>) -> App {
        App {
            items: StatefulList::with_items(config),
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<String> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "we quit the interactive console",
                        ))
                    }
                    KeyCode::Left => app.items.unselect(),
                    KeyCode::Char('j') => app.items.next(),
                    KeyCode::Char('k') => app.items.previous(),
                    KeyCode::Enter => return app.items.enter(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let db = i.database.clone().expect("this should not happen");
            let lines = vec![Spans::from(db)];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::LightMagenta))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Database Connections"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}
