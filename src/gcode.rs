use std::{fs, path::PathBuf};

pub fn calculate_end_pos(start_pos: &Pos2D, tube_width: f32, cut_angle: f32, overshoot_amount: f32) -> Pos2D {
    let end_x = tube_width + overshoot_amount;
    let mut end_y = start_pos.y.clone();

    if cut_angle != 0.0 {
        end_y = (tube_width + overshoot_amount) / cut_angle.to_radians().tan();
    }

    return Pos2D::new(end_x, end_y);
}

// units enum
#[derive(PartialEq)]
pub enum DistUnit {
    Metric, // mm
    Imperial, // inches
}

// positioning mode enum, has absolute and relative

pub enum PositioningMode {
    Absolute,
    Relative,
}


pub struct Gcode {
    pub gcode_string: String,
}

pub struct Pos2D {
    pub x: f32,
    pub y: f32,
}

impl Pos2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Gcode {
    pub fn new() -> Self {
        let mut gcode = Gcode { gcode_string: String::new() };
        gcode.set_units_to_mm();
        gcode.set_positioning_mode(PositioningMode::Absolute);
        return gcode;
    }

    // add a command with no comment
    pub fn add_command(&mut self, g_command: String) {
        self.gcode_string += &format!("{}\n", g_command);
    }

    // add a command with a comment
    pub fn add_command_comment(&mut self, g_command: String, g_comment: String) {
        self.gcode_string += &format!("{}       ({}) \n", g_command, g_comment);
    }

    // set the units
    fn set_units_to_mm(&mut self) {
        self.add_command_comment("G21".to_owned(), "set units to mm".to_owned());
    }

    // set positioning mode of gcode
    pub fn set_positioning_mode(&mut self, positioning_mode: PositioningMode) {
        match positioning_mode {
            PositioningMode::Absolute => {self.add_command_comment("G90".to_owned(), "set positioning to absolute".to_owned());},
            PositioningMode::Relative => {self.add_command_comment("G91".to_owned(), "set positioning to relative".to_owned());}
        }
    }

    // move to specified x and y positions
    pub fn move_xy(&mut self, new_pos: &Pos2D, feedrate: f32) {
        let g_command = format!("G1 X{} Y{} F{}", new_pos.x, new_pos.y, feedrate);
        let g_comment = format!("move to X: {}, Y: {} with feedrate: {}", new_pos.x, new_pos.y, feedrate);
        self.add_command_comment(g_command, g_comment);
    }

    // home command, moves toolhead to 0, 0
    pub fn home2D(&mut self, feedrate: f32) {
        self.move_xy(&Pos2D::new(0.0, 0.0), feedrate);
    }

    // dwell command waits specified seconds
    pub fn dwell(&mut self, dwell_time: f32) {
        let g_command = format!("G4 P{}", dwell_time);
        let g_comment = format!("wait {} seconds", dwell_time);
        self.add_command_comment(g_command, g_comment);
    }

    pub fn set_plasma_enabled(&mut self, enabled: bool) {
        if enabled {
            self.add_command_comment("M3".to_owned(), "set plasma enabled".to_owned());
        } else {
            self.add_command_comment("M5".to_owned(), "set plasma disabled".to_owned());
        }
    }

    // write the gcode to a specified file
    pub fn write_to_file(&self, filename: PathBuf) {
        fs::write(filename, &self.gcode_string);
    }
    
}