use crate::steam::steam;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use interpolate::{fuel_temperature, graphline, neutron_flux, neutron_rate, turbine};
use std::{io, sync::mpsc::channel, thread, time::Duration, vec};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Wrap},
    Terminal,
};

// import local modules
use crate::commands::send_command;
use crate::draw::draw;
use crate::interpolate::interpolate_position;
use crate::structs::MainStruct;

mod arcfm;
mod commands;
mod draw;
mod interpolate;
mod steam;
mod structs;
mod svg;

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let mut mainstruct = MainStruct::default();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut command_text: Vec<String> = Vec::new();
    //let mut log_text: Vec<Spans<>> = Vec::new();

    // Create a new menu

    //let mut fuel_rods = vec![vec![FuelRod::default(); 5]; 5];
    let (tx, rx) = channel();
    thread::spawn(move || loop {
        tx.send(()).unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    let mut previous_commands: (Vec<Vec<String>>, i32) = (Vec::new(), 0);
    let mut labels = Vec::new();
    let mut y_labels = Vec::new();
    // labels is adds 5 until 30 from -30
    for i in -100..=120 {
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
            neutron_flux(&mut mainstruct);
            fuel_temperature(&mut mainstruct);
            steam(&mut mainstruct);
            turbine(&mut mainstruct);
        }
        let graphs = mainstruct.data.graphs.clone();
        let datasets = vec![
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
                .style(Style::default().fg(Color::Red))
                .data(&graphs[0]),
            Dataset::default()
                .name("Reactor Level")
                .marker(symbols::Marker::Lines(symbols::Lines::CROSS))
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Red))
                .data(&graphs[1]),
            Dataset::default()
                .name("Reactivity")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Green))
                .data(&[]), 
        ];

        let graph = Chart::new(datasets)
            .block(Block::default().title("Trend Chart").borders(Borders::ALL))
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([-100.0, 120.0])
                    .labels(labels.iter().cloned().map(Span::from).collect()),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 30.0])
                    .labels(y_labels.iter().cloned().map(Span::from).collect()),
            );

        // log text is a vector of spans, where mainstruct.data.log is a vector of strings
        let log_text: Vec<Spans> = mainstruct
            .data
            .log
            .iter()
            .map(|x| Spans::from(x.clone()))
            .collect();

        //interplate every 100ms without slowing drawing or user input

        let log_paragraph = Paragraph::new(log_text.clone())
            .block(Block::default().title("Log").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        let tui_command_text = Paragraph::new(command_text.concat())
            .block(Block::default().title("Command").borders(Borders::ALL));
        let block_2 = Block::default().title("Menu").borders(Borders::ALL);
        //interplate_position(&mut fuel_rods);

        let height = draw(
            &mut terminal,
            tui_command_text,
            block_2,
            &mut mainstruct,
            log_paragraph.clone(),
            graph.clone(),
        )
        .1;
        while mainstruct.data.log.len() > (height - 2) as usize {
            mainstruct.data.log.remove(0);
        }
        read_input(
            &mut terminal,
            &mut command_text,
            &mut mainstruct,
            &mut previous_commands,
            height,
        )?;
    }
}

pub fn read_input(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    command_text: &mut Vec<String>,
    mainstruct: &mut MainStruct,
    previous_command: &mut (Vec<Vec<String>>, i32),
    height: u16,
) -> Result<(), io::Error> {
    if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Char('Q') && event.modifiers == event::KeyModifiers::SHIFT
                {
                    // clear the screen
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;

                    Err(io::Error::new(io::ErrorKind::Other, "Quit"))                } else if event.code == KeyCode::Char('c')
                    && event.modifiers == event::KeyModifiers::CONTROL
                {
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
                        }
                        KeyCode::Enter => {
                            if mainstruct.data.text_input {
                                send_command(command_text.concat().as_str(), mainstruct, height);
                                //prepend the command to the previous commands
                                previous_command.0.insert(0, command_text.clone());
                                let blacklist = vec!["clear", "cls", "help"];
                                if !blacklist.contains(&command_text.concat().as_str()) {
                                    mainstruct.data.log.push(command_text.concat());
                                }
                                //mainstruct.data.log.push(format!("length: {}", mainstruct.data.log.len()));
    
                                command_text.clear();
                            } else if let Some(item) = mainstruct.data.items.iter_mut().find(|item| item.0 == mainstruct.data.selected_item) {
                                item.2 = !item.2;
                            }

                            return Ok(());
                        }
                        KeyCode::Up => {
                            if mainstruct.data.text_input {
                                previous_command.1 += 1;
                                if previous_command.1 > previous_command.0.len() as i32 {
                                    previous_command.1 = previous_command.0.len() as i32;
                                }
                                if previous_command.1 > 0 {
                                    command_text.clear();
                                    command_text.append(
                                        &mut previous_command.0[(previous_command.1 - 1) as usize]
                                            .clone(),
                                    );
                                }
                            } else if mainstruct.data.checklist_selected > 1 {
                                mainstruct.data.checklist_selected -= 1;

                            } else {
                                mainstruct.data.checklist_selected = 1;
                            }

                            return Ok(());
                        }
                        KeyCode::Down => {
                            if mainstruct.data.text_input {
                                previous_command.1 -= 1;
                                if previous_command.1 < 0 {
                                    previous_command.1 = 0;
                                }
                                if previous_command.1 > 0 {
                                    command_text.clear();
                                    command_text.append(
                                        &mut previous_command.0[(previous_command.1 - 1) as usize]
                                            .clone(),
                                    );
                                }
                            } else if mainstruct.data.checklist_selected < mainstruct.data.checklist_length {
                                mainstruct.data.checklist_selected += 1;
                            } else {
                                mainstruct.data.checklist_selected = mainstruct.data.checklist_length;
                            }

                            return Ok(());
                        }
                        KeyCode::Tab => {
                            if mainstruct.data.left_tab_index + 1 > mainstruct.data.left_tab_length.try_into().unwrap() {
                                mainstruct.data.left_tab_index = 0;
                            } else {
                                mainstruct.data.left_tab_index += 1;
                            }

                            return Ok(());
                        }
                        _ => return Ok(()),
                    }
                }
            }
            _ => Ok(()),
        }
    } else {
        Ok(())
    }
}
