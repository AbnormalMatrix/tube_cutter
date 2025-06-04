use crossbeam_channel::{Sender, Receiver};
use serial2::SerialPort;
use std::{collections::LinkedList, task, thread::spawn};

// struct to store everything related to the serial connection to the machine
#[flutter_rust_bridge::frb(opaque)]
pub struct MachineConnection{
    serial_port: String,
    baudrate: u32,
    serial_tx: Option<Sender<MachineCommand>>,
    serial_rx: Option<Receiver<String>>,
}

impl MachineConnection {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new() -> Self {
        Self { serial_port: "/dev/ttyUSB0".to_string(), baudrate: 115200, serial_tx: None, serial_rx: None }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn set_serial_port(&mut self, new_port: String) {
        self.serial_port = new_port;
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn send_string_command(&self, command: String) {
        if self.serial_tx.is_some() {
            let command = MachineCommand::StringCommand(command);
            self.serial_tx.as_ref().unwrap().send(command);
        }

    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn send_string_command_low_priority(&self, command: String) {
        if self.serial_tx.is_some() {
            let command = MachineCommand::StringCommandLowPriority(command);
            self.serial_tx.as_ref().unwrap().send(command);
        }

    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn send_gcode_command(&self, command: String) {
        if self.serial_tx.is_some() {
            let command = MachineCommand::GcodeCommand(command);
            self.serial_tx.as_ref().unwrap().send(command);
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn home(&self) {
        self.send_string_command("G1 X0 Y0 F1000".to_string());
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn make_connection(&mut self) {
        if self.serial_rx.is_none() {
            let (to_gui_tx, from_machine_rx) = crossbeam_channel::unbounded();
            
            self.serial_rx = Some(from_machine_rx);

            let (to_machine_tx, from_gui_rx) = crossbeam_channel::unbounded();

            self.serial_tx = Some(to_machine_tx);

            let serial_path = self.serial_port.clone();
            let serial_baudrate = self.baudrate.clone();

            // a linked list to store the buffered commands (the commands that are waiting to be sent)
            let mut buffered_commands = LinkedList::new();
            
            // a flag if the current command has been completed
            let mut command_completed = true;

            spawn(move || {

                let mut port = SerialPort::open(serial_path, serial_baudrate).unwrap();
                let mut buf = [0u8; 1024];
                let mut line_buf = String::new();

                loop {
                    // check if the gui sent any commands
                    if let Ok(msg) = from_gui_rx.try_recv() {
                        match msg {
                            MachineCommand::StringCommand(cmd) => {
                                buffered_commands.push_back(cmd);
                            },
                            MachineCommand::StringCommandLowPriority(cmd) => {
                                if buffered_commands.len() == 0 {
                                    buffered_commands.push_back(cmd);
                                }
                            },
                            MachineCommand::GcodeCommand(cmd) => {
                                for c in cmd.split("\n") {
                                    buffered_commands.push_back(c.to_string());
                                }
                            }
                        }
                    }

                    // check if there are any commands in buffered_commands to send to the machine
                    if buffered_commands.len() > 0 && command_completed {
                        let cmd = buffered_commands.pop_front().unwrap();
                        println!("{}", cmd);
                        let _ = port.write_all(format!("{}\n", cmd).as_bytes());

                        command_completed = false;
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
                                        command_completed = true;

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
                                if buffered_commands.is_empty() {
                                    std::thread::sleep(std::time::Duration::from_millis(1));
                                }
                        }
                    }
                }
            });

        }
    }
}

#[flutter_rust_bridge::frb(opaque)]
pub enum MachineCommand {
    StringCommand(String),
    StringCommandLowPriority(String),
    // just a bunch of commands together
    GcodeCommand(String),
}