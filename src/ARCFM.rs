use std::io::Stdout;

use tui::Frame;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Rect, Direction};
use tui::style::{Style, Color};
use tui::widgets::{Block, Borders, Paragraph};
use tui::text::{Text, Spans};
use rayon::prelude::*;

pub struct FuelRodData {
    FuelPellet: FuelPellet,
    Cladding: Cladding,
}
pub struct FuelPellet {
    U_composition: UComposition,
    OM: f32,
    density: f32,
    Diameter: f32,
    Stack_length: f32,
}

pub struct UComposition {
    U235_composition: f32,
    U238_composition: f32,
}
pub struct Cladding {
    material: String,
    inner_diameter: f32,
    thickness: f32,
}
impl Default for Cladding {
    fn default() -> Self {
        Self {
            material: "Zircaloy".to_string(),
            inner_diameter: 7.62,
            thickness: 0.127,
        }
    }
}

impl Default for UComposition {
    fn default() -> Self {
        Self {
            U235_composition: 0.035,
            U238_composition: 0.965,
        }
    }
}
impl Default for FuelPellet {
    fn default() -> Self {
        Self {
            U_composition: UComposition::default(),
            OM: 3.0,
            density: 10.7,
            Diameter: 7.62,
            Stack_length: 150.0,
        }
    }
}
pub struct PhysicalVariables {
    pub fuel_rod_data: FuelRodData,
    pub distance_between_c_and_f_rods: f32,
}
impl Default for PhysicalVariables {
    fn default() -> Self {
        Self {
            fuel_rod_data: FuelRodData {
                FuelPellet: FuelPellet::default(),
                Cladding: Cladding::default(),
            },
            distance_between_c_and_f_rods: 1.25,
        }
    }
}


#[derive(Clone,Copy, Debug)]
pub struct ARCFMMENU {
    pub width: u16,
    pub height: u16,
    hold_rods: bool,
    // slow, medium, fast
    pub speed_setpoint: f32,
    pub neutron_flux: f32,
}
#[derive(Clone, Debug)]
pub struct MainStruct {
    pub menu: ARCFMMENU,
    pub absorber_rods: Vec<Vec<FuelRod>>,
    pub data: Data
}
#[derive(Clone, Debug)]
pub struct Data {
    pub graphs: Vec<Vec<(f64,f64)>>,
    pub reactivity: f32,
    pub neutron_flux: f32,
    pub neutron_rate: f32,
    pub log: Vec<String>,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            graphs: vec![vec![(0.0,0.0); 2];5],
            reactivity: 0.0,
            neutron_flux: 0.0,
            neutron_rate: 0.0,
            log: Vec::new(),
        }
    }
}

impl Default for MainStruct {
    fn default() -> Self {
        Self {
            menu: ARCFMMENU::default(),
            absorber_rods: vec![vec![FuelRod::default(); 5]; 5],
            data: Data::default(),

        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FuelRod {
    pub absorber_rod_position: f32,
    pub fuel_temperature: f32,
    pub thermal_power_output: f32,
    pub insert_rod: bool,
    pub set_point: f32,
    pub reactivity: f32,
    pub neutron_rate: f32,
}


impl Default for ARCFMMENU {
    fn default() -> Self {
        Self {
            hold_rods: false,
            speed_setpoint: 0.1,
            width: 5,
            height: 5,
            neutron_flux: 0.0,

        }
    }
}

impl Default for FuelRod {
    fn default() -> Self {
        Self {
            absorber_rod_position: 0.0,
            fuel_temperature: 0.0,
            thermal_power_output: 0.0,
            insert_rod: true,
            set_point: 20.0,
            reactivity: 0.0,
            neutron_rate: 0.0,
        }
    }
}


pub fn fuel_rod_table(width: i32, height: i32, layout: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>, fuel_rods: &Vec<Vec<FuelRod>>) {
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

    
    for i in 0..height {
        let column_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(column_constraints.clone())
            .split(row_rects[i as usize]);
        for j in 0..width {
            let text = Text::from(format!("{}:{:.1}% ,{}, {}", i*width+j+1,fuel_rods[i as usize][j as usize].absorber_rod_position, fuel_rods[i as usize][j as usize].fuel_temperature, fuel_rods[i as usize][j as usize].thermal_power_output));
            let cell_text = Paragraph::new(text)
                .block(Block::default().borders(Borders::NONE)
                .style(Style::default().bg(Color::Black)));
            frame.render_widget(cell_text, column_rects[j as usize]);
        }
    }

}   

