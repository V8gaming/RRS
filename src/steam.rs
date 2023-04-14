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
    let _superheat_degrees =
        SUPERHEAT - (SATMAX - SATMIN) * mainstruct.core.steam.steam_pressure / 6.9;

    steam_core_turbine(mainstruct);
    steam_turbine_outside(mainstruct);
    deaerator_process(mainstruct);
    condenser_process(mainstruct);

    // flowrate based on diameter of the pipe times drain valve position times velocity
    //mainstruct.core.steam.steam_flow_rate = mainstruct.core.drain_valve * diameter;
}
fn steam_core_turbine(mainstruct: &mut MainStruct) {
    mainstruct.core.steam.steam_flow_rate = calculate_flowrate(
        mainstruct.core.steam.steam_pressure,
        mainstruct.turbine.steam_pressure,
        mainstruct.core.drain_valve,
    );

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
        mainstruct.core.steam.steam_pressure += pressure_change / 500.0;
    }
}
fn steam_turbine_outside(mainstruct: &mut MainStruct) {
    let atmospheric_pressure_mpa = 0.101325;
    mainstruct.turbine.steam_flow_rate = calculate_flowrate(
        mainstruct.turbine.steam_pressure,
        atmospheric_pressure_mpa,
        mainstruct.turbine.steam_drain_valve,
    );
    mainstruct.turbine.steam_pressure -= mainstruct.turbine.steam_flow_rate;
    if mainstruct.turbine.steam_pressure < 0.0 {
        mainstruct.turbine.steam_pressure = 0.0;
    }
}

pub fn calculate_density(steam_pressure: f32) -> f32 {
    const DENMIN: f32 = 0.251560; // density of steam at 0MPa
    const DENMAX: f32 = 18.2339; // density of steam at 6.9MPa
    DENMIN + (DENMAX - DENMIN) * steam_pressure / 6.9
}
pub fn calculate_flowrate(steam_pressure: f32, location_pressure: f32, drain_valve: f32) -> f32 {
    let density = calculate_density(steam_pressure);
    let specific_gravity = density / 1000.0;
    let condenser_pressure = 4.2058; // pressure of the turbine at no load in MPa

    let differential_pressure = steam_pressure - (condenser_pressure + location_pressure);

    (differential_pressure.abs() / specific_gravity).sqrt() * (drain_valve) / 10000.0
}

fn deaerator_process(mainstruct: &mut MainStruct) {
    // Constants
    const DEGAS_RATE: f32 = 0.1; // Adjust this constant based on the actual degas rate in the system
    const WATER_LEVEL_MIN: f32 = 0.0;
    const WATER_LEVEL_MAX: f32 = 100.0;
    const FEEDWATER_PUMP_COEFFICIENT: f32 = 0.5; // Adjust this constant based on the actual feedwater pump characteristics

    // Steam input
    let steam_input = mainstruct.turbine.steam_flow_rate * DEGAS_RATE;

    // Update deaerator pressure and temperature based on the steam input
    mainstruct.deaerator.pressure += steam_input * 0.001; // Update this calculation based on the actual relationship between steam input and pressure
    mainstruct.deaerator.temperature += steam_input * 0.01; // Update this calculation based on the actual relationship between steam input and temperature

    // Feedwater pump operation
    let feedwater_flow_rate = mainstruct.deaerator.pressure * FEEDWATER_PUMP_COEFFICIENT;

    // Update deaerator water level based on the steam input and feedwater pump operation
    mainstruct.deaerator.water_level += steam_input - feedwater_flow_rate;

    // Ensure water level stays within limits
    if mainstruct.deaerator.water_level < WATER_LEVEL_MIN {
        mainstruct.deaerator.water_level = WATER_LEVEL_MIN;
    }
    if mainstruct.deaerator.water_level > WATER_LEVEL_MAX {
        mainstruct.deaerator.water_level = WATER_LEVEL_MAX;
    }
}


fn condenser_process(mainstruct: &mut MainStruct) {
    // Constants
    const COOLING_WATER_FLOW_RATE: f32 = 500.0; // Adjust this constant based on the actual cooling water flow rate in the system
    const HEAT_TRANSFER_COEFFICIENT: f32 = 0.001; // Adjust this constant based on the actual heat transfer coefficient in the system
    const MIN_CONDENSER_PRESSURE: f32 = 0.001;
    
    // Steam input from the turbine
    let steam_input = mainstruct.turbine.steam_flow_rate;

    // Update condenser pressure based on the steam input
    mainstruct.condenser.pressure += steam_input * 0.001; // Update this calculation based on the actual relationship between steam input and pressure
    if mainstruct.condenser.pressure < MIN_CONDENSER_PRESSURE {
        mainstruct.condenser.pressure = MIN_CONDENSER_PRESSURE;
    }

    // Update condenser temperature based on the heat transfer between steam and cooling water
    mainstruct.condenser.temperature += steam_input * HEAT_TRANSFER_COEFFICIENT * (mainstruct.deaerator.temperature - mainstruct.condenser.temperature);

    // Cooling water system
    mainstruct.condenser.cooling_water_flow_rate = COOLING_WATER_FLOW_RATE;
    mainstruct.condenser.heat_transfer_coefficient = HEAT_TRANSFER_COEFFICIENT;
}
