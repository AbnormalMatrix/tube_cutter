use std::sync::{mpsc::{Receiver, Sender}, Arc, Mutex};
use tokio::task;

use nannou_egui::egui;
use serial2::SerialPort;

use crate::{gcode, Settings};

// struct to store info/status about the physical machine
// example of idle status report: <Idle|MPos:0.000,0.000,0.000|Bf:35,1023|FS:0,0|Pn:XYZ>
struct MachineInfo {
    position: gcode::Pos2D,
    idle: bool,
}

pub fn get_port_list(ui: &mut egui::Ui) {
    
    // get the avalible serial ports
    if let Ok(ports) = SerialPort::available_ports() {
        // iterate through the ports and list them in the UI
        for port in ports {
            ui.label(format!("{}", port.display()));
        }
    } else {
        ui.label("No serial ports detected.");
    }

}

pub fn make_connection_button(ui: &mut egui::Ui, settings: &mut Settings, command_status: Arc<Mutex<CommandStatus>>) {
    if settings.serial_rx.is_none() {
        if ui.button("Connect").clicked() {
            let (to_gui_tx, from_serial_rx) = std::sync::mpsc::channel();     // serial → GUI
            let (to_serial_tx, from_gui_rx) = std::sync::mpsc::channel();     // GUI → serial

            start_serial_connection(
                settings.serial_path.clone(),
                settings.baudrate,
                to_gui_tx,
                from_gui_rx,
                command_status
            );

            settings.serial_rx = Some(from_serial_rx);
            settings.serial_tx = Some(to_serial_tx);
            settings.serial_connected = true;
        }
    } else {
        ui.label("Connected");
        if ui.button("Get Status").clicked() {
            
        }
    }
}

pub enum CommandStatus {
    Idle,
    Waiting,
}
pub enum MachineCommand {
    StringCommand(String),
}


pub fn start_serial_connection(
    serial_path: String,
    baudrate: u32,
    output_tx: Sender<String>, // from serial to GUI
    command_rx: Receiver<MachineCommand>, // from GUI to serial
    command_status: Arc<Mutex<CommandStatus>>
) {
    task::spawn_blocking(move || {
        let mut port = SerialPort::open(&serial_path, baudrate).expect("Failed to open serial port");
        let mut buf = [0u8; 1024];
        let mut line_buf = String::new();

        loop {
            // check if there are any commands to send
            if let Ok(msg) = command_rx.try_recv() {
                match msg {
                    MachineCommand::StringCommand(cmd) => {
                        let _ = port.write_all(format!("{}\n", cmd).as_bytes());
                        println!("Command sent!");

                        // update the command status
                        let mut command_status = command_status.lock().unwrap();
                        *command_status = CommandStatus::Waiting;
                    }
                }
            }

            // Read incoming serial data
            match port.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let chunk = String::from_utf8_lossy(&buf[..n]);
                    for ch in chunk.chars() {
                        if ch == '\r' {
                            // If the line ends with \r, process it
                            let line = line_buf.trim();
                            if line == "ok" {
                                println!("Ok!");
                                // update the command status
                                let mut command_status = command_status.lock().unwrap();
                                *command_status = CommandStatus::Idle;
                            }
                            line_buf.clear();
                        } else if ch != '\n' {
                            // Append anything that's not a newline
                            line_buf.push(ch);
                        }
                    }
                }
                _ => {
                    // timeout or no data
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }
    });
}

