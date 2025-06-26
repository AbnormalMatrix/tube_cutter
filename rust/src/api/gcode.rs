use std::{fs, path::PathBuf};

use super::cut::Cut;


pub fn calculate_end_pos(start_pos: &Pos2D, tube_width: f32, cut_angle: f32, overshoot_amount: f32, cut_right: bool) -> Pos2D {
    let mut end_x = 0.0;
    if cut_right {
        end_x = start_pos.x + tube_width + overshoot_amount;
    } else {
        end_x = start_pos.x - tube_width - overshoot_amount;
    } 
    
    let mut end_y = start_pos.y.clone();
    
    if cut_angle != 0.0 {
        end_y = (tube_width + overshoot_amount) / cut_angle.to_radians().tan();
    }

    return Pos2D::new(end_x, end_y);
}

pub fn get_midpoint(start_pos: &Pos2D, end_pos: &Pos2D) -> Pos2D {
    let avg_x = (start_pos.x + end_pos.x) / 2.0;
    let avg_y = (start_pos.y + end_pos.y) / 2.0;

    return Pos2D::new(avg_x, avg_y);
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

#[flutter_rust_bridge::frb(opaque)]
pub struct Gcode {
    pub gcode_string: String,
}

#[derive(Clone)]
#[flutter_rust_bridge::frb(opaque)]
pub struct Pos2D {
    pub x: f32,
    pub y: f32,
}

impl Pos2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_screen_space(&self, tube_width: &f32, scale_factor: &f32) -> Pos2D {
        // the origin is offset to the left by half the tube with * the scale factor
        let origin = Pos2D::new(-(tube_width/2.0) *scale_factor, 0.0); // origin screen space
        let screen_position = Pos2D::new((self.x * scale_factor) + origin.x, self.y*scale_factor);
        return screen_position;
    }
}

impl Gcode {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new() -> Self {
        let mut gcode = Gcode { gcode_string: String::new() };
        gcode.set_units_to_mm();
        gcode.set_positioning_mode(PositioningMode::Absolute);
        return gcode;
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn get_gcode_string(&self) -> String {
        return self.gcode_string.clone();
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

    // sets machine's 0 to current position
    pub fn set_zero(&mut self) {
        self.add_command_comment("G10 P0 L20 X0 Y0 Z0".to_owned(), "set machine zero".to_owned());
    }
    
    // write the gcode to a specified file
    pub fn write_to_file(&self, filename: String) {
        fs::write(filename, &self.gcode_string);
    }

    // add a cut to the gcode

    #[flutter_rust_bridge::frb(sync)]
    pub fn add_cut(&mut self, tube_cut: Cut) {
        // calculate the end position

        let real_start = Pos2D::new(&tube_cut.start_position.x + 40.0, tube_cut.start_position.y);

        let end_position = calculate_end_pos(&real_start, tube_cut.tube_width, tube_cut.cut_angle, 1.0, true);

        // move by the laser offset distance
        self.move_xy(&real_start, tube_cut.cut_feedrate);

        let midpoint = get_midpoint(&real_start, &end_position);

        // goto the midpoint
        self.move_xy(&midpoint, tube_cut.cut_feedrate);

        // enable plasma
        self.set_plasma_enabled(true);
        // pierce delay
        self.dwell(tube_cut.pierce_delay);
        // do the movement
        self.move_xy(&end_position, tube_cut.cut_feedrate);
        // disable the plasma
        self.set_plasma_enabled(false);

        // wait for cutter to stop blowing
        self.dwell(2.0);
        // go back to the midpoint
        self.move_xy(&midpoint, tube_cut.cut_feedrate);

        // enable plasma
        self.set_plasma_enabled(true);
        // pierce delay
        self.dwell(tube_cut.pierce_delay);
        // do the movement
        self.move_xy(&real_start, tube_cut.cut_feedrate);
        self.set_plasma_enabled(false);
        self.move_xy(&tube_cut.start_position, tube_cut.cut_feedrate);

    }
    
}

#[flutter_rust_bridge::frb(sync)]
pub fn jog(x_dist: f32, y_dist: f32) -> String {
    format!("$J=G91 G21 X{} Y{} F600", x_dist, y_dist)
}