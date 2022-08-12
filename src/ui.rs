use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
  backend::{Backend, CrosstermBackend},
  layout::{Constraint, Layout},
  style::{Color, Modifier, Style},
  widgets::{Block, Borders, Cell, Row, Table, TableState},
  Frame, Terminal,
};
use crate::api::DataProvider;

struct App<'a> {
  state: TableState,
  items: Vec<Vec<&'a str>>,
}

impl<'a> App<'a> {
  fn new() -> App<'a> {
    App {
      state: TableState::default(),
      items: vec![],
    }
  }

  pub fn next(&mut self) {
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

  pub fn previous(&mut self) {
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

pub fn run() -> Result<(), Box<dyn Error>> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // create app and run it
  let mut app = App::new();
  let mut provider = DataProvider::new();
  app.items = provider.update_items();

  let res = run_app(&mut terminal, app);

  // restore terminal
  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  if let Err(err) = res {
    println!("{:?}", err)
  }

  Ok(())
}

fn run_app<'a, B: Backend>(terminal: &mut Terminal<B>, mut app: App<'a>) -> io::Result<()> {
  loop {
    terminal.draw(|f| ui(f, &mut app))?;

    if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Char('q') => return Ok(()),
        KeyCode::Down => app.next(),
        KeyCode::Up => app.previous(),
        _ => {}
      }
    }
  }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  let rects = Layout::default()
      .constraints([Constraint::Percentage(100)].as_ref())
      .split(f.size());

  let selected_style = Style::default().add_modifier(Modifier::REVERSED);
  let header_cells = ["Symbol", "Name", "Price(USD)", "Price Change 24h"]
      .iter()
      .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
  let header = Row::new(header_cells)
      .style(Style::default())
      .height(1)
      .bottom_margin(1);
  let rows = app.items.iter().map(|item| {
      let height = item
          .iter()
          .map(|content| content.chars().filter(|c| *c == '\n').count())
          .max()
          .unwrap_or(0)
          + 1;

      let is_down = item[3].starts_with("-");
      let cells = item.iter().map(|c| {
        Cell::from(*c)
          .style(Style::default()
          .fg(if is_down { Color::Red } else { Color::Green }))
      });
      Row::new(cells).height(height as u16).bottom_margin(1)
  });
  let t = Table::new(rows)
      .header(header)
      .block(Block::default().borders(Borders::ALL))
      .highlight_style(selected_style)
      .highlight_symbol(">> ")
      .widths(&[
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(20),
      ])
      .column_spacing(5);
  f.render_stateful_widget(t, rects[0], &mut app.state);
}