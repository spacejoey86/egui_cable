use eframe::egui;
use egui::vec2;
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

            let num_cables = self.connected.len();

            fn helper(port_index: &Option<usize>) -> Plug {
                match port_index {
                    Some(index) => Plug::to(index.clone()),
                    None => Plug::unplugged(),
                }
            }
            // ui.add_space(40.0);
            // ui.dnd_drag_source("only_drag_source".into(), 1, |dnd_ui| {
            //     dnd_ui.add(Port::new(9usize));
            // });
            // ui.scope_builder(egui::UiBuilder::new(), add_contents);

            // draw existing cables
            for (index, (from, to)) in self.connected.iter_mut().enumerate() {
                let mut my_cable = Cable::new(index, helper(from), helper(to));
                // if index + 1 == num_cables {
                //     my_cable = my_cable.forward_drag_response(&port_response);
                // }
                let mut cable_response = ui.add(my_cable);

                if let Some(new_from) = cable_response.in_plug().connected_to() {
                    *from = Some(new_from.downcast_ref::<usize>().unwrap().clone());
                }
                if let Some(new_to) = cable_response.out_plug().connected_to() {
                    *to = Some(new_to.downcast_ref::<usize>().unwrap().clone());
                }

                // not checking for disconnected cables, so when you drop a cable, it gets put back
                // you could handle it here if you wanted to allow floating plugs
            }

            // add a port that you can create cables from
            ui.add_space(150.0);
            let port_response = ui.add(Port::new(6usize));
            if port_response.dragged() {
                let dragged_cable_response = ui.add(
                    Cable::new(num_cables, helper(&Some(6)), helper(&None))
                        .forward_drag_response(&port_response),
                );
            }
            if port_response.drag_stopped() {
                let mut dragged_cable_response = ui.add(
                    Cable::new(num_cables, helper(&Some(6)), helper(&None))
                        .forward_drag_response(&port_response),
                );
                self.connected.push((
                    Some(6usize),
                    dragged_cable_response
                        .out_plug()
                        .connected_to()
                        .map(|id| id.downcast_ref::<usize>().unwrap().clone()),
                ));
            }

            ui.add_space(40.0);
            ui.add(Port::new(7usize));
            ui.add_space(40.0);
            ui.add(Port::new(8usize));
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::WHITE.to_array()
    }
}
