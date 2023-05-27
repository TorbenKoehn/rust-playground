use std::{io, path::PathBuf};

use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lslib::{
  dos_ee::{
    item::{DosEeItemResourceReaderExt, DosEeItemResourceWriterExt},
    player::DosEePlayerResourceReaderExt,
  },
  file::File,
  resource::Resource,
};
use tui::{
  backend::{Backend, CrosstermBackend},
  layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  widgets::{Block, Borders, Cell, Row, Table, TableState},
  Frame, Terminal,
};

use crate::error::Error;

use super::util::get_dos_ee_data_path;

#[derive(Debug, Clone, Copy)]
enum Intent {
  SelectPlayer,
  ShowPlayer(usize),
  ShowPlayerInventory(usize),
  AddPlayerInventoryItem(usize),
}

#[derive(Default, Debug, Clone)]
struct CurrentTable {
  state: TableState,
  size: usize,
}

impl CurrentTable {
  pub fn new(table_size: usize) -> Self {
    Self {
      state: TableState::default(),
      size: table_size,
    }
  }

  pub fn table_state(&self) -> &TableState {
    &self.state
  }

  pub fn table_state_mut(&mut self) -> &mut TableState {
    &mut self.state
  }

  pub fn table_size(&self) -> usize {
    self.size
  }

  fn select(&mut self, index: Option<usize>) {
    self.state.select(index);
  }

  fn selected(&self) -> Option<usize> {
    self.state.selected()
  }

  pub fn resize(&mut self, size: usize) {
    self.size = size;
    if self.selected().is_none() && size > 0 {
      self.select(Some(0));
    } else if self.selected().unwrap() >= size {
      self.select(Some(size - 1));
    }
  }
}

struct App {
  globals: Resource,
  intent_stack: Vec<Intent>,
  current_table: CurrentTable,
}

impl App {
  pub fn new(globals: Resource) -> Self {
    Self {
      intent_stack: vec![Intent::SelectPlayer],
      globals,
      current_table: CurrentTable::new(0),
    }
  }

  pub fn current_intent(&self) -> Option<&Intent> {
    self.intent_stack.last()
  }

  pub fn go_to(&mut self, intent: Intent) {
    self.intent_stack.push(intent);
  }

  pub fn go_back(&mut self) {
    self.intent_stack.pop();
  }

  pub fn next_row(&mut self) {
    let i = match self.current_table.selected() {
      Some(i) => {
        if i >= self.current_table.size - 1 {
          0
        } else {
          i + 1
        }
      }
      None => 0,
    };
    self.current_table.select(Some(i));
  }

  pub fn prev_row(&mut self) {
    let i = match self.current_table.selected() {
      Some(i) => {
        if i == 0 {
          self.current_table.size - 1
        } else {
          i - 1
        }
      }
      None => 0,
    };
    self.current_table.select(Some(i));
  }

  pub fn confirm(&mut self) {
    match self.current_intent() {
      Some(Intent::SelectPlayer) => {
        if let Some(index) = self.current_table.selected() {
          self.go_to(Intent::ShowPlayer(index));
        }
      }
      Some(Intent::ShowPlayer(index)) => {
        if let Some(menu_index) = self.current_table.selected() {
          match menu_index {
            0 => self.go_to(Intent::ShowPlayerInventory(*index)),
            _ => {}
          }
        }
      }
      Some(Intent::ShowPlayerInventory(i)) => {}
      Some(Intent::AddPlayerInventoryItem(_)) => {}
      None => {}
    }
  }

  pub fn create(&mut self) {
    match self.current_intent() {
      Some(Intent::SelectPlayer) => {}
      Some(Intent::ShowPlayer(_)) => {}
      Some(Intent::ShowPlayerInventory(player_index)) => {
        let players = self.globals.dos_ee_players();
        let player = players[*player_index];
        let create_item_data = self.globals.dos_ee_player_create_item_data(
          player,
          "CON_Potion_Invisible_C".to_owned(),
          "Common".to_owned(),
          50,
        );
        let item_factory = self.globals.dos_ee_item_factory();
        let new_item = self
          .globals
          .dos_ee_create_item(item_factory, create_item_data);
      }
      Some(Intent::AddPlayerInventoryItem(_)) => {}
      None => {}
    }
  }

  pub fn render_frame<B: Backend>(&mut self, frame: &mut Frame<B>) -> () {
    let frame_size = frame.size();
    let sizes = Layout::default()
      .direction(Direction::Vertical)
      .margin(1)
      .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
      .split(frame_size);

    self.render_main(frame, frame_size);

    match self.current_intent() {
      Some(Intent::SelectPlayer) => {
        self.render_player_selection(frame, sizes[1]);
      }
      Some(Intent::ShowPlayer(index)) => {
        self.render_player_info(*index, frame, sizes[1]);
      }
      Some(Intent::ShowPlayerInventory(player_index)) => {
        self.render_player_inventory(*player_index, frame, sizes[1]);
      }
      Some(Intent::AddPlayerInventoryItem(_)) => {}
      None => {}
    }
  }

  fn render_main<B: Backend>(&self, frame: &mut Frame<B>, size: Rect) -> () {
    let block = Block::default()
      .borders(Borders::ALL)
      .title("D:OS EE Savegame Manager")
      .style(Style::default().fg(Color::Black).bg(Color::White));
    frame.render_widget(block, size);
  }

