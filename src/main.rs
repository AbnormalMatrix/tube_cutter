use std::sync::{mpsc::{Receiver, Sender}, Arc, Mutex};

use gcode::Pos2D;
use nannou::prelude::*;
use nannou_egui::{self, egui::{self, Align2, Color32, DragValue, FontDefinitions, Pos2, RichText, Slider, Visuals}, Egui};
mod phosphor_fonts;


mod gcode;
mod sender;
mod status;


fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    resolution: u32,
    scale: f32,
    rotation: f32,
    color: Srgb<u8>,
    position: Vec2,

    scale_factor: f32,
    start_pos: gcode::Pos2D,
    end_pos: gcode::Pos2D,

    gc: gcode::Gcode,
    units: gcode::DistUnit,
    tube_width: f32,
    cut_angle: f32,
    feedrate: f32,
    pierce_delay: f32,

    cut_right: bool,

    // serial port stuff
    serial_path: String,
    baudrate: u32,
    serial_buffer: String,
    serial_tx: Option<Sender<sender::MachineCommand>>,
    serial_rx: Option<Receiver<String>>,
    serial_connected: bool,

    command_status: Arc<Mutex<sender::CommandStatus>>,

    msg_to_send: String,

    // machine status stuff
    machine_status: Arc<Mutex<status::MachineStatus>>,
}

struct Model {
    settings: Settings,
    egui: Egui,
}

