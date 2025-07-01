use pest::Parser;
use pest_derive::Parser;

use crate::api::gcode::Pos2D;

#[derive(Parser)]
#[grammar = "./api/status.pest"]
struct StatusParser;
#[derive(Clone)]
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
    pub position: Pos2D,
    pub machine_state: MachineState
}
impl MachineStatus {
    pub fn new() -> MachineStatus {
        MachineStatus { position: Pos2D::new(0.0, 0.0), machine_state: MachineState::Idle }
    }
}

pub fn parse_status(status_string: String) -> MachineStatus {
    let parsed_msg = StatusParser::parse(Rule::status, &status_string).expect("Failed to parse status").next().unwrap();

    let mut machine_status = MachineStatus { position: Pos2D::new(0.0, 0.0), machine_state: MachineState::Idle };

    for part in parsed_msg.into_inner() {
        match part.as_rule() {
            Rule::state => {
                // match the machine state string to the MachineState enum
                match part.as_str() {
                    "Idle" => { machine_status.machine_state = MachineState::Idle },
                    "Run" => { machine_status.machine_state = MachineState::Run },
                    "Hold" => { machine_status.machine_state = MachineState::Hold },
                    "Jog" => { machine_status.machine_state = MachineState::Jog },
                    "Alarm" => { machine_status.machine_state = MachineState::Alarm },
                    "Door" => { machine_status.machine_state = MachineState::Door },
                    "Check" => { machine_status.machine_state = MachineState::Check },
                    "Home" => { machine_status.machine_state = MachineState::Home },
                    "Sleep" => { machine_status.machine_state = MachineState::Sleep },
                    "Tool" => { machine_status.machine_state = MachineState::Tool },
                    _ => { /* handle unknown state if necessary */ },
                }
            },

            Rule::mpos => {
                // get the x and y positions by getting the inner rules
                let mut inner_rules = part.into_inner();
                let pos_x: f32 = inner_rules.next().unwrap().as_str().parse().unwrap();
                let pos_y: f32 = inner_rules.next().unwrap().as_str().parse().unwrap();
                machine_status.position = Pos2D::new(pos_x, pos_y); 

            },
            _ => {}
        }
    }
    
    return machine_status;
}