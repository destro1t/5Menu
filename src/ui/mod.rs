use std::os::unix::fs::PermissionsExt;
use iced::{
    widget::{text_input, container, Column, Text, text_input::TextInput},
    executor, Application, Command, Element, Theme, Length, Subscription,
    theme::{self, Text as TextTheme},
    Background, Color,
    keyboard,
    mouse,
    event::{self, Event},
    subscription,
};
use crate::{config::Config, themes::Theme as AppTheme};

// Custom styles for our UI elements
mod style {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct DarkContainer {
        pub bg_color: Color,
        pub text_color: Color,
        pub border_color: Option<Color>,
        pub border_width: f32,
        pub border_radius: f32,
    }

    impl container::StyleSheet for DarkContainer {
        type Style = Theme;

        fn appearance(&self, _style: &Self::Style) -> container::Appearance {
            container::Appearance {
                text_color: Some(self.text_color),
                background: Some(Background::Color(self.bg_color)),
                border_radius: self.border_radius.into(),
                border_width: self.border_width,
                border_color: self.border_color.unwrap_or(self.bg_color),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct DarkTextInput {
        pub bg_color: Color,
        pub text_color: Color,
        pub border_color: Color,
        pub border_radius: f32,
        pub border_width: f32,
    }

    impl text_input::StyleSheet for DarkTextInput {
        type Style = Theme;

        fn active(&self, _style: &Self::Style) -> text_input::Appearance {
            text_input::Appearance {
                background: Background::Color(self.bg_color),
                border_radius: self.border_radius.into(),
                border_width: self.border_width,
                border_color: self.border_color,
                icon_color: self.text_color,
            }
        }

        fn focused(&self, style: &Self::Style) -> text_input::Appearance {
            let active = self.active(style);
            text_input::Appearance {
                border_color: self.text_color,
                ..active
            }
        }

        fn placeholder_color(&self, _style: &Self::Style) -> Color {
            Color {
                a: 0.5,
                ..self.text_color
            }
        }

        fn value_color(&self, _style: &Self::Style) -> Color {
            self.text_color
        }

        fn selection_color(&self, _style: &Self::Style) -> Color {
            Color { a: 0.2, ..self.text_color }
        }

        fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
            self.active(style)
        }

        fn disabled_color(&self, _style: &Self::Style) -> Color {
            Color { a: 0.5, ..self.text_color }
        }
    }
}
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::process::Command as ProcessCommand;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Execute(String),
    KeyPressed(iced::keyboard::KeyCode),
    Error(String),
    EntrySelected(usize),
    WheelScrolled(mouse::ScrollDelta),
}

pub struct MenuWindow {
    config: Config,
    theme: AppTheme,
    input_value: String,
    entries: Vec<String>,
    filtered_entries: Vec<String>,
    selected_index: usize,
    display_start_index: usize,
}

impl Application for MenuWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = Config::load().unwrap_or_default();
        let theme = AppTheme::load(&config.theme).unwrap_or_default();
        
        let entries = Self::load_applications().unwrap_or_default();
        let filtered_entries = entries.clone().into_iter().take(config.max_entries).collect();
        
