use crate::structs::MainStruct;


pub fn steam(mainstruct: &mut MainStruct) {
    // calculate steam pressure in MPa, approx FP: 6.9MPa (100%)
    // calculate steam flow rate in kg/s
    // calculate steam temperature in C

    steam_production(mainstruct.core.steam.steam_pressure, mainstruct);
    //mainstruct.data.log.push(format!("{}", mainstruct.core.steam.steam_pressure));
    // calulate steam temperature between 100 and 600(superheated) C
    const SATMIN: f32 = 99.9743; // saturation temperature at 0MPa
    const SATMAX: f32 = 286.803; // saturation temperature at 6.9MPa
    const SUPERHEAT: f32 = 600.0; // superheated temperature

    // superheat - saturation temperature at the current pressure
    let superheat_degrees = SUPERHEAT - (SATMAX - SATMIN) * mainstruct.core.steam.steam_pressure / 6.9;

    steam_core_turbine(mainstruct);
    steam_turbine_outside(mainstruct)

    // flowrate based on diameter of the pipe times drain valve position times velocity
    //mainstruct.core.steam.steam_flow_rate = mainstruct.core.drain_valve * diameter;

}
fn steam_core_turbine(mainstruct: &mut MainStruct) {
    mainstruct.core.steam.steam_flow_rate = calculate_flowrate(mainstruct.core.steam.steam_pressure, mainstruct.turbine.steam_pressure, mainstruct.core.drain_valve);
    
    //let flow_factor = mainstruct.core.drain_valve; // flow factor of the pipe
    //mainstruct.data.log.push(format!("D: {}, PR: {}, DP: {}, FF: {}, FR: {}", density, mainstruct.core.steam.steam_pressure, differential_pressure, flow_factor, mainstruct.core.steam.steam_flow_rate));
    mainstruct.core.steam.steam_pressure -= mainstruct.core.steam.steam_flow_rate;
    if mainstruct.core.steam.steam_pressure < 0.0 {
        mainstruct.core.steam.steam_pressure = 0.0;
    }
    mainstruct.turbine.steam_pressure += mainstruct.core.steam.steam_flow_rate;
}

fn steam_production(steam_pressure: f32, mainstruct: &mut MainStruct) {
    // pressure coefficient based on the current pressure
    let pressure_coefficient = steam_pressure * 0.15 + 1.0;
    // pressure change based on the current thermal power
    let pressure_change = (mainstruct.core.thermal_power / 100.0 * 600.0 - 100.0).max(0.0) * 0.3
        / pressure_coefficient;
    
    if (mainstruct.core.thermal_power / 100.0 * 600.0 - 100.0) > 0.0 {
        mainstruct.core.steam.steam_pressure += pressure_change /500.0;
    }
}
fn steam_turbine_outside(mainstruct: &mut MainStruct) {
    let atmospheric_pressure_mpa = 0.101325;
    mainstruct.turbine.steam_flow_rate = calculate_flowrate(mainstruct.turbine.steam_pressure, atmospheric_pressure_mpa, mainstruct.turbine.steam_drain_valve);
    mainstruct.turbine.steam_pressure -= mainstruct.turbine.steam_flow_rate;
    if mainstruct.turbine.steam_pressure  < 0.0 {
        mainstruct.turbine.steam_pressure = 0.0;
    }
}

pub fn calculate_density(steam_pressure: f32) -> f32 {
    const DENMIN: f32 = 0.251560; // density of steam at 0MPa
    const DENMAX: f32 = 18.2339; // density of steam at 6.9MPa

    let density = DENMIN + (DENMAX - DENMIN) * steam_pressure / 6.9;
    density
}
pub fn calculate_flowrate(steam_pressure: f32, location_pressure: f32, drain_valve: f32) -> f32 {

    let density = calculate_density(steam_pressure);
    let specific_gravity = density /1000.0;
    let condenser_pressure = 4.2058; // pressure of the turbine at no load in MPa

    let differential_pressure = steam_pressure- (condenser_pressure+ location_pressure);

    let flowrate = (differential_pressure.abs()/specific_gravity).sqrt() * (drain_valve)/ 10000.0;

    
    flowrate
}