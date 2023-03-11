use std::{io::{self, Write}, thread, time::Duration, vec};
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
// import local modules
use crate::{ARCFM::fuel_rod_table};
use crate::ARCFM::fuel_rod_table_row;

#[allow(non_snake_case)]
mod ARCFM;
#[warn(non_snake_case)]

fn main() -> Result<(), io::Error>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut command_text = String::new();
    // Create a new menu
    
    loop {
        
        let block = Block::default().title("Fuel Rods").borders(Borders::ALL);    
        
        terminal.draw(|frame| {
            let terminal_rect = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(terminal_rect);
            // render a 5x5 grid of fuel rods with a table

            let table = Table::new(
                vec![
                //add blocks around each cell
                fuel_rod_table_row(1, 5),
                fuel_rod_table_row(6, 5),
                fuel_rod_table_row(11,  5),
                fuel_rod_table_row(16, 5),
                fuel_rod_table_row(21,  5)

                ],

                
            ).block(Block::default().title("Fuel Rods").borders(Borders::ALL)).widths(
                &[
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ],
            ).style(
                tui::style::Style::default().fg(tui::style::Color::Yellow),
            );
        
            //fuel_rod_table(5, 5); 
            let tui_command_text = Paragraph::new(command_text.as_str()).block(Block::default().title("Command").borders(Borders::ALL));
            //let block_3 = Block::default().title("Command").borders(Borders::ALL);
            frame.render_widget(tui_command_text, chunks[0]);
            frame.render_widget(block, chunks[1]);

            fuel_rod_table(5, 5, chunks[1], frame);
            let block_2 = Block::default().title("Menu").borders(Borders::ALL);
            frame.render_widget(block_2, chunks[2]);
 
            
        })?;
        // break when ctrl Q is pressed
        match event::read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Char('Q') && event.modifiers == event::KeyModifiers::SHIFT {
                    break;
                } else {
                    command_text = format!("{}{}", command_text, format!("{:?}", event.code).strip_prefix("Char('").unwrap().strip_suffix("')").unwrap());
                    println!("{}", command_text);
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

