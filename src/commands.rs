use crate::ARCFM::FuelRod;

pub fn send_command(command: &str, fuel_rods: &mut Vec<Vec<FuelRod>>) {
    match command {
        "scram" => {
            // set all fuel rods to insert = false
            for i in 0..fuel_rods.len() {
                for j in 0..fuel_rods[i].len() {
                    fuel_rods[i][j].insert_rod = false;
                }
            }
        }
        _ => {

        }
    } 
}