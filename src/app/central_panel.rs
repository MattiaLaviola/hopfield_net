mod state_renderer;

use crate::app::hop_net;

pub struct CentralPanel {
    button_size: egui::Vec2,
    net_state: Vec<f64>,
    just_changed: Vec<bool>,
    saved_state: Vec<f64>,
    net_state_changed: bool,
    state_sqrt: usize,
    mouse_down: bool,
    network_type: hop_net::NetworkType,
    nodes_being_edited: bool,
}

impl CentralPanel {
    pub fn new(network_type: hop_net::NetworkType, net_state: &Vec<f64>) -> Self {
        let sqrt = (net_state.len() as f32).sqrt() as usize;
        Self {
            net_state_changed: false,
            button_size: egui::vec2(20.0, 20.0),
            just_changed: vec![false; net_state.len()],
            net_state: net_state.clone(),
            state_sqrt: sqrt,
            saved_state: net_state.clone(),
            mouse_down: false,
            network_type,
            nodes_being_edited: false,
        }
    }

    pub fn generate_ui(&mut self, ui: &mut egui::Ui) {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.heading("Network state");

        // Here we extrat the mouse position,and if the mouse primary button is pressed form the context
        // when the mouse is released we also reset the just_cahnged mask to all false
        let mouse_pos = self.handle_mouse(ui);

        // More than a single net uses the same renderer, so we store the call in a closure to improve redability
        let mut square_descrete_render = || {
            state_renderer::render_square_discrete(
                ui,
                &mut self.net_state,
                &mut self.just_changed,
                &mut self.net_state_changed,
                self.button_size,
                mouse_pos,
                self.mouse_down,
                &mut self.nodes_being_edited
            )
        };

        match self.network_type {
            hop_net::NetworkType::StorkeySquareDiscrete => square_descrete_render(),
            hop_net::NetworkType::SquareDiscrete => square_descrete_render(),
            _ => panic!("Renderer not available"),
        }

        // egui::warn_if_debug_build(ui);
    }

    fn handle_mouse(&mut self, ui: &mut egui::Ui) -> egui::Pos2 {
        let mut mouse_pos = egui::Pos2::new(0.0, 0.0);
        ui.ctx().input(|i| {
            if i.pointer.primary_pressed() {
                self.mouse_down = true;
            }

            // primary_released is global and true even when the user is not interacing with this panel
            // so, to not waste time looping over and over through just_changed, we make sure that the nodes are being edited
            if i.pointer.primary_released() {
                self.mouse_down = false;
                if self.nodes_being_edited {
                    for i in 0..self.just_changed.len() {
                        // Just looping througt all the nodes si not the smartest way, but this
                        // shuold happen quite rarely
                        self.just_changed[i] = false;
                    }
                    // Since we have just resetted the just_changed mask, we dont run this code until new edits are made
                    self.nodes_being_edited = false;
                }
            }

            if i.pointer.hover_pos().is_some() {
                mouse_pos = i.pointer.hover_pos().unwrap();
            }
        });
        mouse_pos
    }

    // Getters

    pub fn get_net_state(&self) -> Vec<f64> {
        self.net_state.clone()
    }

    pub fn has_net_state_changed(&self) -> bool {
        self.net_state_changed
    }

    // Setters

    pub fn set_node_size(&mut self, size: f32) {
        if size < 0.0 {
            return;
        }
        self.button_size = egui::vec2(size, size);
    }

    pub fn set_net_state(&mut self, net_state: Vec<f64>) {
        if net_state.len() != self.net_state.len() {
            self.just_changed = vec![false; net_state.len()];
            self.state_sqrt = (net_state.len() as f32).sqrt() as usize;
            self.net_state_changed = false;
        }

        self.net_state = net_state;
    }

    pub fn set_net_type(&mut self, network_type: hop_net::NetworkType) {
        self.network_type = network_type;
    }
}