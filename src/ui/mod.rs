use super::app::*;
use crossterm::event::{self, Event, KeyCode};
use std::{
    error::Error,
    time::{Duration, Instant},
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame, Terminal,
};

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut app = App::new(Duration::from_millis(1000));
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = app
            .tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => match app.timer.space_press() {
                        Some(mut t) => {
                            t.gen_stats(&app.times);
                            app.times.push(t);
                            app.tick_rate = Duration::from_millis(1000);
                        }
                        None => app.tick_rate = Duration::from_millis(100),
                    },
                    KeyCode::Esc => app.route.esc(),
                    KeyCode::Enter => app.route.enter(),
                    KeyCode::Char('h') => app.mv(Dir::Left),
                    KeyCode::Char('j') => app.mv(Dir::Down),
                    KeyCode::Char('k') => app.mv(Dir::Up),
                    KeyCode::Char('l') => app.mv(Dir::Right),
                    _ => (),
                }
            }
        }
        if last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // define chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(7),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // render left side
    render_help_and_tools(f, app, left_chunks[0]);
    render_timer(f, app, left_chunks[1]);
    render_times(f, app, left_chunks[2]);

    // render right side
    render_scramble(f, app, right_chunks[0]);
    render_bests(f, app, right_chunks[1]);
    render_main(f, app, right_chunks[2]);
}

fn render_help_and_tools<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layout_chunk);

    let border_style = Style::default().fg(app.get_color_from_id(ActiveBlock::Help));
    let paragraph = Paragraph::new("Press ? for help".to_string())
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let border_style = Style::default().fg(app.get_color_from_id(ActiveBlock::Tools));
    let block = Block::default()
        .title("Tools")
        .borders(Borders::ALL)
        .style(border_style);
    f.render_widget(block, chunks[1]);
}

fn render_timer<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let text = format!("\n\n{}", app.timer.text());
    let style = Style::default().fg(app.get_color_from_id(ActiveBlock::Timer));
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title("Timer")
                .borders(Borders::ALL)
                .border_style(style),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, layout_chunk);
}

pub fn render_times<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let selected_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::LightGreen);
    let normal_style = Style::default().fg(Color::White);
    let header_cells = ["i", "time", "ao5", "ao12"].iter().map(|h| Cell::from(*h));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.times.iter().enumerate().map(|(i, t)| {
        let ao5 = match t.ao5 {
            Some(v) => format!("{:.2}", v),
            None => "-".to_string(),
        };
        let ao12 = match t.ao12 {
            Some(v) => format!("{:.2}", v),
            None => "-".to_string(),
        };
        let cells = vec![
            i.to_string(),
            format!("{:.2}", t.time),
            format!("{}", ao5),
            format!("{}", ao12),
        ];
        Row::new(cells)
    });
    let border_style = Style::default().fg(app.get_color_from_id(ActiveBlock::Times));
    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Table")
                .border_style(border_style),
        )
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Ratio(1, 10),
            Constraint::Ratio(3, 10),
            Constraint::Ratio(3, 10),
            Constraint::Ratio(3, 10),
        ]);
    f.render_stateful_widget(table, layout_chunk, &mut app.times_state);
}

pub fn render_scramble<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let border_style = Style::default().fg(app.get_color_from_id(ActiveBlock::Scramble));
    let block = Block::default()
        .title("Scramble")
        .borders(Borders::ALL)
        .border_style(border_style);
    f.render_widget(block, layout_chunk);
}

pub fn render_bests<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    render_stat(
        f,
        app,
        "PB Single".to_string(),
        "pb single".to_string(),
        chunks[0],
    );
    render_stat(
        f,
        app,
        "PB ao5".to_string(),
        "pb ao5".to_string(),
        chunks[5],
    );
    render_stat(
        f,
        app,
        "PB ao12".to_string(),
        "pb ao12".to_string(),
        chunks[1],
    );
    render_stat(f, app, "ao100".to_string(), "ao100".to_string(), chunks[2]);
    render_stat(f, app, "ao1k".to_string(), "ao1k".to_string(), chunks[3]);
    render_stat(
        f,
        app,
        "rolling avg".to_string(),
        "rolling avg".to_string(),
        chunks[4],
    );
}

fn render_stat<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    title: String,
    text: String,
    layout_chunk: Rect,
) {
    let border_style = Style::default().fg(app.get_color_from_id(ActiveBlock::Stats));
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, layout_chunk);
}

pub fn render_main<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let text = format!("\n\n{:?}\n{:?}", app.route.selected_block, app.pos);
    let border_style = Style::default().fg(app.get_color_from_id(ActiveBlock::Main));
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title("Main")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, layout_chunk);
}
