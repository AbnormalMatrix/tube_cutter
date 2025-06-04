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

}