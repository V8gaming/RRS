use crate::ARCFM::AbsorberRodControlAndFuelMonitoring;
pub struct Menus {
    absorber_rod_control_and_fuel_monitoring: AbsorberRodControlAndFuelMonitoring,
}
impl Default for Menus {
    fn default() -> Self {
        Self {
            absorber_rod_control_and_fuel_monitoring: AbsorberRodControlAndFuelMonitoring::default(),
        }
    }
}
