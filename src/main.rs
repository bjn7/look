use std::{env::current_dir, io};
mod command;
mod tui;
mod utility;
use tui::NavigationDataFeild;
mod dir_handler;

fn main() -> io::Result<()> {
    let args = utility::get_args();
    let mut navigation_data: Vec<NavigationDataFeild> = Vec::new();
    let mut terminal = tui::init()?;
    //currentdir ownership changed
    dir_handler::walk_dir(&args, &mut navigation_data, current_dir()?.to_path_buf())?;
    let mut app = tui::App::new(&mut navigation_data);
    app.run(&mut terminal)?;

    tui::restore()?;
    match app.input.stdout_result {
        Some(r) => println!("{r}"),
        None => {}
    }
    Ok(())
}
