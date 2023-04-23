use crate::app::utilities;
use strum::IntoEnumIterator;

pub struct SidePanel {
    reset: bool,
    save_current_state: bool,
    node_dim: utilities::EditableValue<f32>,
    network: utilities::EditableValue<utilities::NetworkType>,
    state_size: utilities::EditableValue<usize>,
    text_holder: String,
    learn_current_state: bool,
    forget_all: bool,
    remember: bool,
    remember_speed: utilities::EditableValue<f32>,
}

impl SidePanel {
    pub fn new(network_type: utilities::NetworkType, state_size: usize) -> Self {
        Self {
            node_dim: utilities::EditableValue::new(20.0),
            reset: false,
            network: utilities::EditableValue::new(network_type),
            state_size: utilities::EditableValue::new(state_size),
            text_holder: state_size.to_string(),
            save_current_state: false,
            learn_current_state: false,
            forget_all: false,
            remember: false,
            remember_speed: utilities::EditableValue::new(10.0),
        }
    }

    pub fn generate_ui<'a>(&mut self, ui: &'a mut egui::Ui) {
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
            egui::ComboBox
                ::from_label("")
                .selected_text(self.network.value.to_string())
                .show_ui(ui, |ui| {
                    for network_type in utilities::NetworkType::iter() {
                        ui.selectable_value(
                            &mut self.network.value,
                            network_type,
                            network_type.to_string()
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
            let response = ui.button("Start");
            if response.clicked() {
                self.remember = true;
            }
            let response = ui.button("Stop");
            if response.clicked() {
                self.remember = false;
            }
        });
        let response = ui.add(
            egui::Slider::new(&mut self.remember_speed.value, 1.0..=600.0).text("step/sec")
        );
        self.remember_speed.changed = response.dragged();
        // End of learning section

        ui.add_space(std_space);

        // Start of node size selection
        ui.label("Node size:");
        let response = ui.add(
            egui::Slider::new(&mut self.node_dim.value, 1.0..=100.0).text("value")
        );
        self.node_dim.changed = response.dragged();

        // End of node size selection

        ui.add_space(std_space / 2.0);

        // Start of state size selection
        ui.label("State size:");
        ui.horizontal(|ui| {
            let text_edit_singleline = egui::TextEdit
                ::singleline(&mut self.text_holder)
                .desired_width(0.0)
                .min_size((10.0, 0.0).into());

            ui.text_edit_singleline(&mut self.text_holder);

            let response = ui.button("Apply");
            // The state_size.changed is restored to false every frame, if something had to change, we assume it already did
            self.state_size.changed = false;
            if response.clicked() {
                let num = self.text_holder.parse::<usize>();
                if num.is_ok() {
                    let num = num.unwrap();
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

    pub fn get_remember_speed(&self) -> f32 {
        self.remember_speed.value
    }

    pub fn is_remembering_speed_changed(&self) -> bool {
        self.remember_speed.changed
    }

    pub fn is_remembering(&self) -> bool {
        self.remember
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

    pub fn get_net_type(&self) -> utilities::NetworkType {
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
}