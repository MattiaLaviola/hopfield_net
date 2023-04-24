use crate::app::hop_net;
use crate::app::utilities;
use strum::IntoEnumIterator;

pub struct SidePanel {
    reset: bool,
    save_current_state: bool,
    node_dim: utilities::EditableValue<f32>,
    network: utilities::EditableValue<hop_net::NetworkType>,
    state_size: utilities::EditableValue<usize>,
    text_holder: String,
    learn_current_state: bool,
    forget_all: bool,
    start_stepping_pressed: bool,
    stop_stepping_pressed: bool,
    is_stepping: bool,
    remember_speed: utilities::EditableValue<u64>,
}

impl SidePanel {
    pub fn new(network_type: hop_net::NetworkType, state_size: usize) -> Self {
        Self {
            node_dim: utilities::EditableValue::new(20.0),
            reset: false,
            network: utilities::EditableValue::new(network_type),
            state_size: utilities::EditableValue::new(state_size),
            text_holder: state_size.to_string(),
            save_current_state: false,
            learn_current_state: false,
            forget_all: false,
            start_stepping_pressed: false,
            stop_stepping_pressed: false,
            is_stepping: false,
            remember_speed: utilities::EditableValue::new(10),
        }
    }

    pub fn generate_ui(&mut self, ui: &mut egui::Ui) {
        //let Self { node_dim,network, ..} = self;

        let std_space = 15.0;
        ui.heading("Settings");
        ui.add_space(std_space);

        // Start of state reset
        let response = ui.add(egui::Button::new("Reset to starting state"));
        self.reset = response.clicked();
        ui.add_space(std_space / 3.0);
        let response = ui.add(egui::Button::new("Set current state as starting state"));
        self.save_current_state = response.clicked();
        // End of state reset

        ui.add_space(std_space);

        // Start of network type selection
        ui.label("Select network type:");
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("")
                .selected_text(self.network.value.to_string())
                .show_ui(ui, |ui| {
                    for network_type in hop_net::NetworkType::iter() {
                        ui.selectable_value(
                            &mut self.network.value,
                            network_type,
                            network_type.to_string(),
                        );
                    }
                });

            let response = ui.button("Apply");
            self.network.changed = response.clicked();
        });
        // End of network type selection

        ui.add_space(std_space);

        // Start of learning section
        ui.horizontal(|ui| {
            let response = ui.button("Learn current state");
            self.learn_current_state = response.clicked();
            let response = ui.button("Forget all");
            self.forget_all = response.clicked();
        });
        ui.label("Memory recovery");
        ui.horizontal(|ui| {
            let response = ui.selectable_label(!self.is_stepping, "Start");
            self.start_stepping_pressed = response.clicked();
            let response = ui.selectable_label(self.is_stepping, "Stop");
            self.stop_stepping_pressed = response.clicked();

            if self.start_stepping_pressed {
                self.is_stepping = true;
            } else if self.stop_stepping_pressed {
                self.is_stepping = false;
            }
        });
        let response =
            ui.add(egui::Slider::new(&mut self.remember_speed.value, 1..=600).text("step/sec"));
        self.remember_speed.changed = response.dragged();
        // End of learning section

        ui.add_space(std_space);

        // Start of node size selection
        ui.label("Node size:");
        let response =
            ui.add(egui::Slider::new(&mut self.node_dim.value, 1.0..=100.0).text("value"));
        self.node_dim.changed = response.dragged();

        // End of node size selection

        ui.add_space(std_space / 2.0);

        // Start of state size selection
        ui.label("State size:");
        ui.horizontal(|ui| {
            let text_edit_singleline = egui::TextEdit::singleline(&mut self.text_holder)
                .desired_width(50.0)
                .min_size((10.0, 0.0).into());

            ui.add(text_edit_singleline);
            let response = ui.button("Apply");
            // The state_size.changed is restored to false every frame, if something had to change, we assume it already did
            self.state_size.changed = false;
            if response.clicked() {
                let num = self.text_holder.parse::<usize>();
                if let Ok(num) = num {
                    if num > 1 && num < 100 && num != self.state_size.value {
                        self.state_size.value = num;
                        // Since the state size changed, we set the changed flag to true
                        self.state_size.changed = true;
                    }
                }
                // If the user has written random stuff, we reset the text holder to the current value
                self.text_holder = self.state_size.value.to_string();
            }
        });
        // End of state size selection
    }

    // Getters

    pub fn get_stepping_speed(&self) -> u64 {
        self.remember_speed.value
    }

    pub fn has_stepping_speed_changed(&self) -> bool {
        self.remember_speed.changed
    }

    pub fn start_stepping_pressed(&self) -> bool {
        self.start_stepping_pressed
    }

    pub fn stop_stepping_pressed(&self) -> bool {
        self.stop_stepping_pressed
    }

    pub fn learn_current_state(&self) -> bool {
        self.learn_current_state
    }

    pub fn forget_all(&self) -> bool {
        self.forget_all
    }

    pub fn get_node_dim(&self) -> f32 {
        self.node_dim.value
    }

    pub fn has_node_dim_changed(&self) -> bool {
        self.node_dim.changed
    }

    pub fn get_selected_network(&self) -> hop_net::NetworkType {
        self.network.value
    }

    pub fn has_selected_network_changed(&self) -> bool {
        self.network.changed
    }

    pub fn get_state_size(&self) -> usize {
        self.state_size.value.pow(2)
    }

    pub fn has_state_size_changed(&self) -> bool {
        self.state_size.changed
    }

    pub fn save_current_state(&self) -> bool {
        self.save_current_state
    }

    pub fn load_saved_state(&self) -> bool {
        self.reset
    }

    // Setters

    pub fn set_is_stepping(&mut self, is_stepping: bool) {
        self.is_stepping = is_stepping;
    }
}
