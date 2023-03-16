use std::{io::{self, Write}, thread, time::Duration, vec, sync::mpsc::channel};
use ARCFM::{FuelRod, ARCFMMENU, MainStruct};
use interpolate::{graphline, neutron_flux, neutron_rate};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, Wrap, Sparkline, Chart, Dataset, GraphType, Axis},
    layout::{Layout, Constraint, Direction},
    Terminal, text::{Spans, Span}, style::{Color, Style}, symbols
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// import local modules
use crate::ARCFM::{fuel_rod_table};
use crate::commands::send_command;
use crate::interpolate::interpolate_position;

#[allow(non_snake_case)]
mod ARCFM;
#[warn(non_snake_case)]
mod commands;
mod interpolate;
fn main() -> Result<(), io::Error>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let mut mainstruct = MainStruct::default();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut command_text: Vec<String> = Vec::new();
    let mut log_text: Vec<Spans<>> = Vec::new();

    // Create a new menu
    
    //let mut fuel_rods = vec![vec![FuelRod::default(); 5]; 5];
    let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            tx.send(()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mut previous_commands: (Vec<Vec<String>>, i32) = (Vec::new(), 0);
    let mut labels = Vec::new();
    let mut y_labels = Vec::new();
    // labels is adds 5 until 30 from -30
    for i in -100..=110 {
        if i % 20 == 0 {
            labels.push(i.to_string());
            //mainstruct.data.log.push(format!("{}: ", i));
        }
    }
    // y_labels is adds 5 until 50 from 0
    for i in 0..=30 {
        if i % 5 == 0 {
            y_labels.push(i.to_string());
            //mainstruct.data.log.push(format!("{}: ", i));
        }
    }
    loop {
        while rx.try_recv().is_ok() {
            interpolate_position(&mut mainstruct);
            graphline(&mut mainstruct);
            neutron_rate(&mut mainstruct);
            neutron_flux(&mut mainstruct)

        }
        let graphs = mainstruct.data.graphs.clone();
        let mut datasets = vec![
            Dataset::default()
                .name("Neutron Flux")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Cyan))
                .data(&graphs[3]),
            Dataset::default()
                .name("Neutron Rate")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::White))
                .data(&graphs[2]),
            Dataset::default()
                .name("Thermal Power")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Gray))
                .data(&graphs[0]),
            Dataset::default()
                .name("Reactor Level")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Red))
                .data(&graphs[1]),
            Dataset::default()
                .name("Reactivity")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Green))
                .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
        ];

        let graph = Chart::new(datasets).block(Block::default().title("Trend Chart").borders(Borders::ALL))
            .x_axis(Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([-100.0, 110.0])
                .labels(labels.iter().cloned().map(Span::from).collect()))
            .y_axis(Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 30.0])
                .labels(y_labels.iter().cloned().map(Span::from).collect()));
        
        // log text is a vector of spans, where mainstruct.data.log is a vector of strings
        log_text = mainstruct.data.log.iter().map(|x| Spans::from(x.clone())).collect();

        //interplate every 100ms without slowing drawing or user input

        let log_paragraph = Paragraph::new(log_text.clone()).block(Block::default().title("Log").borders(Borders::ALL)).wrap(Wrap { trim: true });
        let block = Block::default().title("Reactor core").borders(Borders::ALL);    
        let tui_command_text = Paragraph::new(command_text.concat()).block(Block::default().title("Command").borders(Borders::ALL));
        let block_2 = Block::default().title("Menu").borders(Borders::ALL);
        //interplate_position(&mut fuel_rods);
        
        let height = draw(&mut terminal, tui_command_text, block, block_2, &mut mainstruct.absorber_rods, log_paragraph.clone(), graph.clone()).1;
        while mainstruct.data.log.len() > (height-2) as usize {
            mainstruct.data.log.remove(0);
        }
        read_input(&mut terminal, &mut command_text, &mut mainstruct, &mut previous_commands, &mut log_text, height)?;

    }
}

pub fn read_input(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, command_text: &mut Vec<String>, mainstruct: &mut MainStruct, previous_command: &mut (Vec<Vec<String>>, i32), log_text: &mut Vec<Spans<>>, height: u16) -> Result<(), io::Error> {
    
    if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Char('Q') && event.modifiers == event::KeyModifiers::SHIFT {
                    // clear the screen
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    return Err(io::Error::new(io::ErrorKind::Other, "Quit"));
                } else {
                    match event.code {
                        KeyCode::Char(c) => {
                            command_text.push(c.to_string());
                            return Ok(());
                        }
                        KeyCode::Backspace => {
                            command_text.pop();
                            return Ok(());
                        },
                        KeyCode::Enter => {
                            send_command(command_text.concat().as_str(), mainstruct, height);
                            //prepend the command to the previous commands
                            previous_command.0.insert(0, command_text.clone());
                            if command_text.concat() != "clear" {
                                if command_text.concat().starts_with("help") {
                                    
                                } else {
                                    mainstruct.data.log.push(command_text.concat());
                                }
                            }
                            //mainstruct.data.log.push(format!("length: {}", mainstruct.data.log.len()));


                            command_text.clear();
                            return Ok(());
                        },
                        KeyCode::Up => {
                            previous_command.1 += 1;
                            if previous_command.1 > previous_command.0.len() as i32 {
                                previous_command.1 = previous_command.0.len() as i32;
                            }
                            if previous_command.1 > 0 {
                                command_text.clear();
                                command_text.append(&mut previous_command.0[(previous_command.1 - 1) as usize].clone());
                            }
                            return Ok(());

                        },
                        KeyCode::Down => {
                            previous_command.1 -= 1;
                            if previous_command.1 < 0 {
                                previous_command.1 = 0;
                            }
                            if previous_command.1 > 0 {
                                command_text.clear();
                                command_text.append(&mut previous_command.0[(previous_command.1 - 1) as usize].clone());
                            }
                            return Ok(());

                        },
                        _ => return Ok(()),
                }
            }
        
        }
        _ => return Ok(()),
        }
    } else {
        return Ok(());
    }
    
}
fn draw(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, tui_command_text: Paragraph<'_>, reactor_core: Block<'_>, block_2: Block<'_>, fuel_rods: &mut Vec<Vec<FuelRod>>, log_text: Paragraph, graph: Chart) -> (Result<(), io::Error>, u16) {
    let mut chunks_2 = Vec::new();
    let mut chunks_3 = Vec::new();
    let draw = terminal.draw(|frame| {
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
        // split block 2 into 2 columns
        chunks_2 = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(chunks[2]);
        chunks_3 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
        
        frame.render_widget(tui_command_text, chunks[0]);
        frame.render_widget(reactor_core, chunks_3[0]);
        //println!("{}", chunks_2[0].height);
        fuel_rod_table(5, 5, chunks_3[0], frame, &fuel_rods);
        frame.render_widget(graph, chunks_3[1]);
        frame.render_widget(block_2, chunks_2[0]);
        frame.render_widget(log_text, chunks_2[1]);
    });
    drop(draw);
    return (Ok(()), chunks_2[0].height);
}