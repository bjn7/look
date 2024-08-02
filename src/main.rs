use std::{
    collections::HashMap,
    env::{self, args},
    fs::read_dir,
    io, process,
};
mod command;
mod tui;
mod utility;
use ratatui::crossterm::style::Stylize;
use tui::NavigationDataFeild;

enum FlagsEnum {
    All,
    Case,
    Dir,
    File,
}

#[derive(Clone, Copy)]
struct Flags {
    all: bool,
    case_sensitive: bool,
    dir: bool,
    file: bool,
}

fn main() -> io::Result<()> {
    let command_list: HashMap<&str, (&str, FlagsEnum)> = HashMap::from([
        ("-all", ("Search in all directories.", FlagsEnum::All)),
        (
            "-case",
            ("Perform a case-sensitive search.", FlagsEnum::Case),
        ),
        ("-dir", ("Search for directories only.", FlagsEnum::Dir)),
        ("-file", ("Search for files only.", FlagsEnum::File)),
    ]);

    let args = args().collect::<Vec<String>>();
    if args.len() <= 1 {
        cmd_help(&command_list);
    }

    let mut flags = Flags {
        all: false,
        case_sensitive: false,
        dir: false,
        file: false,
    };

    let search_substring = if args.len() > 1 {
        for arg in args.iter().skip(1) {
            match command_list.get(arg.as_str()) {
                Some((_, FlagsEnum::All)) => flags.all = true,
                Some((_, FlagsEnum::Case)) => flags.case_sensitive = true,
                Some((_, FlagsEnum::Dir)) => flags.dir = true,
                Some((_, FlagsEnum::File)) => flags.file = true,
                None => {
                    if !arg.starts_with('-') {
                        // Assume it's the search substring
                        break;
                    }
                    println!(
                        "{}: unexpected argument: '{}'\n",
                        "Error".bold().red(),
                        arg.clone().bold().red()
                    );
                    cmd_help(&command_list);
                }
            }
        }
        args.last().cloned().unwrap_or_default()
    } else {
        "".to_string()
    };

    let mut navigation_data: Vec<NavigationDataFeild> = Vec::new();
    let mut terminal = tui::init()?;
    walk_dir(&search_substring, &mut navigation_data, &flags)?;
    if navigation_data.len() < 1 {
        println!("Couldn't find any!")
    }
    let mut app = tui::App::new(&mut navigation_data);
    app.run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}

fn cmd_help(cmd_list: &HashMap<&str, (&str, FlagsEnum)>) {
    println!(
        "{} {} {}\n",
        "USAGE:".bold().dark_red(),
        "look run".bold().blue(),
        "[OPTIONS] [ARGS]".blue()
    );
    println!(" Arguments:\n");
    for v in cmd_list.iter() {
        println!("\t{}:\t{}", v.0.blue(), v.1 .0)
    }
    println!("\n");
    process::exit(2)
}

fn walk_dir(
    search_substring: &str,
    feild: &mut Vec<NavigationDataFeild>,
    flags: &Flags,
) -> io::Result<()> {
    let folder_read = read_dir(env::current_dir()?.to_path_buf())?;
    for entry in folder_read {
        let elem = entry?;
        if (flags.dir && elem.path().is_dir())
            || (flags.file && elem.path().is_file())
            || (!flags.dir && !flags.file)
        {
            let file_name = elem.file_name().into_string().unwrap_or_default();
            let path_len = elem.path().to_string_lossy().len();
            let name_len = file_name.len();

            if let Some(start) = file_name.find(search_substring) {
                let start_calc = path_len - name_len + start;
                let end = start_calc + search_substring.len();
                feild.push(NavigationDataFeild {
                    path: elem.path(),
                    start: start_calc,
                    end,
                })
            }
        }
    }
    Ok(())
}
