pub mod app;
pub mod commands;
pub mod models;
pub mod storage;
pub mod theme;
pub mod ui;
pub use app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
