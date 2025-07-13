use std::{fs, path::Path};
use serde::{Deserialize, Serialize};


// settings needed:
// cut method (straight/split)
// use laser (on/off)
// laser offset (x,y)
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[flutter_rust_bridge::frb(opaque)]
#[flutter_rust_bridge::frb]
pub enum CutMethod {
    Straight,
    Split,
}

#[derive(Serialize, Deserialize)]
#[flutter_rust_bridge::frb(opaque)]
#[flutter_rust_bridge::frb]
pub struct CutterSettings {
    pub cut_method: CutMethod,
    pub use_laser: bool,
    pub laser_offset_x: f32,
    pub laser_offset_y: f32,
}


impl CutterSettings {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new() -> Self {
        return CutterSettings::load();
        
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn save(&self) {
        let settings_path = Path::new("prefs.json");
        // save the file
        let contents = serde_json::to_string(self).expect("Failed to serialize settings!");
        fs::write(settings_path, contents).expect("Failed to write settings file!");
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn load() -> Self {
        // check if file exists
        let settings_path = Path::new("prefs.json");
        if settings_path.exists() {
            let contents = fs::read_to_string(settings_path).expect("Failed to read settings file!");
            let settings: Self = serde_json::from_str(&contents).expect("Failed to parse settings file!");
            return settings;
        } else {
            let settings = Self { cut_method: CutMethod::Split, use_laser: false, laser_offset_x: 0.0, laser_offset_y: 0.0 };
            // save the new file
            let contents = serde_json::to_string(&settings).expect("Failed to serialize settings!");
            fs::write(settings_path, contents).expect("Failed to write settings file!");
            return settings;
        }
    }


}