use std::io;

mod tui;
use ratatui::widgets::ListItem;
fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = tui::App::new();
    let items = vec![ListItem::new("hello"),ListItem::new("hello2")];
    app.run(&mut terminal, items)?;
    tui::restore()?;
    Ok(())
}
