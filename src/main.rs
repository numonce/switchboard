mod arrows;
use crate::arrows::{draw_arrow, Arrow, DOWNARROW, LEFTARROW, RIGHTARROW, UPARROW};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use derive_setters::Setters;
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Widget, Wrap},
    Terminal,
};
use std::{
    error::Error,
    io::{stdout, Stdout},
};

fn main() -> Result<(), Box<dyn Error>> {
    //Setup with alternate screen.
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    draw_ui(&mut terminal)?;
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
fn activate_phase(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    loop {
        let popup = Popup::default()
            .content("Phase activated!!! Press 'q' to quit.")
            .style(Style::new().yellow())
            .title("Sequence accepted.")
            .title_style(Style::new().white().bold())
            .border_style(Style::new().red());
        terminal.draw(|f| {
            f.render_widget(popup, centered_rect(f.size(), 35, 35));
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn draw_ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let phases: Vec<Phase> = vec![
        Phase::new(
            "Phase 1",
            "In this phase, stuff happens.",
            [
                UPARROW, DOWNARROW, RIGHTARROW, LEFTARROW, RIGHTARROW, UPARROW, DOWNARROW,
            ]
            .to_vec(),
            [
                KeyCode::Up,
                KeyCode::Down,
                KeyCode::Right,
                KeyCode::Left,
                KeyCode::Right,
                KeyCode::Up,
                KeyCode::Down,
            ]
            .to_vec(),
        ),
        Phase::new(
            "Phase 2",
            "This one, crazy",
            [
                DOWNARROW, DOWNARROW, DOWNARROW, RIGHTARROW, UPARROW, LEFTARROW,
            ]
            .to_vec(),
            [
                KeyCode::Down,
                KeyCode::Down,
                KeyCode::Down,
                KeyCode::Right,
                KeyCode::Up,
                KeyCode::Left,
            ]
            .to_vec(),
        ),
        Phase::new(
            "Phase 3",
            "SOUUUUUPPPPP",
            [
                UPARROW, DOWNARROW, RIGHTARROW, LEFTARROW, RIGHTARROW, UPARROW, DOWNARROW,
            ]
            .to_vec(),
            [
                KeyCode::Up,
                KeyCode::Down,
                KeyCode::Right,
                KeyCode::Left,
                KeyCode::Right,
                KeyCode::Up,
                KeyCode::Down,
            ]
            .to_vec(),
        ),
    ];
    let mut stateful_phases = Phases::new(phases);
    loop {
        let area = terminal.size()?;
        let hchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);
        let title = Span::styled(
            "Example text. Use J/K for movement. Enter to select phase.\n press 'q' to quit",
            Style::default().fg(Color::Green),
        );
        let titlebox = Paragraph::new(title)
            .centered()
            .block(Block::default().borders(Borders::ALL));
        let items: Vec<ListItem> = stateful_phases
            .items
            .iter()
            .map(|i| ListItem::new(i.name))
            .collect();
        let item_list = List::new(items)
            .block(
                Block::default()
                    .title("Phases")
                    .borders(Borders::ALL)
                    .title_alignment(ratatui::layout::Alignment::Center),
            )
            .style(Style::default().fg(Color::Green))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol(">>");
        terminal.draw(|f| {
            f.render_widget(titlebox, hchunks[0]);
            f.render_stateful_widget(item_list, hchunks[1], &mut stateful_phases.state);
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => stateful_phases.next(),
                    KeyCode::Char('k') => stateful_phases.previous(),
                    KeyCode::Enter => {
                        stateful_phases.start(terminal)?;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

struct Phases<'a> {
    items: Vec<Phase<'a>>,
    state: ListState,
}
#[derive(Clone)]
struct Phase<'a> {
    name: &'a str,
    about: &'a str,
    pattern: Vec<Arrow>,
    game: Vec<KeyCode>,
}
impl Phase<'_> {
    fn new<'a>(
        name: &'a str,
        about: &'a str,
        pattern: Vec<Arrow>,
        game: Vec<KeyCode>,
    ) -> Phase<'a> {
        Phase {
            name,
            about,
            pattern,
            game,
        }
    }
}

impl Phases<'_> {
    fn new(items: Vec<Phase>) -> Phases {
        Phases {
            items,
            state: ListState::default(),
        }
    }

    fn start(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        terminal.clear()?;
        let index = self.state.selected().unwrap();
        let phase = self.items.get(index).unwrap();
        let game = &phase.game;
        let mut game_index = 0;
        loop {
            let arrows = phase.pattern.clone();
            let mut constraints = Vec::new();
            let area = terminal.size()?;
            let hchunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(area);
            for _ in arrows.iter() {
                constraints.push(Constraint::Percentage(100 / arrows.len() as u16));
            }
            let wchunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(hchunks[1]);
            let mut renderable_arrows = Vec::new();
            for (i, arrow) in arrows.iter().enumerate() {
                let tmp = draw_arrow(arrow.to_owned())?;
                if i < game_index {
                    renderable_arrows.push(tmp.background_color(Color::Black))
                } else {
                    renderable_arrows.push(tmp);
                }
            }
            let asdf = wchunks.iter().zip(renderable_arrows.iter());
            let phase_about = Span::styled(phase.about, Style::default().fg(Color::Green));
            let phase_name = Span::styled(phase.name, Style::default().fg(Color::Green));
            let aboutbox = Paragraph::new(phase_about)
                .centered()
                .block(Block::default().title(phase_name).borders(Borders::ALL));
            terminal.draw(|f| {
                f.render_widget(aboutbox, hchunks[0]);
                for (area, widget) in asdf {
                    f.render_widget(widget, area.to_owned());
                }
            })?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up if game.get(game_index) == Some(&KeyCode::Up) => {
                            game_index += 1;
                        }
                        KeyCode::Down if game.get(game_index) == Some(&KeyCode::Down) => {
                            game_index += 1;
                        }
                        KeyCode::Left if game.get(game_index) == Some(&KeyCode::Left) => {
                            game_index += 1;
                        }
                        KeyCode::Right if game.get(game_index) == Some(&KeyCode::Right) => {
                            game_index += 1;
                        }
                        _ => {
                            game_index = 0;
                        }
                    }
                }
            }
            if game_index == game.len() {
                activate_phase(terminal)?;
                break;
            }
        }
        Ok(())
    }
    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
#[derive(Debug, Default, Setters)]
struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}
impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}
