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

            // add a port that you can create cables from
            ui.add_space(150.0);
            let response = ui.add(Port::new(6usize));
            if response.drag_started() {
                self.connected.push((Some(6usize), None));
            }

            fn helper(port_index: &Option<usize>) -> Plug {
                match port_index {
                    Some(index) => Plug::to(index.clone()),
                    None => Plug::unplugged(),
                }
            }

            let num_cables = self.connected.len();

            // draw existing cables
            for (index, (from, to)) in self.connected.iter_mut().enumerate() {
                let mut my_cable = Cable::new(index, helper(from), helper(to));
                if response.drag_started() && (index + 1) == num_cables {
                    my_cable = my_cable.forward_drag(
                        response.drag_delta()
                            + (response.interact_pointer_pos().unwrap() - response.rect.center())
                            - vec2(50.0, 0.0) // cable.rs line 102
                            - vec2(8.0, 8.0) // half of utils.rs SIZE
                            - vec2(2.0, 2.0) // extra offset to get the plug where I want it
                            - (ui.next_widget_position() - response.rect.center()),
                    )
                } else if response.dragged() && (index + 1) == num_cables {
                    my_cable = my_cable.forward_drag(response.drag_delta());
                }
                if response.drag_stopped() && (index + 1) == num_cables {
                    my_cable = my_cable.forward_drag_stop();
                }
                let mut response = ui.add(my_cable);

                if let Some(new_from) = response.in_plug().connected_to() {
                    *from = Some(new_from.downcast_ref::<usize>().unwrap().clone());
                }
                if let Some(new_to) = response.out_plug().connected_to() {
                    *to = Some(new_to.downcast_ref::<usize>().unwrap().clone());
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
