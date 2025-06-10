use eframe::egui;
use egui::{response, vec2};
use egui_cable::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|ctx| {
            ctx.egui_ctx.set_theme(egui::Theme::Light);
            Ok(Box::new(MyEguiApp::default()))
        }),
    )
    .expect("Failed to start native platform");
}

#[derive(Default)]
struct MyEguiApp {
    connected: Vec<(Option<usize>, Option<usize>)>,
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // draw a load of ports
            ui.add_space(40.0);
            ui.horizontal(|ui| {
                ui.add_space(40.0);
                ui.add(Port::new(0usize));
                ui.add_space(40.0);
                ui.add(Port::new(1usize));
                ui.add_space(40.0);
                ui.add(Port::new(2usize));
            });
            ui.add_space(150.0);
            ui.horizontal(|ui| {
                ui.add_space(40.0);
                ui.add(Port::new(3usize));
                ui.add_space(40.0);
                ui.add(Port::new(4usize));
                ui.add_space(40.0);
                ui.add(Port::new(5usize));
            });

            ui.add_space(150.0);
            ui.add(Port::new(6usize));

            fn helper(port_index: &Option<usize>) -> Plug {
                match port_index {
                    Some(index) => Plug::to(index.clone()),
                    None => Plug::unplugged(),
                }
            }

            // draw existing cables
            for (index, (from, to)) in self.connected.iter_mut().enumerate() {
                let mut response = ui.add(Cable::new(index, helper(from), helper(to)));

                if let Some(new_from) = response.in_plug().connected_to() {
                    *from = Some(new_from.downcast_ref::<usize>().unwrap().clone());
                }
                if let Some(new_to) = response.out_plug().connected_to() {
                    *to = Some(new_to.downcast_ref::<usize>().unwrap().clone());
                }

                // not checking for disconnected cables, so when you drop a cable, it gets put back
                // you could handle it here if you wanted to allow floating plugs
            }

            // draw dragged cable
            if let Some(dragged_cable) = Cable::create_if_drag(self.connected.len(), ui) {
                let mut response = ui.add(dragged_cable);
                if let Some(connected_id) = response.out_plug().connected_to() {
                    self.connected.push((
                        response
                            .in_plug()
                            .connected_to()
                            .map(|id| id.downcast_ref::<usize>().unwrap().clone()),
                        Some(connected_id.downcast_ref::<usize>().unwrap().clone()),
                    ))
                }
            }
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::WHITE.to_array()
    }
}
