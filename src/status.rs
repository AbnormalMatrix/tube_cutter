use pest::Parser;
use pest_derive::Parser;

use crate::gcode::Pos2D;

#[derive(Parser)]
#[grammar = "status.pest"]
struct StatusParser;

pub enum MachineState {
    Idle,
    Run,
    Hold,
    Jog,
    Alarm,
    Door,
    Check,
    Home,
    Sleep,
    Tool
}
pub struct MachineStatus {
    position: Pos2D,
    machine_state: MachineState
}
pub fn parse_status(status_string: String) -> MachineStatus {
    
    MachineStatus { position: Pos2D::new(0.0, 0.0), machine_state: MachineState::Idle }
}