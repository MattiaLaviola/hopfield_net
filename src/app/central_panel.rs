pub struct CentralPanel {
    button_size: egui::Vec2,
    net_state: Vec<f64>,
    just_changed: Vec<bool>,
    saved_state: Vec<f64>,
    net_state_changed: bool,
    state_sqrt: usize,
    singledrag: bool,
    mouse_down: bool,
}

impl CentralPanel {
    pub fn new(net_state: Vec<f64>) -> Self {
        let sqrt = (net_state.len() as f32).sqrt() as usize;
        Self {
            net_state_changed: false,
            singledrag: false,
            button_size: egui::vec2(20.0, 20.0),
            just_changed: vec![false; net_state.len()],
            net_state: net_state.clone(),
            state_sqrt: sqrt,
            saved_state: net_state,
            mouse_down: false,
        }
    }

    pub fn generate_ui<'a>(&mut self, ui: &'a mut egui::Ui) {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.heading("Network state");

        let mut mouse_pos = egui::Pos2::new(0.0, 0.0);
        ui.ctx().input(|i| {
            if i.pointer.primary_pressed() {
                self.mouse_down = true;
            }

            if i.pointer.primary_released() {
                self.mouse_down = false;
                for i in 0..self.just_changed.len() {
                    self.just_changed[i] = false;
                }
            }

            if i.pointer.hover_pos().is_some() {
                mouse_pos = i.pointer.hover_pos().unwrap();
            }
        });
        egui::Grid
            ::new("central_panel_grid_0")
            .spacing(egui::vec2(3.0, 3.0))
            .min_col_width(0.0)
            .min_row_height(0.0)
            .show(ui, |ui| {
                for i in 0..self.net_state.len() {
                    let mut button = egui::Button
                        ::new(" ")
                        .sense(egui::Sense::click())
                        .min_size(self.button_size);

                    if self.net_state[i] == -1.0 {
                        button = button.fill(egui::Color32::from_rgb(0, 0, 0));
                    } else {
                        button = button.fill(egui::Color32::from_rgb(255, 255, 255));
                    }

                    let response = ui.add(button);
                    // If the mouse is over the button, and the mouse is pressed, invert it's state

                    if
                        response.rect.contains(mouse_pos) &&
                        self.mouse_down &&
                        !self.just_changed[i]
                    {
                        self.net_state_changed = true;
                        self.just_changed[i] = true;
                        self.net_state[i] = -self.net_state[i];
                    }
                    if (i + 1) % self.state_sqrt == 0 {
                        ui.end_row();
                    }
                }
            });

        // egui::warn_if_debug_build(ui);
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
        self.state_sqrt = (net_state.len() as f32).sqrt() as usize;
        self.net_state = net_state.clone();
        self.saved_state = net_state;
        self.net_state_changed = false;
    }

    pub fn load_saved_state(&mut self) {
        self.net_state = self.saved_state.clone();
    }

    pub fn save_current_state(&mut self) {
        self.saved_state = self.net_state.clone();
    }
}