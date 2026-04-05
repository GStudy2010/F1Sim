use std::string;

#[derive(Debug)]
pub enum FuelType {
    Gasoline,
    Diesel,
}
#[derive(Debug)]
pub enum EngineError {
    InvalidFuelType(String),
    InvalidRPM(f64),
}
impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EngineError::InvalidFuelType(s) => write!(f, "Unknown fuel type: '{}'", s),
            EngineError::InvalidRPM(i) => write!(f, "Invalid RPM: '{}'", i),
        }
    }
}
#[derive(Debug)]
pub struct Engine {

    pub displacement: f64,   // How many liters engine has
    pub cylinders: u8,       // How many cylinders engine has
    pub comp_ratio: f64,     // Ration of max to min cylider volume
    pub fuel_type: FuelType, // Fuel type

    pub max_rpm: f64,                  // Max rpm engine can deal with
    pub idle_rpm: f64,                 // The rpm when engine is running under no pressure
    pub current_rpm: f64,              // Current rpm
    pub max_hp: f64,                   // Horse power of the engine
    pub max_torque: f64,               // Max torque of the engine
    pub torque_curve: Vec<(f64, f64)>, // Curve of the torque of the engine

    pub running: bool,   // If engine is running
    pub throtle: f64,    // amount of throttle 0.0->1.0
    pub fuel_level: f64, // fuel_level in % 0.0%->100.0%
}
impl Engine {
    pub fn new(c: u8, max_r: f64, max_h: f64, max_t: f64, eng_s: f64, fuel: String) -> Result<Engine, EngineError>{
        let f_t = match fuel.as_str() {
            "gasoline" => FuelType::Gasoline,
            "diesel"   => FuelType::Diesel,
            other      => return Err(EngineError::InvalidFuelType(other.to_string())),
        };

        Ok(Engine { 
        displacement: eng_s,
        cylinders: c,
        comp_ratio: 1.0,
        fuel_type: f_t,
        max_rpm: max_r,
        idle_rpm: 0.0,
        current_rpm: 0.0,
        max_hp: max_h,
        max_torque: max_t,
        torque_curve: Vec::new(),
        running: false,
        throtle: 0.0,
        fuel_level: 100.0
        })
    }
}
fn main() {
    // Creating an engine.
    match Engine::new(4, 5000.0, 100.0, 200.0, 3.0, "gasoline".to_string()) {
        Ok(engine) => println!("Created an engine: {:?}", engine),
        Err(e)     => println!("Error while creating an engine: {}", e),
    }
}