        (
            Self {
                config,
                theme,
                input_value: String::new(),
                entries,
                filtered_entries,
                selected_index: 0,
                display_start_index: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("5Menu")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
                self.filter_entries();
                self.selected_index = 0;
                self.display_start_index = 0;
                Command::none()
            }
            Message::Execute(cmd) => {
                // Don't execute math results as commands
                if cmd.starts_with("Answer: ") {
                    Command::none()
                } else {
                    if let Err(e) = self.execute_command(&cmd) {
                        Command::perform(async move { e.to_string() }, Message::Error)
                    } else {
                        std::process::exit(0);
                    }
                }
            }
            Message::KeyPressed(key_code) => {
                match key_code {
                    keyboard::KeyCode::Up => {
                        self.move_selection(-1);
                        Command::none()
                    }
                    keyboard::KeyCode::Down => {
                        self.move_selection(1);
                        Command::none()
                    }
                    keyboard::KeyCode::Enter => {
                        if self.selected_index < self.filtered_entries.len() {
                            if let Some(entry) = self.filtered_entries.get(self.selected_index).cloned() {
                                Command::perform(async { entry }, Message::Execute)
                            } else {
                                Command::none()
                            }
                        } else {
                            Command::none()
                        }
                    }
                    keyboard::KeyCode::Escape => {
                        std::process::exit(0);
                    }
                    _ => Command::none(),
                }
            }
            Message::EntrySelected(index) => {
                if index < self.filtered_entries.len() {
                    self.selected_index = index;
                    if let Some(entry) = self.filtered_entries.get(index).cloned() {
                        Command::perform(async { entry }, Message::Execute)
                    } else {
                        Command::none()
                    }
                } else {
                    Command::none()
                }
            }
            Message::WheelScrolled(delta) => {
                match delta {
                    mouse::ScrollDelta::Lines { y, .. } |
                    mouse::ScrollDelta::Pixels { y, .. } => {
                        if y > 0.0 && self.display_start_index > 0 {
                            self.display_start_index = self.display_start_index.saturating_sub(1);
                        } else if y < 0.0 && self.display_start_index < self.filtered_entries.len().saturating_sub(self.config.max_entries) {
                            self.display_start_index += 1;
                        }
                    }
                }
                Command::none()
            }
            Message::Error(_) => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let input = TextInput::new(
            "Type to search...",
            &self.input_value,
        )
        .on_input(Message::InputChanged)
        .padding(12)
        .style(theme::TextInput::Custom(Box::new(style::DarkTextInput {
            bg_color: self.theme.parse_color(&self.theme.background_color),
            text_color: self.theme.parse_color(&self.theme.text_color),
            border_color: self.theme.parse_color(&self.theme.border_color),
            border_radius: self.theme.border_radius,
            border_width: self.theme.border_width,
        })));

        // Always create a fixed number of entries (max_entries)
        let visible_entries = (0..self.config.max_entries)
            .map(|i| {
                let actual_index = i + self.display_start_index;
                let entry_text = self.filtered_entries
                    .get(actual_index)
                    .map(|s| s.as_str())
                    .unwrap_or("");

                let (bg_color, text_color) = if !entry_text.is_empty() && actual_index == self.selected_index {
                    (
                        self.theme.parse_color(&self.theme.selected_background_color),
                        self.theme.parse_color(&self.theme.selected_text_color),
                    )
                } else {
                    (
                        self.theme.parse_color(&self.theme.background_color),
                        self.theme.parse_color(&self.theme.text_color),
                    )
                };

                let text = Text::new(entry_text)
                    .style(TextTheme::Color(text_color));

                container(text)
                    .width(Length::Fill)
                    .padding(8)
                    .style(theme::Container::Custom(Box::new(style::DarkContainer {
                        bg_color,
                        text_color,
                        border_color: None,
                        border_width: 0.0,
                        border_radius: 0.0,
                    })))
                    .into()
            })
            .collect();

        let entries: Element<_> = Column::with_children(visible_entries)
            .spacing(2)
            .padding(2)
            .into();

        let col = Column::new()
            .push(input)
            .push(entries)
            .max_width(self.config.width as f32)
            .spacing(8);

        container(col)
            .padding(16)
            .style(theme::Container::Custom(Box::new(style::DarkContainer {
                bg_color: self.theme.parse_color(&self.theme.background_color),
                text_color: self.theme.parse_color(&self.theme.text_color),
                border_color: Some(self.theme.parse_color(&self.theme.border_color)),
                border_width: self.theme.border_width,
                border_radius: 0.0,
            })))
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                    Some(Message::KeyPressed(key_code))
                }
                Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                    Some(Message::WheelScrolled(delta))
                }
                _ => None,
            }
        })
    }
}

