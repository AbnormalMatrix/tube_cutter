use nannou::prelude::*;
use nannou_egui::{self, egui::{self, Align2, Color32, Pos2, RichText}, Egui};

mod gcode;


fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    resolution: u32,
    scale: f32,
    rotation: f32,
    color: Srgb<u8>,
    position: Vec2,

    gc: gcode::Gcode,
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

    Model {
        egui,
        settings: Settings {
            resolution: 10,
            scale: 200.0,
            rotation: 0.0,
            color: WHITE,
            position: vec2(0.0, 0.0),

            gc: gcode::Gcode::new(),
        },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    

    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        ui.label("Resolution:");
        ui.add(egui::Slider::new(&mut settings.resolution, 1..=40));

        // Scale slider
        ui.label("Scale:");
        ui.add(egui::Slider::new(&mut settings.scale, 0.0..=1000.0));

        // Rotation slider
        ui.label("Rotation:");
        ui.add(egui::Slider::new(&mut settings.rotation, 0.0..=360.0));

        // Random color button
        let clicked = ui.button("Random color").clicked();

        if clicked {
            settings.color = rgb(random(), random(), random());
        }



        if ui.button(RichText::new("Set Positioning to Absolute").color(Color32::WHITE).size(14.0)).clicked() {
            settings.gc.set_positioning_mode(gcode::PositioningMode::Absolute);
        }

        ui.horizontal(|ui| {
            ui.label("hi");
            ui.label("there");
        });

        if ui.button("Write Gcode").clicked() {
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

    egui::Window::new("Gcode Preview")
        .fixed_pos(Pos2::new(ctx.available_rect().right() - 10.0, ctx.available_rect().top() + 10.0))
        .pivot(Align2::RIGHT_TOP).show(&ctx, |ui| {
        ui.code(RichText::code((&settings.gc.gcode_string).into()).color(Color32::WHITE));
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

    let rotation_radians = deg_to_rad(settings.rotation);
    draw.ellipse()
        .resolution(settings.resolution as f32)
        .xy(settings.position)
        .color(settings.color)
        .rotate(-rotation_radians)
        .radius(settings.scale);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}