use std::io::{self, Stdout};

use tui::Frame;
use tui::backend::CrosstermBackend;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Layout, Rect, Direction};
use tui::style::{Style, Color};
use tui::symbols::block;
use tui::widgets::{Block, Borders, Cell, Widget, TableState, StatefulWidget, Row, Table, Paragraph};
use tui::text::{Text, Span, Spans};
use unicode_width::UnicodeWidthStr;

#[derive(Clone,Copy)]
pub struct ARCFMMENU {
    pull_rods: bool,
    insert_rods: bool,
    hold_rods: bool,
    // slow, medium, fast
    speed_setpoint: u8,
    reactivity: f32,
    centre_core_only: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct FuelRod {
    absorber_rod_position: f32,
    fuel_temperature: f32,
    thermal_power_output: f32,
    insert_rod: bool,
}


impl Default for ARCFMMENU {
    fn default() -> Self {
        Self {
            pull_rods: false,
            insert_rods: false,
            hold_rods: false,
            speed_setpoint: 0,
            reactivity: 0.0,
            centre_core_only: false,
        }
    }
}

impl Default for FuelRod {
    fn default() -> Self {
        Self {
            absorber_rod_position: 0.0,
            fuel_temperature: 0.0,
            thermal_power_output: 0.0,
            insert_rod: false,
        }
    }
}

pub fn fuel_rod_table_row(starting_number: i32, width: i32) -> Row<'static> {
    let mut cell_vec = Vec::new();
    //println!("starting number: {}", starting_number+1);
    for i in starting_number..starting_number+width+1 {
        //println!("cell number: {}", i);
        let text = Text::from(i.to_string());
        let cell = Cell::from(text).style(tui::style::Style::default().fg(tui::style::Color::Yellow));
        cell_vec.push(cell);
    }
    Row::new(cell_vec)
    
}

pub fn fuel_rod_table(width: i32, height: i32, layout: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let column_constraints = std::iter::repeat(Constraint::Percentage((100 / width) as u16))
        .take((width) as usize)
        .collect::<Vec<_>>();
    let row_constraints = std::iter::repeat(Constraint::Percentage((100 / height) as u16))
        .take((height) as usize)
        .collect::<Vec<_>>();

    let row_rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .margin(1)
        .split(layout);

    let fuel_rods = vec![vec![FuelRod::default(); 5]; 5];
    for i in 0..height {
        let column_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(column_constraints.clone())
            .split(row_rects[i as usize]);
        for j in 0..width {
/*             let text = vec![
                //Spans::from(Span::from(format!("{}", (i)*width+j+1)))
                // every struct value from the fuel rod
                Spans::from(Span::from(format!("{:?}", fuel_rods[i as usize][j as usize].absorber_rod_position))),
                Spans::from(Span::from(format!("{:?}", fuel_rods[i as usize][j as usize].fuel_temperature))),
                Spans::from(Span::from(format!("{:?}", fuel_rods[i as usize][j as usize].thermal_power_output))),
                Spans::from(Span::from(format!("{:?}", fuel_rods[i as usize][j as usize].insert_rod))),
            ]; */
            let text = Text::from(format!("FR: {}, POS: {}, °C: {}. TMO: {}, IN: {}", i*width+j+1,fuel_rods[i as usize][j as usize].absorber_rod_position, fuel_rods[i as usize][j as usize].fuel_temperature, fuel_rods[i as usize][j as usize].thermal_power_output, fuel_rods[i as usize][j as usize].insert_rod));
            let cell_text = Paragraph::new(text)
                .block(Block::default().borders(Borders::NONE)
                .style(Style::default().bg(Color::Black)));
            frame.render_widget(cell_text, column_rects[j as usize]);
        }
    }

}   