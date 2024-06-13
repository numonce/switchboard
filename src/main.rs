use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::{
    error::Error,
    io::{stdout, Stdout},
};
const LEFTARROW: char = '\u{2b05}';
const RIGHTARROW: char = '\u{2b95}';
const UPARROW: char = '\u{2b06}';
const DOWNARROW: char = '\u{2b07}';

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

fn draw_ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let phases: Vec<Phase> = vec![
        Phase::new(
            "Phase 1",
            "In this phase, shit happens.",
            [
                UPARROW, DOWNARROW, RIGHTARROW, LEFTARROW, RIGHTARROW, UPARROW, DOWNARROW,
            ]
            .iter()
            .collect(),
        ),
        Phase::new(
            "Phase 2",
            "This one, crazy",
            [
                DOWNARROW, DOWNARROW, DOWNARROW, RIGHTARROW, UPARROW, LEFTARROW,
            ]
            .iter()
            .collect(),
        ),
        Phase::new(
            "Phase 3",
            "SOUUUUUPPPPP",
            [
                UPARROW, DOWNARROW, RIGHTARROW, LEFTARROW, RIGHTARROW, UPARROW, DOWNARROW,
            ]
            .iter()
            .collect(),
        ),
    ];
    let mut stateful_phases = Phases::new(phases);
    loop {
        let area = terminal.size()?;
        let hchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);
        let title = Span::styled("Example text", Style::default().fg(Color::Green));
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
    pattern: String,
}
impl Phase<'_> {
    fn new<'a>(name: &'a str, about: &'a str, pattern: String) -> Phase<'a> {
        Phase {
            name,
            about,
            pattern,
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
        let pattern = phase.pattern.clone();
        loop {
            let phase_pattern = Span::styled(pattern.to_owned(), Style::default().fg(Color::Green));
            let area = terminal.size()?;
            let hchunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(area);
            let phase_about = Span::styled(phase.about, Style::default().fg(Color::Green));
            let phase_name = Span::styled(phase.name, Style::default().fg(Color::Green));
            let pattern_box = Paragraph::new(phase_pattern)
                .centered()
                .block(Block::default().borders(Borders::ALL));
            let aboutbox = Paragraph::new(phase_about)
                .centered()
                .block(Block::default().title(phase_name).borders(Borders::ALL));
            terminal.draw(|f| {
                f.render_widget(aboutbox, hchunks[0]);
                f.render_widget(pattern_box, hchunks[1]);
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
