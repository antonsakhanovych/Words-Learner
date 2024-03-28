mod app;
mod errors;
mod tui;

use color_eyre::{eyre::OptionExt, Result};
use std::{env, mem};

fn parse_args() -> Option<String> {
    let mut args: Vec<String> = env::args().collect();
    if let Some(arg) = args.get_mut(1) {
        Some(mem::take(arg))
    } else {
        None
    }
}

fn main() -> Result<()> {
    errors::install_hooks()?;
    let file_path = parse_args().ok_or_eyre("ERROR: Please provide file path!")?;
    let mut terminal = tui::enable()?;
    terminal.clear()?;

    let mut app = app::App::new(&file_path)?;
    app.run(&mut terminal)?;

    tui::restore()?;
    Ok(())
}
