#![allow(dead_code)]
#![allow(unused_imports)]
mod app;
mod cli;
mod config;
mod event;

#[macro_use]
mod log;

use crate::cli::parse;
#[allow(unused_imports)]
use anyhow::Result;
use std::os::unix::process::CommandExt;

use std::{error::Error, io, process::Command, time::Duration};

use crate::config::Connection;
use tui::{backend::CrosstermBackend, Terminal};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::app::*;

fn run_psql(conn_str: String) -> anyhow::Result<()> {
    let _child = Command::new("psql")
        .arg(conn_str)
        // .stderr(std::process::Stdio::null()) // don't care about stderr
        // .stdout(std::process::Stdio::piped())
        // .stdin(std::process::Stdio::piped()) // set up stdin so we can write on it
        .spawn()
        .expect("Could not run the command")
        .wait();

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let value = parse();
    let config = config::Config::new(&value.config)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new(config.conn);
    let res = run_app(&mut terminal, app, tick_rate)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    run_psql(res)?;

    Ok(())
}
