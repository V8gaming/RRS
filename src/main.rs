use std::{io::{self, Write}, thread, time::Duration, vec};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Table},
    layout::{Layout, Constraint, Direction},
    Terminal, buffer::Buffer,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// import local modules
use crate::{generic_structs::Menus, ARCFM::fuel_rod_table};
use crate::ARCFM::fuel_rod_table_row;

mod generic_structs;
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

    // Create a new menu
    let mut menu = Menus::default();
    
    loop {
            
        
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(size);
            let block = Block::default().borders(Borders::ALL);
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
            
            block.inner(f.size());

            //fuel_rod_table(5, 5);
            f.render_widget(block, chunks[0]);
            f.render_widget(table, chunks[1]);
 
            
        })?;
        thread::sleep(Duration::from_millis(5000));

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

    }
    
    
}

