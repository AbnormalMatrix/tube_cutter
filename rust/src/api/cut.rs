use crate::api::gcode::calculate_end_pos;

use super::gcode::Pos2D;


// the cut struct has info like start and end positions, cut angle and so on
#[flutter_rust_bridge::frb(opaque)]
pub struct Cut {
    pub start_position: Pos2D,
    pub tube_width: f32,
    pub end_position: Pos2D,
    pub cut_angle: f32,
    pub cut_feedrate: f32,
    pub pierce_delay: f32,
    pub pierce_delay_2: f32,
}

#[flutter_rust_bridge::frb]
pub struct TestStruct {
    pub test_a: f32,
}

impl Cut {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new() -> Self {
        Self { 
            start_position: Pos2D::new(0.0, 0.0),
            tube_width: 25.0,
            end_position: Pos2D::new(0.0, 0.0),
            cut_angle: 90.0,
            cut_feedrate: 1000.0,
            pierce_delay: 0.5,
            pierce_delay_2: 0.25,
        }
    }

    // setters (basically just for the ui)
    #[flutter_rust_bridge::frb(sync)]
    pub fn set_tube_width(&mut self, new_width: f32) {
        self.tube_width = new_width;
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn set_cut_angle(&mut self, new_angle: f32) {
        self.cut_angle = new_angle;
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn set_cut_feedrate(&mut self, new_feedrate: f32) {
        self.cut_feedrate = new_feedrate;
    }
    
    #[flutter_rust_bridge::frb(sync)]
    pub fn set_pierce_delay(&mut self, new_delay: f32) {
        self.pierce_delay = new_delay;
    }
    
    #[flutter_rust_bridge::frb(sync)]
    pub fn set_pierce_delay_2(&mut self, new_delay: f32) {
        self.pierce_delay_2 = new_delay;
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn get_end_pos(&self) -> (f32, f32) {
        let end_pos = calculate_end_pos(&self.start_position, self.tube_width, self.cut_angle, 1.0, true);
        return (end_pos.x, end_pos.y);
    }

}