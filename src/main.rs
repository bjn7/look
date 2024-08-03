use std::{env::current_dir, fs::read_dir, io};
mod command;
mod tui;
mod utility;
use tui::NavigationDataFeild;
use utility::Flags;

fn main() -> io::Result<()> {
    let args = utility::get_args();
    let mut navigation_data: Vec<NavigationDataFeild> = Vec::new();
    let mut terminal = tui::init()?;
    walk_dir(&args, &mut navigation_data)?;

    if navigation_data.len() < 1 {
        println!("Couldn't find any!");
        return Ok(());
    }
    let mut app = tui::App::new(&mut navigation_data);
    app.run(&mut terminal)?;

    tui::restore()?;
    match app.input.stdout_result {
        Some(r) => println!("{r}"),
        None => {}
    }

    Ok(())
}

fn walk_dir(args: &Flags, feild: &mut Vec<NavigationDataFeild>) -> io::Result<()> {
    let folder_read = read_dir(current_dir()?.to_path_buf())?;
    for entry in folder_read {
        let elem = entry?;
        if (args.dir && elem.path().is_dir())
            || (args.file && elem.path().is_file())
            || (!args.dir && !args.file)
        {
            let mut file_name = elem.file_name().into_string().unwrap_or_default();
            let mut sub_str = args.sub_str.to_owned();
            if !args.case_sensitive {
                file_name = file_name.to_lowercase();
                sub_str = sub_str.to_lowercase()
            }
            let path_len = elem.path().to_string_lossy().len();
            let name_len = file_name.len();

            if let Some(start) = file_name.find(&sub_str) {
                let start_calc = path_len - name_len + start;
                let end = start_calc + String::len(&sub_str);
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
