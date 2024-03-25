use std::{io, collections::HashMap};

use tui::{
    backend::{Backend, CrosstermBackend}, 
    widgets::{Block, Borders, List, ListItem, Paragraph, BorderType, Tabs, ListState},
    layout::{Layout, Constraint, Direction, Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    Terminal, Frame
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
               LeaveAlternateScreen},
};

mod dice_roller;
use crate::dice_roller::DiceRoller;

pub trait Activity {
    fn render_op_widget<'a, B: Backend>(f: &mut Frame<B>, _app: &mut App, layout: &Vec<Rect>);
    fn render_tab_widget<'a, B: Backend>(f: &mut Frame<B>, _app: &mut App, layout: &Vec<Rect>);
    fn render_active_widget<'a, B: Backend>(f: &mut Frame<B>, _app: &mut App, layout: &Vec<Rect>);
}

enum InputMode {
    Tab,
    Op,
    Active,
}

pub struct App {
    input: String,
    input_mode: InputMode,
    tabs: Vec<String>,
    cur_tab: usize,
    options: HashMap<String, Vec<String>>,
    cur_option: usize,
    dice_roller: DiceRoller,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Tab,
            tabs: vec![
                String::from("General"), 
                String::from("Fantasy General"), 
                String::from("Modern/Future General"), 
                String::from("D&D 5e"), 
                String::from("OSE"),
            ],
            cur_tab: 0,
            options: HashMap::from([
                (String::from("General"), vec![
                    String::from("Dice Roller"),
                    String::from("Markov Name Generator"),
                ]),
                (String::from("Fantasy General"), vec![]),
                (String::from("Modern/Future General"), vec![]),
                (String::from("D&D 5e"), vec![]),
                (String::from("OSE"), vec![]),
            ]),
            cur_option: 0,
            dice_roller: DiceRoller::default(),
        }
    }


}

impl App {
    pub fn next_tab(&mut self) {
        self.cur_tab = (self.cur_tab + 1) % self.tabs.len();
    }

    pub fn prev_tab(&mut self) {
        if self.cur_tab > 0 {
            self.cur_tab -= 1;
        } else {
            self.cur_tab = self.tabs.len() - 1;
        }
    }

    pub fn next_option(&mut self) {
        let op_len = self.options.get(&self.tabs[self.cur_tab]).unwrap().len();
        if op_len < 1 {
            return;
        }
        self.cur_option = (self.cur_option + 1) % op_len;
    }

