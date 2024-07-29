#[allow(unused_imports)]
use std::io::{self, stdout, Result, Stdout};

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

pub enum MODE {
    Selection,
    Command,
}

pub struct App {
    state: ListState,
    exit: bool,
    user_text_input: String,
    mode: MODE,
}

const VALD_CMD: [&str; 2] = ["coded", "cd"];
const PREFIX: char = ':';

impl App {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            exit: false,
            user_text_input: String::new(),
            mode: MODE::Selection,
        }
    }
    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        items: Vec<ListItem>,
    ) -> io::Result<()> {
        self.state.select(Some(0));
        while !self.exit {
            terminal.draw(|frame| self.render(frame, items.clone()))?;
            self.handle_event();
        }
        Ok(())
    }
    fn render(&mut self, frame: &mut Frame, items: Vec<ListItem>) {
        let block = Block::new()
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
            .block(block)
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
        frame.render_stateful_widget(list, frame.size(), &mut self.state);
    }
    fn handle_event(&mut self) -> io::Result<()> {
        let key = event::read()?;
        match key {
            Event::Key(k) if k.kind == KeyEventKind::Press => {
                self.handle_key(k.code);
            }
        }
        Ok(())
    }
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Down => self.state.select_next(),
            KeyCode::Enter => {
                self.user_text_input = String::new();
                self.mode = MODE::Selection
            }
            KeyCode::Char(':') => {
                println!("Entered cmd mode");
                self.mode = MODE::Command;
            }
            other => {
                //put into  user_text_input
            }
        }
    }
    fn exit(&mut self) {
        self.exit = true
    }
}