impl MenuWindow {
    fn load_applications() -> Result<Vec<String>> {
        let mut entries = Vec::new();
        for path in &Config::default().search_paths {
            if let Ok(entries_in_path) = std::fs::read_dir(path) {
                for entry in entries_in_path.filter_map(Result::ok) {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Ok(metadata) = entry.metadata() {
                                if metadata.permissions().mode() & 0o111 != 0 {
                                    if let Some(name) = entry.file_name().to_str() {
                                        entries.push(name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        entries.sort();
        Ok(entries)
    }

    fn filter_entries(&mut self) {
        if self.input_value.trim().is_empty() {
            // Show all applications when input is empty
            self.filtered_entries = self.entries.clone().into_iter().take(self.config.max_entries).collect();
            return;
        }

        // Check if input is a math expression
        if let Some(result) = self.evaluate_math(&self.input_value) {
            self.filtered_entries = vec![format!("Answer: {}", result)];
            return;
        }

        // Regular fuzzy search
        let matcher = SkimMatcherV2::default();
        let mut matches: Vec<_> = self.entries
            .iter()
            .filter_map(|entry| {
                matcher.fuzzy_match(entry, &self.input_value)
                    .map(|score| (score, entry))
            })
            .collect();
        
        matches.sort_by(|a, b| b.0.cmp(&a.0));
        self.filtered_entries = matches.into_iter()
            .map(|(_, entry)| entry.clone())
            .take(self.config.max_entries)
            .collect();
    }

    fn execute_command(&self, cmd: &str) -> Result<()> {
        ProcessCommand::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn()?;
        Ok(())
    }

    fn move_selection(&mut self, delta: isize) {
        if !self.filtered_entries.is_empty() {
            let len = self.filtered_entries.len();
            self.selected_index = if delta < 0 && self.selected_index == 0 {
                len - 1
            } else if delta > 0 && self.selected_index == len - 1 {
                0
            } else {
                let new_index = (self.selected_index as isize + delta) as usize;
                if new_index < len {
                    new_index
                } else if delta > 0 {
                    0
                } else {
                    len - 1
                }
            };

            // Adjust display window to keep selected item visible
            let max_visible = self.config.max_entries;
            if self.selected_index < self.display_start_index {
                self.display_start_index = self.selected_index;
            } else if self.selected_index >= self.display_start_index + max_visible {
                self.display_start_index = self.selected_index.saturating_sub(max_visible - 1);
            }
        }
    }

    fn evaluate_math(&self, input: &str) -> Option<f64> {
        let input = input.trim().replace(" ", "");
        
        // Simple math parser for basic operations
        if let Some(pos) = input.find('+') {
            let (left, right) = input.split_at(pos);
            let right = &right[1..]; // skip the '+'
            if let (Ok(a), Ok(b)) = (left.parse::<f64>(), right.parse::<f64>()) {
                return Some(a + b);
            }
        }
        
        if let Some(pos) = input.find('-') {
            if pos > 0 { // Don't treat negative numbers as subtraction
                let (left, right) = input.split_at(pos);
                let right = &right[1..]; // skip the '-'
                if let (Ok(a), Ok(b)) = (left.parse::<f64>(), right.parse::<f64>()) {
                    return Some(a - b);
                }
            }
        }
        
        if let Some(pos) = input.find('*') {
            let (left, right) = input.split_at(pos);
            let right = &right[1..]; // skip the '*'
            if let (Ok(a), Ok(b)) = (left.parse::<f64>(), right.parse::<f64>()) {
                return Some(a * b);
            }
        }
        
        if let Some(pos) = input.find('/') {
            let (left, right) = input.split_at(pos);
            let right = &right[1..]; // skip the '/'
            if let (Ok(a), Ok(b)) = (left.parse::<f64>(), right.parse::<f64>()) {
                if b != 0.0 {
                    return Some(a / b);
                }
            }
        }
        
        None
    }
}
