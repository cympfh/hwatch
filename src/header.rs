// Copyright (c) 2021 Blacknon. All rights reserved.
// Use of this source code is governed by an MIT license
// that can be found in the LICENSE file.

// TODO: commandの表示を単色ではなく、Syntax highlightしたカラーリングに書き換える(v0.3.0)
// TODO: input内容の表示

use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
    Frame,
};

// local module
use app::{ActiveArea, DiffMode, InputMode, OutputMode};
use exec::CommandResult;

//const
const POSITION_X_HELP_TEXT: usize = 56;
const WIDTH_TEXT_INTERVAL: usize = 19;

#[derive(Clone)]
pub struct HeaderArea<'a> {
    ///
    pub area: tui::layout::Rect,

    ///
    interval: f64,

    ///
    command: String,

    ///
    timestamp: String,

    ///
    exec_status: bool,

    ///
    data: Vec<Spans<'a>>,

    ///
    line_number: bool,

    ///
    ansi_color: bool,

    ///
    active_area: ActiveArea,

    ///
    diff_mode: DiffMode,

    ///
    output_mode: OutputMode,

    ///
    input_mode: InputMode,

    ///
    input_prompt: String,

    ///
    pub input_text: String,
}

/// Header Area Object Trait
impl<'a> HeaderArea<'a> {
    pub fn new() -> Self {
        Self {
            area: tui::layout::Rect::new(0, 0, 0, 0),

            interval: ::DEFAULT_INTERVAL,

            command: "".to_string(),
            timestamp: "".to_string(),
            exec_status: true,

            data: vec![Spans::from("")],
            ansi_color: false,
            line_number: false,

            active_area: ActiveArea::History,

            diff_mode: DiffMode::Disable,

            output_mode: OutputMode::Output,

            input_mode: InputMode::None,
            input_prompt: "".to_string(),
            input_text: "".to_string(),
        }
    }

    pub fn set_area(&mut self, area: tui::layout::Rect) {
        self.area = area;
    }

    pub fn set_active_area(&mut self, active: ActiveArea) {
        self.active_area = active;
    }

    pub fn set_output_mode(&mut self, mode: OutputMode) {
        self.output_mode = mode;
    }

    pub fn set_line_number(&mut self, line_number: bool) {
        self.line_number = line_number;
    }

    pub fn set_ansi_color(&mut self, ansi_color: bool) {
        self.ansi_color = ansi_color;
    }

    pub fn set_current_result(&mut self, result: CommandResult) {
        self.command = result.command;
        self.timestamp = result.timestamp;
        self.exec_status = result.status;
    }

    pub fn set_interval(&mut self, interval: f64) {
        self.interval = interval;
    }

    pub fn set_diff_mode(&mut self, diff_mode: DiffMode) {
        self.diff_mode = diff_mode;
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.input_mode = input_mode;
    }

    pub fn update(&mut self) {
        // init data
        self.data = vec![];

        // help message
        let help_message = "Display help with h key!";

        // HeaderArea Width
        let width = self.area.width as usize;

        // Value for width calculation.
        let command_width: usize;
        if WIDTH_TEXT_INTERVAL + POSITION_X_HELP_TEXT < width {
            command_width = width - (WIDTH_TEXT_INTERVAL + POSITION_X_HELP_TEXT);
        } else {
            command_width = 0;
        }
        let timestamp_width =
            width - (WIDTH_TEXT_INTERVAL + command_width + help_message.len()) + 1;

        // filter keyword.
        let filter_keyword_width = width - POSITION_X_HELP_TEXT - 2 - 13;
        let filter_keyword = format!("{:wid$}", self.input_text, wid = filter_keyword_width);
        let filter_keyword_style: Style;

        if self.input_text.len() == 0 {
            match self.input_mode {
                InputMode::Filter => self.input_prompt = "/".to_string(),
                InputMode::RegexFilter => self.input_prompt = "*".to_string(),

                _ => self.input_prompt = "".to_string(),
            }

            filter_keyword_style = Style::default().fg(Color::Gray);
        } else {
            filter_keyword_style = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
        }

        // Get the data to display at header.
        let interval = format!("{:.3}", self.interval);

        // Set output type value
        let value_output: String;
        match self.output_mode {
            OutputMode::Output => value_output = "Output".to_string(),
            OutputMode::Stdout => value_output = "Stdout".to_string(),
            OutputMode::Stderr => value_output = "Stderr".to_string(),
        }

        // Set Active Area value
        let value_active: String;
        match self.active_area {
            ActiveArea::History => value_active = "history".to_string(),
            ActiveArea::Watch => value_active = "watch".to_string(),
        }

        // Set Diff mode value
        let value_diff: String;
        match self.diff_mode {
            DiffMode::Disable => value_diff = "None".to_string(),
            DiffMode::Watch => value_diff = "Watch".to_string(),
            DiffMode::Line => value_diff = "Line".to_string(),
            DiffMode::Word => value_diff = "Word".to_string(),
        }

        // Set Color
        let command_color: Color;
        let is_color_enable_color: Color;
        let is_line_number_enable_color: Color;
        match self.exec_status {
            true => command_color = Color::Green,
            false => command_color = Color::Red,
        }
        match self.ansi_color {
            true => is_color_enable_color = Color::Green,
            false => is_color_enable_color = Color::Reset,
        }
        match self.line_number {
            true => is_line_number_enable_color = Color::Green,
            false => is_line_number_enable_color = Color::Reset,
        }

        // Create 1st line.
        self.data.push(Spans::from(vec![
            Span::raw("Every "),
            Span::styled(
                format!("{:>wid$}", interval, wid = 9),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw("s: "),
            Span::styled(
                format!("{:wid$}", self.command, wid = command_width),
                Style::default().fg(command_color),
            ),
            Span::styled(
                format!("Display help with h key!"),
                Style::default().add_modifier(Modifier::REVERSED),
            ),
            Span::styled(
                format!("{:>wid$}", self.timestamp, wid = timestamp_width),
                Style::default().fg(Color::Cyan),
            ),
        ]));

        // Create 2nd line
        self.data.push(Spans::from(vec![
            // filter keyword
            Span::styled(self.input_prompt.clone(), Style::default().fg(Color::Gray)),
            Span::styled(filter_keyword, filter_keyword_style),
            // Line number flag
            Span::styled("Number: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:wid$}", self.line_number, wid = 5),
                Style::default().fg(is_line_number_enable_color),
            ),
            // Color flag
            Span::styled("Color: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:wid$}", self.ansi_color, wid = 5),
                Style::default().fg(is_color_enable_color),
            ),
            Span::raw(" "),
            // Output Type
            Span::styled("Output: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:wid$}", value_output, wid = 6),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(" "),
            // Active Area
            Span::styled("Active: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:wid$}", value_active, wid = 7),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" "),
            // Diff Type
            Span::styled("Diff: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:wid$}", value_diff, wid = 5),
                Style::default().fg(Color::Magenta),
            ),
        ]));
    }

    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let block = Paragraph::new(self.data.clone());
        frame.render_widget(block, self.area);
    }
}
