use rayon::prelude::*;

use crate::{ARCFM::MainStruct, main};
use rand::seq::SliceRandom;

pub fn interpolate_position(mainstruct: &mut MainStruct) {
    mainstruct.absorber_rods.par_iter_mut().for_each(|row| {
        row.par_iter_mut().for_each(|rod| {
            match rod.insert_rod {
                true => {
                    if rod.absorber_rod_position < rod.set_point {
                        rod.absorber_rod_position += mainstruct.menu.speed_setpoint;
                    } else if rod.absorber_rod_position > (rod.set_point + 1.0){
                        rod.absorber_rod_position -= mainstruct.menu.speed_setpoint;
                    } else {
                        rod.absorber_rod_position = rod.set_point;
                    }
                }
                false => {
                    if rod.absorber_rod_position > 0.0 {
                        rod.absorber_rod_position -= mainstruct.menu.speed_setpoint;
                    } else {
                        rod.absorber_rod_position = 0.0;
                    }
                }
            }
        });
    });
}

pub fn graphline(mainstruct: &mut MainStruct) {
    //println!("In graph line method");
    mainstruct.data.log.push(format!("Graph newest: {:?}, pos: {}", mainstruct.data.graphs[3][0], mainstruct.absorber_rods[0][0].absorber_rod_position));

    while mainstruct.data.graphs[2].len() > 30 {
        mainstruct.data.graphs[2].pop();
    }
    while mainstruct.data.graphs[3].len() > 30 {
        mainstruct.data.graphs[3].pop();
    }

    
    for i in 0..mainstruct.data.graphs[2].len() {
        let j = mainstruct.data.graphs[2][i];
        
        //graph_data[i] = (j.0, j.1+1.0);
        mainstruct.data.graphs[2][i] = (j.0, j.1+1.0);
        
    }
    for i in 0..mainstruct.data.graphs[3].len() {
        let j = mainstruct.data.graphs[3][i];
        
        //graph_data[i] = (j.0, j.1+1.0);
        mainstruct.data.graphs[3][i] = (j.0, j.1+1.0);
        
    }


    let vs: Vec<f64> = vec![1.0,2.0,3.0];
    mainstruct.data.graphs[2].insert(0,(mainstruct.data.neutron_rate as f64, 0.0));
    
    mainstruct.data.graphs[0].insert(0,(*vs.choose(&mut rand::thread_rng()).unwrap(), 0.0));
    mainstruct.data.graphs[1].insert(0,(*vs.choose(&mut rand::thread_rng()).unwrap(), 0.0));
    mainstruct.data.graphs[3].insert(0,(mainstruct.data.neutron_flux as f64, 0.0));

}
pub fn neutron_rate(mainstruct: &mut MainStruct) {
    mainstruct.absorber_rods.par_iter_mut().for_each(|row| {
        row.par_iter_mut().for_each(|rod| {
            const FLUX_MAX: f32 = 10.0; // 100% neutron flux rate
            const FLUX_MIN: f32 = 0.0;  // 0% neutron flux rate
            const FLUX_HALF: f32 = 5.0; // 50% neutron flux rate
            rod.neutron_rate = match rod.absorber_rod_position as i32{
                0..=49 => 10.0*((FLUX_HALF - FLUX_MIN) * (1.0 - rod.absorber_rod_position as f32 / 50.0) + FLUX_MIN),
                50 => 0.0,
                51..=100 => 10.0*((FLUX_HALF - FLUX_MIN) * (1.0 - rod.absorber_rod_position as f32 / 50.0) + FLUX_MIN),
                _ => 0.0, // handle invalid input
            };

        });
    });
    mainstruct.data.neutron_rate = mainstruct.absorber_rods.par_iter().map(|row| {
        row.par_iter().map(|rod| {
            rod.neutron_rate
        }).sum::<f32>()/(mainstruct.menu.width) as f32
    }).sum::<f32>() / (mainstruct.menu.width * mainstruct.menu.height) as f32;
    //mainstruct.data.log.push(format!("Reactivity: {}, y value: {}", mainstruct.data.reactivity, mainstruct.data.graphs[2][0].1));
}
pub fn neutron_flux(mainstruct: &mut MainStruct){
    const MAX: f32 = 110.0; // 100% neutron flux rate
    const MIN: f32 = 0.0;  // 0% neutron flux rate

    let average_rod_position = mainstruct.absorber_rods.par_iter().map(|row| {
        row.par_iter().map(|rod| {
            rod.absorber_rod_position
        }).sum::<f32>() as f32
    }).sum::<f32>() / (mainstruct.menu.width * mainstruct.menu.height) as f32;
    let k = 2.5E14; // proportionality constant
    //mainstruct.data.log.push(format!("average rod position: {}", average_rod_position));
    
    let normalized_flux = match average_rod_position as i32{
        0..=49 => (MAX - MIN) * (1.0 - average_rod_position as f32 / 50.0) + MIN,
        50 => 55.0,
        51..=100 => (MAX - MIN) * (1.0 - average_rod_position as f32 / 50.0) + MIN,
        _ => 0.0, // handle invalid input
    };

    mainstruct.data.log.push(format!("normalized flux: {}", normalized_flux));
    // normalized neutron flux should be between 0 and 110
    mainstruct.data.neutron_flux = normalized_flux;
}