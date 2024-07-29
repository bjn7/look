#[allow(unused_imports)]
use std::io::{self, stdout, Result, Stdout};

use crate::command::Command;

#[allow(unused_imports)]
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{
            self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, List, ListDirection, ListItem, ListState, Padding,
    },
    Frame, Terminal,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
};

pub fn init() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

struct UserInput {
    cursor: usize,
    content: String,
}
pub struct App {
    state: ListState,
    pub exit: bool,
    input: UserInput,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            exit: false,
            input: UserInput {
                cursor: 0,
                content: String::new(),
            },
        }
    }
    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        items: Vec<ListItem>,
    ) -> io::Result<()> {
        self.state.select(Some(0));
        terminal.show_cursor()?;
        while !self.exit {
            terminal.draw(|frame| self.render(frame, items.clone()))?;
            self.handle_event()?;
        }
        Ok(())
    }
    fn render(&mut self, frame: &mut Frame, items: Vec<ListItem>) {
        let layout_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(80),
                Constraint::Percentage(15),
                Constraint::Percentage(5),
            ])
            .split(frame.size());

        let result_block = Block::new()
            .title(
                Title::from("Result")
                    .position(Position::Top)
                    .alignment(ratatui::layout::Alignment::Center),
            )
            .style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::LightBlue));

        let list = List::new(items)
            .block(result_block)
            .style(Style::default().fg(Color::Red))
            .highlight_style(
                Style::default()
                    .bg(Color::White)
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(" > ")
            .repeat_highlight_symbol(false)
            .direction(ListDirection::TopToBottom);
        let input_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::LightBlue));

        let input_area = Paragraph::new(self.input.content.as_str())
            .style(Style::default().fg(Color::Red))
            .block(input_block);
        frame.render_widget(input_area, layout_area[1]);
        frame.set_cursor(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            layout_area[1].x + self.input.cursor as u16 + 1,
            // Move one line down, from the border to the input line
            layout_area[1].y + 1,
        );

        frame.render_stateful_widget(list, layout_area[0], &mut self.state);
    }
    fn handle_event(&mut self) -> io::Result<()> {
        let key = event::read()?;
        match key {
            Event::Key(k) if k.kind == KeyEventKind::Press => {
                self.handle_key(k.code);
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Down => self.state.select_next(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Backspace => self.backspace_ch(),
            KeyCode::Enter => {
                self.handle_cmd();
                self.input.content = String::new();
                self.reset_cursor();
            }
            KeyCode::Delete => self.delete_ch(),
            KeyCode::Char(ch) => self.insert_ch(ch),
            _ => {}
        }
    }
    fn handle_cmd(&mut self) {
        self.input.content = String::from(&self.input.content);
        match self.input.content.as_str() {
            "quit" => self.quit(),
            _ => {}
        }
    }
}

trait Input {
    fn move_cursor_left(&mut self);
    fn move_cursor_right(&mut self);
    fn clamp_cursor(&mut self, pos: usize) -> usize;
    fn reset_cursor(&mut self);
    fn insert_ch(&mut self, ch: char);
    fn delete_ch(&mut self);
    fn backspace_ch(&mut self);
}
impl Input for App {
    fn move_cursor_left(&mut self) {
        self.input.cursor = self.clamp_cursor(self.input.cursor.saturating_sub(1));
    }

    fn move_cursor_right(&mut self) {
        self.input.cursor = self.clamp_cursor(self.input.cursor.saturating_add(1));
    }
    fn clamp_cursor(&mut self, new_pos: usize) -> usize {
        new_pos.clamp(0, self.input.content.len())
    }

    fn reset_cursor(&mut self) {
        self.input.cursor = 0;
    }
    fn insert_ch(&mut self, ch: char) {
        self.input.content.insert(self.input.cursor, ch);
        self.move_cursor_right()
    }
    fn delete_ch(&mut self) {
        if self.input.cursor < self.input.content.len() {
            self.input.content.remove(self.input.cursor);
        }
    }
    fn backspace_ch(&mut self) {
        if self.input.cursor > 0 {
            self.move_cursor_left();
            self.input.content.remove(self.input.cursor);
        }
    }
}
