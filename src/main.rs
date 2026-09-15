use color_eyre::Result;

mod app;
mod level;
mod skill;
mod stat;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}
