use crate::structs::MainStruct;
use rayon::prelude::*;
use regex::{Regex, RegexSet};

pub fn send_command(command: &str, mainstruct: &mut MainStruct, height: u16) {
    let set = RegexSet::new([
        r"insert rod (\d+)",
        r"scram",
        r"insert rods",
        r"pull|remove rods",
        r"remove rod (\d+)",
        r"setpoint (\d+)",
        r"set rod (\d+) to (\d+)",
        r"help (\d+)",
        r"cls|clear",
        r"center|centre core only",
        r"setpoint speed (slow|medium|fast)",
        r"dev sp (\d+)",
        r"hold rods|setpoint hold",
        r"drain valve (\d+)",
    ])
    .unwrap();
    let matches: Vec<_> = set.matches(command).into_iter().collect();
    //println!("{:?}", matches);
    match matches[..] {
        [0] => {
            let re = Regex::new(r"insert rod (\d+)").unwrap();
            let cap = re.captures(command).unwrap();
            let rod = cap[1].parse::<usize>().unwrap();
            let amount = mainstruct.absorber_rods.len() * mainstruct.absorber_rods[0].len();
            if rod > amount {
                //println!("rod number too high");
            } else {
                let row = rod / mainstruct.absorber_rods[0].len();
                let col = rod % mainstruct.absorber_rods[0].len();
                mainstruct.absorber_rods[row][col - 1].insert_rod = true;
            }
        }
        [1] => {
            // set all fuel rods to insert = false
            for i in 0..mainstruct.absorber_rods.len() {
                for j in 0..mainstruct.absorber_rods[i].len() {
                    mainstruct.absorber_rods[i][j].insert_rod = false;
                }
            }
        }
        [2] => {
            // set all fuel rods to insert = true
            for i in 0..mainstruct.absorber_rods.len() {
                for j in 0..mainstruct.absorber_rods[i].len() {
                    mainstruct.absorber_rods[i][j].insert_rod = true;
                }
            }
        }
        [3] => {
            // set all fuel rods to insert = false
            for i in 0..mainstruct.absorber_rods.len() {
                for j in 0..mainstruct.absorber_rods[i].len() {
                    mainstruct.absorber_rods[i][j].insert_rod = false;
                }
            }
        }
        [4] => {
            let re = Regex::new(r"remove rod (\d+)").unwrap();
            let cap = re.captures(command).unwrap();
            let rod = cap[1].parse::<usize>().unwrap();
            let amount = mainstruct.absorber_rods.len() * mainstruct.absorber_rods[0].len();
            if rod > amount {
                //println!("rod number too high");
            } else {
                let row = rod / mainstruct.absorber_rods[0].len();
                let col = rod % mainstruct.absorber_rods[0].len();
                mainstruct.absorber_rods[row][col - 1].insert_rod = false;
            }
        }
        [5] => {
            let re = Regex::new(r"setpoint (\d+)").unwrap();
            let cap = re.captures(command).unwrap();
            let setpoint = cap[1].parse::<f32>().unwrap();
            // set all fuel rods to setpoint
            for i in 0..mainstruct.absorber_rods.len() {
                for j in 0..mainstruct.absorber_rods[i].len() {
                    mainstruct.absorber_rods[i][j].set_point = setpoint;
                }
            }
        }
        [6] => {
            let re = Regex::new(r"set rod (\d+) to (\d+)").unwrap();
            let cap = re.captures(command).unwrap();
            let rod = cap[1].parse::<usize>().unwrap();
            let amount = mainstruct.absorber_rods.len() * mainstruct.absorber_rods[0].len();
            if rod > amount {
                //println!("rod number too high");
            } else {
                let row = rod / mainstruct.absorber_rods[0].len();
                let col = rod % mainstruct.absorber_rods[0].len();
                mainstruct.absorber_rods[row][col - 1].set_point = cap[2].parse::<f32>().unwrap();
            }
        }
        [7] => {
            // split into pages based on the height of the chunk
            mainstruct.data.log.clear();
            let help_strings = vec![
                "help <page> - display this page",
                "insert rod <rod number> - insert a fuel rod",
                "remove rod <rod number> - remove a fuel rod",
                "insert rods - insert all fuel rods",
                "pull rods - remove all fuel rods",
                "scram - remove all fuel rods",
                "setpoint <setpoint> - set all fuel rods to a setpoint",
                "set rod <rod number> to <setpoint> - set a fuel rod to a setpoint",
                "cls - clear the log",
                "center core only - insert only the center core",
                "setpoint speed <slow|medium|fast> - set the speed of the setpoint change",
                "dev sp <position> - change the position of the absorber rods to position",
                "hold rods - hold the rods in place",
            ];
            let re = Regex::new(r"help (\d+)").unwrap();
            let cap = re.captures(command).unwrap();
            let page = cap[1].parse::<u16>().unwrap();
            //println!("page: {}", page);
            let start = (page - 1) * (height - 2);
            let end = (page) * (height - 2);
            //println!("start: {}, end: {}", start, end);
            for i in start..end {
                if i as usize >= help_strings.len() {
                    break;
                } else {
                    mainstruct
                        .data
                        .log
                        .insert(0, help_strings[i as usize].to_string());
                }
            }
        }
        [8] => {
            mainstruct.data.log.clear();
        }
        [9] => {
            // center core only where a 5x5 grid the centre is 3x3
            let length = mainstruct.absorber_rods.len();
            let width = mainstruct.absorber_rods[0].len();
            for i in 0..length {
                if i > 0 && i < (length - 1) {
                    for j in 0..width {
                        if j > 0 && j < (length - 1) {
                            mainstruct.absorber_rods[i][j].insert_rod = true;
                        } else {
                            mainstruct.absorber_rods[i][j].insert_rod = false;
                        }
                    }
                } else {
                    for j in 0..width {
                        mainstruct.absorber_rods[i][j].insert_rod = false;
                    }
                }
            }
        }
        [10] => {
            let re = Regex::new(r"setpoint speed (slow|s|medium|m|fast|f)").unwrap();
            let cap = re.captures(command).unwrap();
            let speed = cap[1].parse::<String>().unwrap();

            mainstruct.core.speed_setpoint = match speed.as_str() {
                "slow" => 0.1,
                "s" => 0.1,
                "medium" => 0.5,
                "m" => 0.5,
                "fast" => 1.0,
                "f" => 1.0,
                _ => 0.5,
            };
        }
        [11] => {
            let re = Regex::new(r"dev sp (\d+)").unwrap();
            let cap = re.captures(command).unwrap();
            let setpoint = cap[1].parse::<f32>().unwrap();
            for i in 0..mainstruct.absorber_rods.len() {
                for j in 0..mainstruct.absorber_rods[i].len() {
                    mainstruct.absorber_rods[i][j].absorber_rod_position = setpoint;
                }
            }
        }
        [12] => {
            mainstruct.absorber_rods.par_iter_mut().for_each(|row| {
                row.par_iter_mut().for_each(|cell| {
                    cell.set_point = cell.absorber_rod_position;
                })
            });

        }
        [13] => {
            let re = Regex::new(r"drain valve (\d+)").unwrap();
            let cap = re.captures(command).unwrap();
            let valve = cap[1].parse::<f32>().unwrap();
            mainstruct.core.drain_valve = valve;
        }

        _ => {
            //println!("no match");
        }
    }
}
