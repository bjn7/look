use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;

use crate::tui::App;
use crate::NavigationDataFeild;
// use std::fs;
use std::path::PathBuf;

pub trait Utility {
    fn get_selected(&self) -> &NavigationDataFeild;
    fn get_path(&self) -> &PathBuf;
    fn get_formated_display_path(nav_field: &NavigationDataFeild) -> ListItem;
}

impl<'a> Utility for App<'a> {
    fn get_selected(&self) -> &NavigationDataFeild {
        &self.field[self.state.selected().unwrap_or(0)]
    }
    fn get_path(&self) -> &PathBuf {
        &self.get_selected().path
    }

    // The  function changed quite a bit because of my fierce battle with the borrow checker 😮‍💨
    // so, to win this battle i had to use my trump card static fn 🃏

    // too random and abit cringe maybe..💀

    // As the checker moved through each line of my code,
    // verifying ownership and borrowing, the compiler knew
    // it was almost time to compile.
    // However, it couldn't do this without passing the ownership and borrowing checks.
    // thus, the linker asked, "Are you the programmer because you are the software enginer, or
    // are you the software enginer because you are the programmer?"
    // Before I used my domain expansion and cursed technique, static function,
    // Rust questioned me, "Can you still write error-free and memory-safe code?"
    // I replied, "If I can't pass the borrow check, I might have a little trouble."

    // "But would you lose?"

    // "NAH, I'D WIN."

    fn get_formated_display_path(nav_field: &NavigationDataFeild) -> ListItem {
        let path = &nav_field.path.to_string_lossy().to_string();
        // fs::write("log.txt", format!("{:#?}", nav_field)).unwrap();
        let left_side = Span::styled(path[..nav_field.start].to_string(), Style::default());
        let middle_side = Span::styled(
            path[nav_field.start..nav_field.end].to_string(),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        );
        let right_side = Span::styled(path[nav_field.end..].to_string(), Style::default());
        let x = Line::from(vec![left_side, middle_side, right_side]);
        ListItem::from(x)
    }
}
