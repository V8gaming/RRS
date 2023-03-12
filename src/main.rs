use std::{io::{self, Write}, thread, time::Duration, vec};
use ARCFM::FuelRod;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Table, Paragraph},
    layout::{Layout, Constraint, Direction},
    Terminal, buffer::Buffer, symbols::block,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(unix)]
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
// import local modules
use crate::ARCFM::{fuel_rod_table, MainStruct, ARCFMMENU};
use crate::commands::send_command;

#[allow(non_snake_case)]
mod ARCFM;
#[warn(non_snake_case)]
mod commands;
impl MainStruct {
    pub fn new() -> Self {
        Self {
            menu: ARCFMMENU::default(),
            fuel_rods: vec![vec![FuelRod::default(); 5]; 5],
        }
    }
    pub fn draw() {
        
    }
        
}

struct Blocks<'a> {
    command_block: Paragraph<'a>,
    menu_block: Block<'a>,
    fuel_rod_block: Block<'a>,
}
impl Default for Blocks<'_> {
    fn default() -> Self {
        Self {
            command_block: Paragraph::new(""),
            menu_block: Block::default().title("Menu").borders(Borders::ALL),
            fuel_rod_block: Block::default().title("Fuel Rods").borders(Borders::ALL),
        }
    }
}
fn main() -> Result<(), io::Error>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut command_text: Vec<String> = Vec::new();
    // Create a new menu
    let mut fuel_rods = vec![vec![FuelRod::default(); 5]; 5];
    loop {
        let mut blocks = Blocks::default();
        let block = Block::default().title("Fuel Rods").borders(Borders::ALL);    
        let tui_command_text = Paragraph::new(command_text.concat()).block(Block::default().title("Command").borders(Borders::ALL));
        let block_2 = Block::default().title("Menu").borders(Borders::ALL);

        terminal.draw(|frame| {
            let terminal_rect = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(50),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(terminal_rect);   
            //fuel_rod_table(5, 5); 

            //let block_3 = Block::default().title("Command").borders(Borders::ALL);
            frame.render_widget(tui_command_text, chunks[0]);
            frame.render_widget(block, chunks[1]);

            fuel_rod_table(5, 5, chunks[1], frame, &fuel_rods);
            frame.render_widget(block_2, chunks[2]);
 
            
        })?;
        //unix specific
        #[cfg(unix)]
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => {
                    break;
                }
                Key::Char(c) => {
                    command_text.push(c.to_string());
                }
                Key::Backspace => {
                    command_text.pop();
                },
                _ => {}
            }
        }
        // windows specific
        #[cfg(windows)]

        match event::read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Char('Q') && event.modifiers == event::KeyModifiers::SHIFT {
                    // clear the screen
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    break;
                } else {
                    match event.code {
                        KeyCode::Char(c) => {
                            command_text.push(c.to_string());
                        }
                        KeyCode::Backspace => {
                            command_text.pop();
                        },
                        KeyCode::Enter => {
                            send_command(command_text.concat().as_str(), &mut fuel_rods);
                            command_text.clear();
                        },
                        KeyCode::Left => (),
                        KeyCode::Right => (),
                        KeyCode::Up => (),
                        KeyCode::Down => (),
                        KeyCode::Home => (),
                        KeyCode::End => (),
                        KeyCode::PageUp => (),
                        KeyCode::PageDown => (),
                        KeyCode::Tab => (),
                        KeyCode::BackTab => (),
                        KeyCode::Delete => (),
                        KeyCode::Insert => (),
                        KeyCode::F(_) => (),
                        KeyCode::Null => (),
                        KeyCode::Esc => (),
                        KeyCode::CapsLock => (),
                        KeyCode::ScrollLock => (),
                        KeyCode::NumLock => (),
                        KeyCode::PrintScreen => (),
                        KeyCode::Pause => (),
                        KeyCode::Menu => (),
                        KeyCode::KeypadBegin => (),
                        KeyCode::Media(_) => (),
                        KeyCode::Modifier(_) => (),
                    }
                }


            },
            Event::FocusGained => (),
            Event::FocusLost => (),
            Event::Mouse(_) => (),
            Event::Paste(_) => (),
            Event::Resize(_, _) => (),

        }
        
        
    }
    Ok(())
    
    
}

