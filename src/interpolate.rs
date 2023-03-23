use std::fmt::format;

use rayon::prelude::*;

use crate::{structs::MainStruct, main};
use rand::seq::SliceRandom;

pub fn interpolate_position(mainstruct: &mut MainStruct) {
    mainstruct.absorber_rods.par_iter_mut().for_each(|row| {
        row.par_iter_mut().for_each(|rod| match rod.insert_rod {
            true => {
                if rod.absorber_rod_position < rod.set_point {
                    rod.absorber_rod_position += mainstruct.core.speed_setpoint;
                } else if rod.absorber_rod_position > (rod.set_point + 1.0) {
                    rod.absorber_rod_position -= mainstruct.core.speed_setpoint;
                } else {
                    rod.absorber_rod_position = rod.set_point;
                }
            }
            false => {
                if rod.absorber_rod_position < 100.0 {
                    rod.absorber_rod_position += mainstruct.core.speed_setpoint;
                } else {
                    rod.absorber_rod_position = 100.0;
                }
            }
        });
    });
}

pub fn graphline(mainstruct: &mut MainStruct) {
    //println!("In graph line method");
    //mainstruct.data.log.push(format!("Graph newest: {:?}, pos: {}", mainstruct.data.graphs[3][0], mainstruct.absorber_rods[0][0].absorber_rod_position));

    while mainstruct.data.graphs[2].len() > 30 {
        mainstruct.data.graphs[2].pop();
    }
    while mainstruct.data.graphs[3].len() > 30 {
        mainstruct.data.graphs[3].pop();
    }
    while mainstruct.data.graphs[0].len() > 30 {
        mainstruct.data.graphs[0].pop();
    }

    for i in 0..mainstruct.data.graphs[2].len() {
        let j = mainstruct.data.graphs[2][i];

        //graph_data[i] = (j.0, j.1+1.0);
        mainstruct.data.graphs[2][i] = (j.0, j.1 + 1.0);
    }
    for i in 0..mainstruct.data.graphs[3].len() {
        let j = mainstruct.data.graphs[3][i];

        //graph_data[i] = (j.0, j.1+1.0);
        mainstruct.data.graphs[3][i] = (j.0, j.1 + 1.0);
    }
    for i in 0..mainstruct.data.graphs[0].len() {
        let j = mainstruct.data.graphs[0][i];

        //graph_data[i] = (j.0, j.1+1.0);
        mainstruct.data.graphs[0][i] = (j.0, j.1 + 1.0);
    }

    let vs: Vec<f64> = vec![1.0, 2.0, 3.0];
    mainstruct.data.graphs[2].insert(0, (mainstruct.data.neutron_rate as f64, 0.0));

    mainstruct.data.graphs[0].insert(0, (mainstruct.core.thermal_power as f64, 0.0));
    mainstruct.data.graphs[1].insert(0, (*vs.choose(&mut rand::thread_rng()).unwrap(), 0.0));
    mainstruct.data.graphs[3].insert(0, (mainstruct.data.neutron_flux as f64, 0.0));
}
pub fn neutron_rate(mainstruct: &mut MainStruct) {
    mainstruct.absorber_rods.par_iter_mut().for_each(|row| {
        row.par_iter_mut().for_each(|rod| {
            const MAX: f32 = 15.0; // 100% neutron flux rate
            const MIN: f32 = -15.0; // 0% neutron flux rate
            rod.neutron_rate = (MAX - MIN) * (1.0 - rod.absorber_rod_position as f32 / 100.0) + MIN as f32;
            
        });
    });
    //mainstruct.data.log.push(format!("Neutron rate: {}", mainstruct.data.neutron_rate));
    mainstruct.data.neutron_rate = mainstruct
        .absorber_rods
        .par_iter()
        .map(|row| {
            row.par_iter().map(|rod| rod.neutron_rate).sum::<f32>() / (mainstruct.core.width) as f32
        })
        .sum::<f32>()
        / mainstruct.core.height as f32;
    //mainstruct.data.log.push(format!("Reactivity: {}, y value: {}", mainstruct.data.reactivity, mainstruct.data.graphs[2][0].1));
}
pub fn neutron_flux(mainstruct: &mut MainStruct) {
    const MAX: f32 = 110.0; // 100% neutron flux rate
    const MIN: f32 = 0.0; // 0% neutron flux rate

    let average_rod_position = mainstruct
        .absorber_rods
        .par_iter()
        .map(|row| {
            row.par_iter()
                .map(|rod| rod.absorber_rod_position)
                .sum::<f32>() as f32
        })
        .sum::<f32>()
        / (mainstruct.core.width * mainstruct.core.height) as f32;
    //let k = 2.5E14; // proportionality constant
    //mainstruct.data.log.push(format!("average rod position: {}", average_rod_position));

    let normalized_flux = match average_rod_position as i32 {
        0..=100 => (MAX - MIN) * (1.0 - average_rod_position as f32 / 100.0) + MIN,
        _ => 0.0, // handle invalid input
    };

    //mainstruct.data.log.push(format!("normalized flux: {}", normalized_flux));
    // normalized neutron flux should be between 0 and 110
    mainstruct.data.neutron_flux = normalized_flux;
}
pub fn fuel_temperature(mainstruct: &mut MainStruct) {
    const MAX: f32 = 600.0; // 100% neutron flux rate
    const MIN: f32 = 0.0; // 0% neutron flux rate

    mainstruct.absorber_rods.par_iter_mut().for_each(|row| {
        row.par_iter_mut().for_each(|rod| {
            rod.fuel_temperature =
                (MAX - MIN) * (1.0 - rod.absorber_rod_position as f32 / 100.0) + MIN as f32;
        });
    });
    let thermal_power = mainstruct
        .absorber_rods
        .par_iter()
        .map(|row| {
            row.par_iter()
                .map(|rod| rod.fuel_temperature)
                .sum::<f32>() / mainstruct.core.width as f32
        })
        .sum::<f32>()
        / mainstruct.core.height as f32;

    const TH_MAX: f32 = 110.0;
    const TH_MIN: f32 = 0.0;
    // 110% is FP
    let normalized_thermal_power = (TH_MAX - TH_MIN) * (thermal_power as f32 / 600.0) + TH_MIN as f32;
    //mainstruct.data.log.push(format!("Thermal power: {}", normalized_thermal_power));
    mainstruct.core.thermal_power = normalized_thermal_power;

    //mainstruct.data.log.push(format!("Fuel temperature: {}", mainstruct.absorber_rods[0][0].fuel_temperature));
}
pub fn turbine(mainstruct: &mut MainStruct) {
    let speeds = [0.0, 900.0, 1800.0, 2700.0, 3600.0];
    let speed_index = mainstruct.turbine.speed_setpoint_step;
    mainstruct.turbine.setpoint_speed = speeds[speed_index as usize];
    if mainstruct.turbine.turbine_speed < mainstruct.turbine.setpoint_speed {
        mainstruct.turbine.turbine_speed += 1.0 * mainstruct.turbine.steam_flow_rate;
    } else if mainstruct.turbine.turbine_speed > (mainstruct.turbine.setpoint_speed + 1.0) {
        mainstruct.turbine.turbine_speed -= 1.0;
    } else {
        mainstruct.turbine.turbine_speed = mainstruct.turbine.setpoint_speed;
    }
    
    
}