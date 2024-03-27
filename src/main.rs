mod app;
mod errors;
mod tui;

use color_eyre::Result;

const FILE_PATH: &'static str = "words.json";

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::enable()?;
    terminal.clear()?;

    let mut app = app::App::new(FILE_PATH)?;
    app.run(&mut terminal)?;

    tui::restore()?;
    Ok(())
}
