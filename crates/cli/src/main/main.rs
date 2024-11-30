mod view;
mod store;
mod action;
mod component;
mod tui;
mod config;
mod logging;
mod errors;
mod cli;

use std::io;

use ratatui::style::Stylize;
use crate::view::tutorial::Tutorial;

fn main() -> io::Result<()> {
	let mut terminal = ratatui::init();
	let app_result = Tutorial::default().run(&mut terminal);
	ratatui::restore();
	app_result
}