use std::{env::args, fs::read_dir, path::PathBuf};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() <= 1 {
        println!("String not provided");
        return;
    }

    let pattern = &args[1];
    dir_lookup(PathBuf::from("."), pattern);
}

fn dir_lookup(path: PathBuf, pattern: &str) {
    if let Ok(entries) = read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_dir() {
                    dir_lookup(file_path, pattern);
                } else {
                    let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
                    if let Some((start, end)) = kmp_search_first(&file_name, pattern) {
                        let formatted_name = format_with_highlight(&file_name, start, end);
                        let formatted_path = file_path.to_string_lossy().replace("\\", "/");
                        println!("{:?}: match at ({}, {})", formatted_path, start, end);
                        println!("{}", formatted_name);
                    }
                }
            }
        }
    }
}

fn kmp_search_first(text: &str, pattern: &str) -> Option<(usize, usize)> {
    let mut lps = vec![0; pattern.len()];
    compute_lps_array(pattern, &mut lps);

    let mut i = 0; // text index
    let mut j = 0; // pattern index

    while i < text.len() {
        if pattern.as_bytes()[j] == text.as_bytes()[i] {
            i += 1;
            j += 1;
        }

        if j == pattern.len() {
            let start = i - j;
            let end = start + j - 1;
            return Some((start, end));
        } else if i < text.len() && pattern.as_bytes()[j] != text.as_bytes()[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    None
}

fn compute_lps_array(pattern: &str, lps: &mut Vec<usize>) {
    let mut length = 0;
    let mut i = 1;
    while i < pattern.len() {
        if pattern.as_bytes()[i] == pattern.as_bytes()[length] {
            length += 1;
            lps[i] = length;
            i += 1;
        } else {
            if length != 0 {
                length = lps[length - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
}

fn format_with_highlight(text: &str, start: usize, end: usize) -> String {
    let highlight_start = "\x1b[1;30;47m"; // Bold, black text on white background
    let highlight_end = "\x1b[0m"; // Reset all attributes
    let mut result = String::new();
    result.push_str(&text[..start]);
    result.push_str(highlight_start);
    result.push_str(&text[start..=end]);
    result.push_str(highlight_end);
    result.push_str(&text[end + 1..]);
    result
}
