use color_eyre::Result;

mod app;
mod level;
mod skill;
mod stat;
mod skill_levels;
mod components;
mod types;
mod traits;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}
