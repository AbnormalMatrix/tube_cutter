use std::sync::mpsc::{Receiver, Sender};
use tokio::task;

use nannou_egui::egui;
use serial2::SerialPort;

use crate::Settings;

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

pub fn make_connection_button(ui: &mut egui::Ui, settings: &mut Settings) {
    if settings.serial_rx.is_none() {
        if ui.button("Connect").clicked() {
            let (to_gui_tx, from_serial_rx) = std::sync::mpsc::channel();     // serial → GUI
            let (to_serial_tx, from_gui_rx) = std::sync::mpsc::channel();     // GUI → serial

            start_serial_connection(
                settings.serial_path.clone(),
                settings.baudrate,
                to_gui_tx,
                from_gui_rx,
            );

            settings.serial_rx = Some(from_serial_rx);
            settings.serial_tx = Some(to_serial_tx);
            settings.serial_connected = true;
        }
    } else {
        ui.label("Connected");
    }
}


pub fn start_serial_connection(
    serial_path: String,
    baudrate: u32,
    output_tx: Sender<String>, // from serial to GUI
    input_rx: Receiver<String>, // from GUI to serial
) {
    task::spawn_blocking(move || {
        let mut port = SerialPort::open(&serial_path, baudrate).expect("Failed to open serial port");
        let mut buf = [0u8; 1024];

        loop {
            // Write if there's an input message
            if let Ok(msg) = input_rx.try_recv() {
                let _ = port.write_all(msg.as_bytes());
            }

            // Read incoming serial data
            match port.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let s = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = output_tx.send(s);
                }
                _ => std::thread::sleep(std::time::Duration::from_millis(10)),
            }
        }
    });
}

pub fn send_serial_message(settings: &mut Settings, message: String) {
    if let Some(tx) = &settings.serial_tx {
        let _ = tx.send(message);
    }
}