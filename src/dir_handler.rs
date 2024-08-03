use crate::{tui::NavigationDataFeild, utility::Flags};
#[allow(unused_imports)]
use std::fs::{read_dir, DirEntry};
use std::io;
use std::path::PathBuf;

// use std::sync::Arc;
pub fn walk_dir(
    args: &Flags,
    nav: &mut Vec<NavigationDataFeild>,
    walk_path: PathBuf,
) -> io::Result<()> {
    let dir_result = read_dir(walk_path)?
        .filter_map(|e| e.ok())
        .collect::<Vec<DirEntry>>();

    dir_result.iter().for_each(|entry| {
        let p = entry.path();
        if args.file && p.is_file() {
            if let Some(nav_field) = get_nav_data_feild(&p, &args.sub_str) {
                // Use thread-safe mechanisms if necessary
                nav.push(nav_field);
            }
        }
        if args.dir {
            //after checking is_file(), the one remamming msut be folder ? nah there are other types too..
            // hence, is_dir() is solution
            if p.is_dir() {
                if let Some(nav_field) = get_nav_data_feild(&p, &args.sub_str) {
                    // Use thread-safe mechanisms if necessary
                    nav.push(nav_field);
                }
            }
            if args.all {
                if let Err(e) = walk_dir(args, nav, p) {
                    eprintln!("Error walking directory: {}", e);
                }
            }
        }
    });
    Ok(())
}

fn get_nav_data_feild(path: &PathBuf, sub_str: &String) -> Option<NavigationDataFeild> {
    let name = &path
        .file_name()
        .expect("Failed to fetch file name")
        .to_string_lossy();
    if let Some(i) = name.find(sub_str) {
        let start = &path.to_string_lossy().len() - &name.len() + i;
        let end = start + sub_str.len();
        return Option::Some(NavigationDataFeild {
            path: path.to_owned(),
            start,
            end,
        });
    } else {
        return Option::None;
    };
}