fn model(app: &App) -> Model {
    // Create window
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    let mut fonts = FontDefinitions::default();
    phosphor_fonts::add_to_fonts(&mut fonts, phosphor_fonts::Variant::Regular);
    egui.ctx().set_fonts(fonts);


    Model {
        egui,
        settings: Settings {
            resolution: 10,
            scale: 200.0,
            rotation: 0.0,
            color: WHITE,
            position: vec2(0.0, 0.0),

            scale_factor: 10.0,
            start_pos: Pos2D::new(0.0, 0.0),
            end_pos: Pos2D::new(0.0, 0.0),

            gc: gcode::Gcode::new(),
            units: gcode::DistUnit::Metric,
            tube_width: 25.0,
            cut_angle: 0.0,
            feedrate: 1000.0,
            pierce_delay: 0.5,

            cut_right: true,

            // serial port stuff
            serial_path: "/dev/ttyUSB0".to_owned(),
            baudrate: 115200,
            serial_buffer: String::new(),
            serial_rx: None,
            serial_tx: None,
            serial_connected: false,

            command_status: Arc::new(Mutex::new(sender::CommandStatus::Idle)),

            msg_to_send: String::new(),

            // machine status stuff
            machine_status: Arc::new(Mutex::new(status::MachineStatus::new())),
        },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    let mut visuals = Visuals::light();
    visuals.override_text_color = Some(Color32::WHITE);
    
    let mut update_pos = false;

    egui::Window::new("Settings").show(&ctx, |ui| {

        

        egui::Grid::new("main_grid")
            .num_columns(1)
            .striped(true)
            .spacing([40.0, 4.0])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.radio_value(&mut settings.units, gcode::DistUnit::Metric, "Metric (mm)");
                    ui.radio_value(&mut settings.units, gcode::DistUnit::Imperial, "Imperial (in)");
                });
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Tube Width").color(Color32::WHITE).size(14.0));
                    // check if we are using metric or imperial to properly display this:
                    match settings.units {
                        gcode::DistUnit::Metric => {
                            ui.add(Slider::new(&mut settings.tube_width, 0.0..=100.0).suffix("mm")).changed().then(|| update_pos = true);
                        },
                        gcode::DistUnit::Imperial => {
                            ui.add(Slider::new(&mut settings.tube_width, 0.0..=100.0).suffix("in")).changed().then(|| update_pos = true);
                        },
                    }
                });
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Cut Angle").color(Color32::WHITE).size(14.0));
                    ui.add(Slider::new(&mut settings.cut_angle, -180.0..=180.0).drag_value_speed(5.0).suffix("°")).changed().then(|| update_pos = true);
                });
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Feedrate (mm/min)").color(Color32::WHITE).size(14.0));
                    ui.add(DragValue::new(&mut settings.feedrate));
                });
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Pierce Delay (s)").color(Color32::WHITE).size(14.0));
                    ui.add(DragValue::new(&mut settings.pierce_delay).speed(0.1));
                });
                ui.end_row();

                ui.checkbox(&mut settings.cut_right, "Cut Right").changed().then(|| update_pos = true);
            });
        
        ui.separator();


        if ui.button(RichText::new("Add Cut").color(Color32::WHITE).size(14.0)).clicked() { 
            settings.gc.set_plasma_enabled(true);
            settings.gc.dwell(settings.pierce_delay);
            settings.gc.move_xy(&settings.end_pos, settings.feedrate);
            settings.gc.set_plasma_enabled(false);
        }

        if ui.button(RichText::new(format!("{} Run Job", phosphor_fonts::regular::PLAY_CIRCLE)).color(Color32::WHITE).size(18.0)).clicked() {
            settings.gc.add_command("?".to_string());
            settings.serial_tx.as_ref().unwrap().send(sender::MachineCommand::GcodeCommand(settings.gc.gcode_string.clone()));

        }

        if ui.button("Clear").clicked() {
            settings.gc = gcode::Gcode::new();
        }

        ui.horizontal(|ui| {
            if ui.button("Set Home").clicked() {
                settings.serial_tx.as_ref().unwrap().send(sender::MachineCommand::StringCommand("G10 P0 L20 X0 Y0 Z0".to_owned()));
            }
            if ui.button("Go Home").clicked() {
                settings.serial_tx.as_ref().unwrap().send(sender::MachineCommand::StringCommand("G1 X0 Y0 F1000".to_owned()));
            }
        });

        if ui.button(RichText::new("Write Gcode").color(Color32::WHITE).size(14.0)).clicked() {
            // use rfd to get the filename
            if let Some(path) = rfd::FileDialog::new()
                .set_title("Save Gcode")
                .set_file_name("cut.gcode")
                .save_file()
            {
                settings.gc.write_to_file(path);
            }
        }
    });

    if update_pos {
        let machine_status = settings.machine_status.lock().unwrap();
        settings.start_pos = machine_status.position.clone();
        
        settings.end_pos = gcode::calculate_end_pos(&settings.start_pos, settings.tube_width, settings.cut_angle, 0.0, settings.cut_right);
    }

    egui::Window::new("Gcode Preview")
        .fixed_pos(Pos2::new(ctx.available_rect().right() - 10.0, ctx.available_rect().top() + 10.0))
        .pivot(Align2::RIGHT_TOP).show(&ctx, |ui| {
        ui.code(RichText::code((&settings.gc.gcode_string).into()).color(Color32::WHITE));
    });

    egui::Window::new("Connection")
        .fixed_pos(Pos2::new(ctx.available_rect().right() - 10.0, ctx.available_rect().bottom() - 10.0))
        .pivot(Align2::RIGHT_BOTTOM)
        .show(&ctx, |ui| {
            sender::make_connection_button(ui, settings, Arc::clone(&settings.command_status), Arc::clone(&settings.machine_status));
            if settings.serial_connected {
                if let Some(rx) = &settings.serial_rx {
                    while let Ok(data) = rx.try_recv() {
                        settings.serial_buffer.push_str(&data);
                    }
                }
                ui.label(&settings.serial_buffer);
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut settings.msg_to_send);
                    if ui.button("send").clicked() {
                        settings.serial_tx.as_ref().unwrap().send(sender::MachineCommand::StringCommand(settings.msg_to_send.clone()));
                    }
                    let command_status = settings.command_status.lock().unwrap();
                    match *command_status {
                        sender::CommandStatus::Idle => {},
                        sender::CommandStatus::Waiting => {
                            ui.spinner();
                        }
                    }
                });
                
                if ui.button("status").clicked() {
                    settings.serial_tx.as_ref().unwrap().send(sender::MachineCommand::StringCommand("?".to_string()));
                }

                
                
            }
        });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;

    let draw = app.draw();
    draw.background().color(BLACK);

    draw.rect()
        .width(settings.tube_width * settings.scale_factor)
        .height(app.window_rect().h())
        .color(STEELBLUE);

    // calculate the line position

    let line_x_offset = 0.0 - (settings.tube_width * settings.scale_factor) / 2.0;

    let line_start = pt2((settings.start_pos.x * settings.scale_factor) + line_x_offset, settings.start_pos.y * settings.scale_factor);
    let line_end = pt2((settings.end_pos.x * settings.scale_factor) + line_x_offset, settings.end_pos.y * settings.scale_factor);

    draw.line()
        .start(line_start)
        .end(line_end)
        .weight(4.0)
        .color(RED);

    // draw the toolhead
    let machine_status = settings.machine_status.lock().unwrap();


    let tool_screen_position = machine_status.position.to_screen_space(&settings.tube_width, &settings.scale_factor);

    
    draw.ellipse()
        .w_h(50.0, 50.0)
        .color(GREEN)
        .x_y(tool_screen_position.x, tool_screen_position.y);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}