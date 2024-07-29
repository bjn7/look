use crate::tui::App;

pub trait Command {
    fn quit(&mut self);
}

impl Command for App {
    fn quit(&mut self) {
        self.exit = true
    }
}