  fn render_player_selection<B: Backend>(&mut self, frame: &mut Frame<B>, size: Rect) -> () {
    let players = self.globals.dos_ee_players();
    let rows = players
      .iter()
      .map(|player| {
        Row::new(vec![
          Cell::from(self.globals.dos_ee_player_name(*player)),
          Cell::from(format!(
            "{} {}",
            self.globals.dos_ee_player_race_name(*player),
            self.globals.dos_ee_player_class_name(*player)
          )),
        ])
      })
      .collect::<Vec<Row>>();
    self.current_table.resize(rows.len());
    let table = Table::new(rows)
      .block(
        Block::default()
          .borders(Borders::ALL)
          .title("Select Player"),
      )
      .header(
        Row::new(vec!["Name", "Race/Class"])
          .style(Style::default().fg(Color::Yellow))
          .bottom_margin(1),
      )
      .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)])
      .highlight_style(Style::default().add_modifier(Modifier::BOLD))
      .highlight_symbol(">> ");
    frame.render_stateful_widget(table, size, &mut self.current_table.state);
  }

  fn render_player_info<B: Backend>(
    &mut self,
    player_index: usize,
    frame: &mut Frame<B>,
    size: Rect,
  ) -> () {
    let players = self.globals.dos_ee_players();
    let player = players[player_index];
    let player_name = self.globals.dos_ee_player_name(player);

    let sizes = Layout::default()
      .direction(Direction::Horizontal)
      .margin(1)
      .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
      .split(size);

    let rows = vec![
      Row::new(vec![Cell::from("Name"), Cell::from(player_name.to_owned())]),
      Row::new(vec![
        Cell::from("Class"),
        Cell::from(self.globals.dos_ee_player_class_name(player)),
        Cell::from("Race"),
        Cell::from(self.globals.dos_ee_player_race_name(player)),
      ]),
    ];
    let table = Table::new(rows)
      .block(
        Block::default()
          .borders(Borders::ALL)
          .title(format!("Player Info: {}", player_name)),
      )
      .widths(&[
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
      ]);
    frame.render_widget(table, sizes[0]);

    let menu_rows = vec![Row::new(vec![Cell::from("Inventory")])];
    self.current_table.resize(menu_rows.len());
    let menu = Table::new(menu_rows)
      .block(Block::default().borders(Borders::ALL).title("Menu"))
      .widths(&[Constraint::Percentage(100)])
      .highlight_style(Style::default().add_modifier(Modifier::BOLD))
      .highlight_symbol(">> ");
    frame.render_stateful_widget(menu, sizes[1], &mut self.current_table.state);
  }

  fn render_player_inventory<B: Backend>(
    &mut self,
    player_index: usize,
    frame: &mut Frame<B>,
    size: Rect,
  ) -> () {
    let players = self.globals.dos_ee_players();
    let player = players[player_index];
    let inventory = self.globals.dos_ee_player_inventory_items(player);
    let rows = inventory
      .iter()
      .map(|item| {
        Row::new(vec![
          Cell::from(self.globals.dos_ee_item_name(*item)),
          Cell::from(format!("{}", self.globals.dos_ee_item_amount(*item))),
        ])
      })
      .collect::<Vec<Row>>();
    self.current_table.resize(rows.len());
    let table = Table::new(rows)
      .block(Block::default().borders(Borders::ALL).title(format!(
        "Inventory: {}",
        self.globals.dos_ee_player_name(player)
      )))
      .header(
        Row::new(vec!["Name", "Amount"])
          .style(Style::default().fg(Color::Yellow))
          .bottom_margin(1),
      )
      .widths(&[Constraint::Percentage(75), Constraint::Percentage(25)])
      .highlight_style(Style::default().add_modifier(Modifier::BOLD))
      .highlight_symbol(">> ");
    frame.render_stateful_widget(table, size, &mut self.current_table.state);
  }
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Error> {
  loop {
    terminal.draw(|f| app.render_frame(f))?;

    if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Char('q') => return Ok(()),
        KeyCode::Backspace => app.go_back(),
        KeyCode::Up => app.prev_row(),
        KeyCode::Down => app.next_row(),
        KeyCode::Enter => app.confirm(),
        KeyCode::Char('c') => app.create(),
        _ => {}
      }
    }
  }
}

pub async fn cli_dos_ee_manage(
  profile: String,
  save_name: String,
  data_path: Option<PathBuf>,
) -> Result<(), Error> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let data_path = data_path
    .or_else(|| get_dos_ee_data_path())
    .ok_or(Error::NoDataPath)?;

  let globals_resource_path = data_path
    .join("PlayerProfiles")
    .join(profile)
    .join("Savegames_patch")
    .join(&save_name)
    .join(format!("{}.lsv", &save_name))
    .join("globals.lsf");

  terminal.draw(|f| {
    let block = Block::default()
      .borders(Borders::ALL)
      .title("Loading...")
      .style(Style::default().fg(Color::Black).bg(Color::White));
    f.render_widget(block, f.size())
  })?;

  let globals_resource = File::open(&globals_resource_path)?.as_lsf()?;
  let mut app = App::new(globals_resource);

  run(&mut terminal, &mut app)?;

  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  Ok(())
}
