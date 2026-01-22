use std::fmt::Debug;
use std::hash::Hash;

use egui::{Vec2, Widget};

use crate::{
    cable::{Cable, CableId},
    custom_widget::CustomWidget,
    default_port::DefaultPort,
    id::Id,
    plug::{DraggedPlug, Plug},
    port_params::PortParams,
    response::ResponseExt,
    state::State,
};

pub type PortId = Id;

#[derive(Debug)]
pub struct Port {
    port_id: PortId,
    widget: Option<CustomWidget>,
    drag_cable: Option<CableId>,
}

impl Port {
    pub fn new<T: Hash + Eq + Debug + Send + Sync + 'static>(port_id: T) -> Self {
        Port {
            port_id: PortId::new(port_id),
            widget: None,
            drag_cable: None,
        }
    }

    pub fn widget(mut self, widget: impl Into<CustomWidget>) -> Self {
        self.widget = Some(widget.into());
        self
    }

    // make this port create a cable with given ID when dragged from
    // while the cable is still being dragged, it is rendered with the port
    // when the cable is dropped, TODO
    // use the same ID as you pass this function to render the cable after it is dropped
    pub fn drag_create_cable(mut self, id: CableId) -> Self {
        self.drag_cable = Some(id);
        self
    }
}

impl Widget for Port {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui
            .push_id(self.port_id.clone(), |ui| {
                // This widget is not need to use egui::Area

                let mut state = State::get_cloned(ui);

                // Render port with params
                PortParams {
                    hovered: state.hovered_port_id() == Some(self.port_id.clone()),
                }
                .set(ui);
                let response = self.widget.unwrap_or_else(|| DefaultPort.into()).ui(ui);

                // advance generation if this port is rendered twice
                state.advance_generation_if_twice(self.port_id.clone());
                // update port's position used for plug rendering
                state.update_port_pos(self.port_id.clone(), response.rect.left_top());

                let dragged_plug = state.dragged_plug().unwrap_or(DraggedPlug {
                    pos: egui::pos2(-100.0, -100.0), // far
                    size: Vec2::ZERO,
                });

                // distance between the port and the dragged plug
                let distance_sq = response.rect.center().distance_sq(dragged_plug.pos);
                let min_length = |vec: Vec2| vec.x.min(vec.y);
                let close_distance =
                    (min_length(response.rect.size()) + min_length(dragged_plug.size)) / 2.0;

                // distance required because `response.hovered()` always returns false when plug is interacted
                let hovered = response.hovered() || distance_sq < close_distance.powi(2);

                // update hovered port id used for cable connection
                if hovered {
                    state.update_hovered_port_id(self.port_id.clone());
                }

                // finally update the state
                state.store_to(ui);

                response
            })
            .inner;

        if let Some(drag_cable_id) = self.drag_cable {
            if response.dragged() {
                ui.add(
                    Cable::new(drag_cable_id, Plug::to(self.port_id), Plug::unplugged())
                        .forward_drag_response(&response),
                );
            } else if response.drag_stopped() {
                let mut dragged_cable_response = ui.add(
                    Cable::new(drag_cable_id, Plug::to(self.port_id), Plug::unplugged())
                        .forward_drag_response(&response),
                );

                let mut state = State::get_cloned(ui);
                state.ephemeral.dropped_cable_plug.insert(
                    response.id,
                    dragged_cable_response.out_plug().connected_to(),
                );
                state.store_to(ui);
            }
        }

        response
    }
}
