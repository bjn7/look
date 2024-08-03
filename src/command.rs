use crate::{tui::App, utility::Utility};
use std::{
    env,
    fs::write,
    io::Error,
    path::PathBuf,
    process::{self, Output},
};

pub trait Command {
    fn quit(&mut self);
    fn code(&mut self);
    fn cd(&mut self);
    fn display_stdout(&mut self, cmd_result: Result<Output, Error>);
}

impl<'a> Command for App<'a> {
    fn quit(&mut self) {
        self.exit = true
    }
    fn code(&mut self) {
        let result = process::Command::new("cmd.exe")
            .args(["/C", "code", &self.get_path().to_string_lossy().to_string()])
            .output();
        self.display_stdout(result);
    }
    fn cd(&mut self) {
        let path = self.get_path();
        let path_buf: PathBuf = if path.is_dir() {
            path.to_path_buf()
        } else {
            match path.parent() {
                Some(p) => p.to_path_buf(),
                None => env::current_dir().unwrap(),
            }
        };
        println!("Working Dir: {:?}", path_buf.to_str());
        self.quit()
        // self.quit();
    }

    fn display_stdout(&mut self, cmd_result: Result<Output, Error>) {
        match cmd_result {
            Ok(r) => {
                let out = String::from_utf8(r.stderr).unwrap();
                write("log_display_stdout.txt", out.clone()).unwrap();
                if out.is_empty() {
                    self.quit();
                } else {
                    self.input.stdout_result = Option::from(out);
                    self.quit();
                }
            }
            Err(_) => {
                self.input.stdout_result = Option::from(String::from("Err! Failed to execute"));
            }
        };
    }
}
