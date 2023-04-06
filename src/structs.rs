use tui::{style::Color, widgets::ListItem};
pub struct FuelRodData {
    fuel_pellet: FuelPellet,
    cladding: Cladding,
}
pub struct FuelPellet {
    /// U235 and U238 composition
    /// OM = Oxide mass
    /// density = g/cm^3
    /// Diameter = cm
    /// Stack_length = m
    u_composition: UComposition,
    om: f32,
    density: f32,
    diameter: f32,
    stack_length: f32,
}

pub struct UComposition {
    u235_composition: f32,
    u238_composition: f32,
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
            u235_composition: 0.035,
            u238_composition: 0.965,
        }
    }
}
impl Default for FuelPellet {
    fn default() -> Self {
        Self {
            u_composition: UComposition::default(),
            om: 3.0,
            density: 10.7,
            diameter: 7.62,
            stack_length: 3.65,
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
                fuel_pellet: FuelPellet::default(),
                cladding: Cladding::default(),
            },
            distance_between_c_and_f_rods: 1.25,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Core {
    pub width: u16,
    pub height: u16,
    // slow, medium, fast
    pub speed_setpoint: f32,
    pub neutron_flux: f32,
    pub thermal_power: f32,
    pub steam: Steam,
    pub rate_of_change: f32,
    pub drain_valve: f32,
    pub drain_setpoint: f32,
    pub selected_rod: usize,
}
impl Default for Core {
    fn default() -> Self {
        Self {
            speed_setpoint: 1.0,
            width: 5,
            height: 5,
            neutron_flux: 0.0,
            thermal_power: 0.0,
            steam: Steam::default(),
            rate_of_change: 0.0,
            drain_valve: 0.0,
            drain_setpoint: 0.0,
            selected_rod: 0,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Turbine {
    /// turbine_speed in RPM (0-3600)
    /// speed_step in RPM (0(stop), 900, 1800, 2700, 3600)
    /// steam_drain_valve in % (0-100)
    /// steam_flow_rate in kg/s
    /// steam_pressure in MPa
    /// turning_gear enabled/disabled
    /// setpoint_speed in RPM (0-3600)
    /// pressure_setpoint in MPa
    pub turbine_speed: f32,
    pub speed_setpoint_step: u8,
    pub steam_drain_valve: f32,
    pub steam_flow_rate: f32,
    pub steam_pressure: f32,
    pub turning_gear: bool,
    pub setpoint_speed: f32,
    pub pressure_setpoint: f32,
}
impl Default for Turbine {
    fn default() -> Self {
        Self {
            turbine_speed: 0.0,
            speed_setpoint_step: 0,
            steam_drain_valve: 100.0,
            steam_flow_rate: 0.0,
            steam_pressure: 0.0,
            turning_gear: false,
            setpoint_speed: 0.0,
            pressure_setpoint: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MainStruct {
    pub core: Core,
    pub absorber_rods: Vec<Vec<FuelRod>>,
    pub data: Data,
    pub turbine: Turbine,
}
impl Default for MainStruct {
    fn default() -> Self {
        Self {
            core: Core::default(),
            absorber_rods: vec![vec![FuelRod::default(); 5]; 5],
            data: Data::default(),
            turbine: Turbine::default(),
        }
    }
}
type Item<'a> = (ListItem<'a>, Vec<ListItem<'a>>, bool);

#[derive(Clone, Debug)]
pub struct Data {
    pub graphs: Vec<Vec<(f64, f64)>>,
    pub reactivity: f32,
    pub neutron_flux: f32,
    pub neutron_rate: f32,
    pub log: Vec<String>,
    pub left_tab_index: usize,
    pub left_tab_length: i32,
    pub checklist_selected: usize,
    pub checklist_length: usize,
    pub text_input: bool,
    pub items: Vec<Item<'static>>,
    pub selected_item: ListItem<'static>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            graphs: vec![
                vec![(0.0, 0.0); 2],
                vec![(0.0, 0.0); 2],
                vec![(-15.0, 0.0); 2],
                vec![(0.0, 0.0); 2],
                vec![(0.0, 0.0); 2],
            ],
            reactivity: 0.0,
            neutron_flux: 0.0,
            neutron_rate: -15.0,
            log: Vec::new(),
            left_tab_index: 0,
            left_tab_length: 3,
            checklist_selected: 1,
            checklist_length: 0,
            text_input: true,
            items: vec![
                (
                    ListItem::new("Core"),
                    vec![
                        ListItem::new("    Reactivity"),
                        ListItem::new("    Neutron Flux"),
                        ListItem::new("    Neutron Rate"),
                    ],
                    false,
                ),
                (
                    ListItem::new("Turbine"),
                    vec![
                        ListItem::new("    Turbine Speed"),
                        ListItem::new("    Steam Flow Rate"),
                        ListItem::new("    Steam Pressure"),
                    ],
                    false,
                ),
                (
                    ListItem::new("Steam"),
                    vec![
                        ListItem::new("    Steam Temperature"),
                        ListItem::new("    Steam Pressure"),
                        ListItem::new("    Steam Flow Rate"),
                    ],
                    false,
                ),
            ],
            selected_item: ListItem::new("Core"),
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
    pub neighbors: ([(u16, u16); 4], [bool; 4]),
    pub temperature_color: Color,
}
impl Default for FuelRod {
    fn default() -> Self {
        Self {
            absorber_rod_position: 100.0,
            fuel_temperature: 0.0,
            thermal_power_output: 0.0,
            insert_rod: true,
            set_point: 0.0,
            reactivity: 0.0,
            neutron_rate: 0.0,
            neighbors: (
                [(0, 0), (0, 0), (0, 0), (0, 0)],
                [false, false, false, false],
            ),
            temperature_color: Color::Reset,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Steam {
    /// steam flow rate in kg/s
    /// steam pressure in MPa
    /// steam temperature in C
    /// feedwater flow rate in kg/s
    /// feedwater temperature in C
    /// thermodynamic cycle
    pub steam_flow_rate: f32,
    pub steam_pressure: f32,
    pub steam_temperature: f32,
    pub feedwater_flow_rate: f32,
    pub feedwater_temperature: f32,
    pub thermodynamic_cycle: String,
}
impl Default for Steam {
    fn default() -> Self {
        Self {
            steam_flow_rate: 0.0,
            steam_pressure: 0.0,
            steam_temperature: 0.0,
            feedwater_flow_rate: 0.0,
            feedwater_temperature: 0.0,
            thermodynamic_cycle: "Rankine".to_string(),
        }
    }
}
