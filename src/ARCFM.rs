use tui::buffer::Buffer;
use tui::layout::{Constraint, Layout, Rect, Direction};
use tui::style::Style;
use tui::widgets::{Block, Borders, Cell, Widget, TableState, StatefulWidget, Row, Table};
use tui::text::Text;
use unicode_width::UnicodeWidthStr;

#[derive(Clone,Copy)]
pub struct AbsorberRodControlAndFuelMonitoring {
    reactor_fuel_rod_matrix: [[FuelRod; 5];5],
    pull_rods: bool,
    insert_rods: bool,
    hold_rods: bool,
    // slow, medium, fast
    speed_setpoint: u8,
    reactivity: f32,
    centre_core_only: bool,
}

#[derive(Clone, Copy)]
pub struct FuelRod {
    absorber_rod_position: f32,
    fuel_temperature: f32,
    thermal_power_output: f32,
    pull_rod: bool,
    insert_rod: bool,
}


impl Default for AbsorberRodControlAndFuelMonitoring {
    fn default() -> Self {
        Self {
            reactor_fuel_rod_matrix: [[FuelRod::default(); 5]; 5],
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
            pull_rod: false,
            insert_rod: false,
        }
    }
}

pub fn fuel_rod_table_row(starting_number: i32, width: i32) -> Row<'static> {
    let mut cell_vec = Vec::new();
    //println!("starting number: {}", starting_number+1);
    for i in starting_number+1..starting_number+width+1 {
        //println!("cell number: {}", i);
        let text = Text::from(i.to_string());
        let cell = Cell::from(text).style(tui::style::Style::default().fg(tui::style::Color::Yellow));
        cell_vec.push(cell);
    }
    Row::new(cell_vec)
    
}

pub fn fuel_rod_table(width: i32, height: i32) -> Table<'static>{
    let mut row_vec = Vec::new();
    for i in 1..height {
        //println!("height: {}", i);
        //println!("{} * {}: {}", i, height, i*height);
        row_vec.push(fuel_rod_table_row(i*height, width));
    }
    Table::new(row_vec).block(Block::default().title("Fuel Rods").borders(Borders::ALL)).column_spacing(100/width as u16)
    .style(
        tui::style::Style::default().fg(tui::style::Color::Yellow),
    )
}   
