use ratatui::crossterm::style::Stylize;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;

use crate::tui::App;
use crate::NavigationDataFeild;
use std::collections::HashMap;
use std::env::args as arguments;
// use std::fs;
use std::path::PathBuf;

pub trait Utility {
    fn get_selected(&self) -> &NavigationDataFeild;
    fn get_path(&self) -> &PathBuf;
    fn get_formated_display_path(nav_field: &NavigationDataFeild) -> ListItem;
}

pub fn cmd_help(cmd_list: &HashMap<&str, (&str, FlagsEnum)>) {
    println!(
        "{} {} {}\n",
        "USAGE:".bold().red(),
        "look run".bold().blue(),
        "[OPTIONS] [ARGS]".blue()
    );
    println!(" Arguments:\n");
    for v in cmd_list.iter() {
        println!("\t{}:\t{}", v.0.blue(), v.1 .0)
    }
    println!("\n");
    std::process::exit(2);
}

pub enum FlagsEnum {
    All,
    Case,
    Dir,
    File,
}

pub struct Flags {
    pub all: bool,
    pub case_sensitive: bool,
    pub dir: bool,
    pub file: bool,
    pub sub_str: String,
}

pub fn get_args() -> Flags {
    let command_list: HashMap<&str, (&str, FlagsEnum)> = HashMap::from([
        ("-all", ("Search in all directories.", FlagsEnum::All)),
        (
            "-case",
            ("Perform a case-sensitive search.", FlagsEnum::Case),
        ),
        ("-dir", ("Search for directories only.", FlagsEnum::Dir)),
        ("-file", ("Search for files only.", FlagsEnum::File)),
    ]);

    let args = arguments().collect::<Vec<String>>();
    if args.len() <= 1 {
        cmd_help(&command_list);
    };
    let mut flags = Flags {
        all: false,
        case_sensitive: false,
        dir: false,
        file: false,
        sub_str: String::new(),
    };

    for arg in args.iter().skip(1) {
        match command_list.get(arg.as_str()) {
            Some((_, FlagsEnum::All)) => flags.all = true,
            Some((_, FlagsEnum::Case)) => flags.case_sensitive = true,
            Some((_, FlagsEnum::Dir)) => flags.dir = true,
            Some((_, FlagsEnum::File)) => flags.file = true,
            None => {
                if arg.starts_with('-') {
                    println!(
                        "{}: unexpected argument: '{}'\n",
                        "Error".bold().red(),
                        arg.clone().bold().red()
                    );
                    cmd_help(&command_list);
                }
            }
        }
    }

    flags.sub_str = args
        .last()
        .clone()
        .expect("substring not provided")
        .to_string();

    return flags;
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