    pub fn prev_option(&mut self) {
        let op_len = self.options.get(&self.tabs[self.cur_tab]).unwrap().len();
        if op_len < 1 {
            return
        }

        if self.cur_option > 0 {
            self.cur_option -= 1;
        } else {
            self.cur_option = op_len - 1;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run the application
    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, 
                                  app: &mut App) -> io::Result<()> {
    loop {
        let _ = terminal.draw(|f| ui(f, app));   
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Tab => match key.code {
                    KeyCode::Char('E') => {
                        return Ok(());
                    }
                    KeyCode::Right => app.next_tab(),
                    KeyCode::Left  => app.prev_tab(),
                    KeyCode::Down  => app.next_option(),
                    KeyCode::Up    => app.prev_option(),
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Op;
                    }
                    _ => {}
                }
                InputMode::Op => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Tab;
                    }
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Active;
                    }
                    KeyCode::Down => app.next_option(),
                    KeyCode::Up   => app.prev_option(),
                    _ => {}
                }
                InputMode::Active => match app.cur_tab {
                    0 => {
                        match app.cur_option {
                            0 => {
                                match key.code {
                                    KeyCode::Esc => {
                                        app.input_mode = InputMode::Op;
                                        app.dice_roller.clear_results();
                                    }
                                    KeyCode::Enter => {
                                        app.dice_roller.eval_dice_roll(app.input.drain(..).collect());
                                    }
                                    KeyCode::Char(c) => {
                                        app.input.push(c);
                                    }
                                    KeyCode::Backspace => {
                                        app.input.pop();
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        match key.code {
                            KeyCode::Esc => {
                                app.input_mode = InputMode::Op;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let app_heading = "RPG CLI";

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
        [
            Constraint::Length(3),  // Heading
            Constraint::Length(3),  // Menu
            Constraint::Min(2),     // Content
            Constraint::Length(3),  // Footer
        ]
        .as_ref(),
        )
        .split(f.size());
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
        [
            Constraint::Percentage(20),  // Options
            Constraint::Percentage(80),  // Active
        ]
        .as_ref(),
        )
        .split(chunks[2]); // Content
    
    // Heading
    let heading = Paragraph::new(app_heading)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Black))
                .border_type(BorderType::Plain),
        );
    f.render_widget(heading, chunks[0]);

    // Menu
    let menu = app.tabs
        .iter()
        .map(|t| {
            Spans::from(vec![ 
                Span::styled(t, Style::default().fg(Color::Black)),
            ])
        })
        .collect();
    let menu_titles = match app.input_mode {
        InputMode::Tab => {
            Tabs::new(menu).block(Block::default().title("Menu")
                .borders(Borders::ALL))
                .style(Style::default().fg(Color::Black))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw(" | "))
                .select(app.cur_tab)
        }
        InputMode::Op | InputMode::Active => {
            Tabs::new(menu).block(Block::default().title("Menu")
                .borders(Borders::ALL))
                .style(Style::default().fg(Color::Black))
                .highlight_style(Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    )
                .divider(Span::raw(" | "))
                .select(app.cur_tab)
        }
    };
    f.render_widget(menu_titles, chunks[1]);

    // Options
    fn options_list<'a>(app: &'a App) -> List<'a> {
        let options = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Black))
            .title("Options")
            .border_type(BorderType::Plain);

        let items: Vec<_> = app.options.get(&app.tabs[app.cur_tab]).unwrap()
            .into_iter()
            .map(|option| ListItem::new(
                    Spans::from(
                        vec![Span::styled(option, Style::default())]
                    )
            ))
            .collect();

        let option_list = match app.input_mode {
            InputMode::Tab | InputMode::Op=> { 
                List::new(items).block(options)
                    .highlight_style(Style::default().fg(Color::Yellow))
            }
            InputMode::Active => {
                List::new(items).block(options)
                    .highlight_style(Style::default()
                        .fg(Color::White)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                    )
            }
        };

        option_list
    }

    active_frame(f, app, &content_chunks);

    let options = options_list(app);
    let mut state = ListState::default();
    state.select(Some(app.cur_option));
    f.render_stateful_widget(options, content_chunks[0], &mut state);

    // footer
    let footer = Paragraph::new(match app.input_mode {
            InputMode::Tab => "Tab",
            InputMode::Op  => "Option",
            InputMode::Active => "Active",
        })
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Black))
                .border_type(BorderType::Plain),
        );
    f.render_widget(footer, chunks[3]);
}

fn active_frame<'a, B: Backend>(f: &mut Frame<B>, app: &'a mut App, layout: &Vec<Rect>) {
    match app.cur_tab {
        0 => match app.cur_option {
            0 => {
                match app.input_mode {
                    InputMode::Tab | InputMode::Op => {
                        DiceRoller::render_op_widget(f, app, layout);
                    }
                    InputMode::Active => {
                        DiceRoller::render_active_widget(f, app, layout);
                    }
                }
            }
            _ => {
                let active = Paragraph::new("Active Here")
                    .block(
                        Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Black))
                        .border_type(BorderType::Plain),
                    );
                f.render_widget(active, layout[1]);   
            }
        }
        _ => {
            let active = Paragraph::new("Active Here")
                .block(
                    Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Black))
                    .border_type(BorderType::Plain),
                );
            f.render_widget(active, layout[1]);
        }
    }
}


