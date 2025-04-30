use eframe::egui;
use egui::{pos2, Rect, Sense, Vec2};
use egui_cable::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|ctx| {
            ctx.egui_ctx.set_theme(egui::Theme::Light);
            Ok(Box::new(MyEguiApp::new()))
        }),
    )
    .expect("Failed to start native application");
}

#[derive(Default)]
struct MyEguiApp {
    cables: Vec<(i32, i32)>,
}

impl MyEguiApp {
    pub fn new() -> Self {
        Self {
            cables: vec![(0, 2), (0, 3), (1, 4)],
        }
    }
}

/// Custom cable with no handle, that can be moved without activating
#[derive(Debug)]
struct CustomCable;

impl egui::Widget for CustomCable {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let params = CableParams::get(ui);
        ui.painter().add(params.bezier);

        // we need a response to return. There might be a better way of getting this?
        let (_, mut response) = ui.allocate_at_least(Vec2::splat(0.0), Sense::hover());
        // pretend the cable control (that we don't have) has been clicked, so the cable is considered activated
        response.flags.set(egui::response::Flags::FAKE_PRIMARY_CLICKED, true);
        // the interact_rect is EVERYTHING so that no clicks are considered outside (which deactivates the cable)
        response.interact_rect = Rect::EVERYTHING;
        return response;
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("My window")
            .default_pos(pos2(20.0, 100.0))
            .show(ctx, |ui| {
                ui.add(Port::new(0));
                ui.add_space(10.0);
                ui.add(Port::new(1));
            });
        egui::Window::new("My window 2")
            .default_pos(pos2(200.0, 20.0))
            .show(ctx, |ui| {
                ui.add(Port::new(2));
                ui.add_space(10.0);
                ui.add(Port::new(3));
            });
        egui::Window::new("My window 3")
            .default_pos(pos2(200.0, 200.0))
            .show(ctx, |ui| {
                ui.add(Port::new(4));
                ui.add_space(10.0);
                ui.add(Port::new(5));

                for (index, (from, to)) in self.cables.iter_mut().enumerate() {
                    let mut response = ui.add(
                        Cable::new(index, Plug::to(from.clone()), Plug::to(to.clone()))
                            .widget(CustomCable),
                    );

                    if let Some(new_from) = response.in_plug().connected_to() {
                        *from = new_from.downcast_ref::<i32>().unwrap().clone();
                    }
                    if let Some(new_to) = response.out_plug().connected_to() {
                        *to = new_to.downcast_ref::<i32>().unwrap().clone();
                    }

                    // not checking for disconnected cables, so when you drop a cable, it gets put back
                    // you could handle it here if you wanted to allow floating plugs
                }
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::WHITE.to_array()
    }
}
