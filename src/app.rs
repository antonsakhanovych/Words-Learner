use crate::tui;
use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::prelude::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Stylize},
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph, Row, Table, Widget,
    },
    Frame,
};
use serde::Deserialize;

use std::{collections::VecDeque, fs, io, mem};

// const GRUVBOX_BG: Color = Color::Rgb(0x21, 0x21, 0x22);
const GRUVBOX_FG: Color = Color::Rgb(0xe6, 0xd2, 0xb5);
const GRUVBOX_RED: Color = Color::Rgb(0xff, 0x2d, 0x21);
const GRUVBOX_GREEN: Color = Color::Rgb(0x9e, 0xb1, 0x00);
const GRUVBOX_YELLOW: Color = Color::Rgb(0xec, 0xc1, 0x00);
const GRUVBOX_BLUE: Color = Color::Rgb(0x7a, 0xac, 0xac);
const GRUVBOX_PURPLE: Color = Color::Rgb(0xdd, 0x95, 0xb4);
const GRUVBOX_AQUA: Color = Color::Rgb(0x8f, 0xb6, 0x6d);

#[derive(Debug, Clone, Deserialize)]
struct Word {
    from: String,
    to: String,
}

impl Default for Word {
    fn default() -> Self {
        Self {
            from: String::new(),
            to: String::new(),
        }
    }
}

const VECDEQUE_LIMIT: usize = 30;

#[derive(Debug)]
pub struct App {
    words: Vec<Word>,
    curr_word: Word,
    exit: bool,
    input: String,
    history: VecDeque<(Word, String)>,
    correct: usize,
    wrong: usize,
}

impl App {
    fn append_input(&mut self, ch: char) {
        self.input.push(ch);
    }
    fn pop_input(&mut self) {
        self.input.pop();
    }
    fn evaluate(&mut self) {
        let taken_input = mem::take(&mut self.input);
        let taken_word = mem::take(&mut self.curr_word);
        if taken_input == taken_word.to {
            self.correct += 1;
        } else {
            self.wrong += 1;
        }
        self.history.push_front((taken_word, taken_input));
        self.history.truncate(VECDEQUE_LIMIT);
        self.randomize();
    }

    fn randomize(&mut self) {
        let rand_num = thread_rng().gen_range(0..self.words.len());
        self.curr_word = self.words[rand_num].clone();
    }
}

impl App {
    pub fn new(word_file: &str) -> color_eyre::Result<Self> {
        let file_content = fs::read_to_string(word_file)?;
        let words: Vec<Word> = serde_json::from_str(&file_content)?;
        Ok(Self {
            words,
            curr_word: Word::default(),
            exit: false,
            input: String::default(),
            history: VecDeque::default(),
            correct: usize::default(),
            wrong: usize::default(),
        })
    }
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        self.randomize();
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.modifiers {
                        KeyModifiers::CONTROL => self.handle_ctrl_key(key_event),
                        KeyModifiers::NONE | KeyModifiers::SHIFT => self.handle_key(key_event),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_ctrl_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn handle_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(ch) => self.append_input(ch),
            KeyCode::Backspace => self.pop_input(),
            KeyCode::Enter => self.evaluate(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let default_title = Title::from("".fg(GRUVBOX_FG))
            .alignment(Alignment::Center)
            .position(Position::Top);

        let default_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ])
            .split(area);

        let display_layout = layout[0];
        let display_title = default_title.clone().content(" Words learner ");
        let display_block = default_block
            .clone()
            .fg(GRUVBOX_YELLOW)
            .title(display_title)
            .padding(Padding::top(display_layout.height / 3));

        let display_text = Text::from(Line::from(
            self.curr_word.from.as_str().bold().fg(GRUVBOX_AQUA),
        ));
        Paragraph::new(display_text)
            .centered()
            .block(display_block)
            .render(display_layout, buf);

        let ratio_title = Title::from(Line::from(vec![
            " ".into(),
            self.correct.to_string().fg(GRUVBOX_GREEN),
            " / ".into(),
            self.wrong.to_string().fg(GRUVBOX_RED),
            " ".into(),
        ]))
        .alignment(Alignment::Center)
        .position(Position::Bottom);

        let user_layout = layout[1];
        let user_title = default_title.clone().content(" Translate ");
        let user_block = default_block
            .clone()
            .title(user_title)
            .title(ratio_title)
            .fg(GRUVBOX_BLUE)
            .padding(Padding::top(user_layout.height / 3));

        let user_text = Text::from(Line::from(self.input.as_str().fg(GRUVBOX_YELLOW)));

        Paragraph::new(user_text)
            .centered()
            .block(user_block)
            .render(user_layout, buf);

        let main_title = default_title.clone().content(" History ");

        let main_instructions = Title::from(Line::from(vec![
            " Exit: ".fg(GRUVBOX_PURPLE),
            " Ctrl + <Q> ".fg(GRUVBOX_AQUA).italic(),
        ]))
        .alignment(Alignment::Center)
        .position(Position::Bottom);

        let table_widths = vec![
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ];

        let main_block = default_block
            .clone()
            .title(main_title)
            .title(main_instructions)
            .fg(GRUVBOX_GREEN);

        Table::new(
            self.history.iter().map(|(word, input)| {
                let row = Row::new(
                    [word.from.as_str(), word.to.as_str(), input.as_str()]
                        .iter()
                        .map(|el| Text::from(*el).centered()),
                );
                if word.to == *input {
                    row.fg(GRUVBOX_GREEN)
                } else {
                    row.fg(GRUVBOX_RED)
                }
            }),
            table_widths,
        )
        .column_spacing(1)
        .header(
            Row::new(
                ["From", "To", "Your Answer"]
                    .iter()
                    .map(|el| Text::from(*el).centered()),
            )
            .fg(GRUVBOX_PURPLE)
            .bold(),
        )
        .block(main_block)
        .render(layout[2], buf);
    }
}
