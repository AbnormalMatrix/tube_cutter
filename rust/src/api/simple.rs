
use serial2;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_serial_ports() -> Vec<String> {
    let ports = serial2::SerialPort::available_ports().unwrap().into_iter().map(|p| p.to_string_lossy().into_owned()).collect();
    return ports
}

