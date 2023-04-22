use strum::IntoEnumIterator;
use strum_macros::EnumIter; 
#[path = "./utilities.rs"]
mod utilites;

#[derive( EnumIter, Debug,PartialEq,Clone,Copy)]
pub enum NetworkType {
    Classic,
    Storkey,
}

impl NetworkType{
    pub fn to_string(&self) -> String {
        match self {
            NetworkType::Classic => "Discrete, hebbian".to_string(),
            NetworkType::Storkey => "Storkey".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "Discrete, hebbian" => NetworkType::Classic,
            "Storkey" => NetworkType::Storkey,
            _ => panic!("Unknown network type"),
        }
    }
}

pub struct SidePanel {
    node_dim : utilites::EditableValue<f32>,
    reset: bool,
    network: utilites::EditableValue<NetworkType>,
    state_size: utilites::EditableValue<usize>,
}

impl SidePanel {
    pub fn new() -> Self {
        Self {
            node_dim: utilites::EditableValue::new(20.0),
            reset: false,
            network: utilites::EditableValue::new(NetworkType::Classic),
            state_size: utilites::EditableValue::new(10),
        }
    }

    pub fn generate_ui<'a> (&mut self, ui: &'a mut egui::Ui) {
         //let Self { node_dim,network, ..} = self;

        let std_space = 5.0;
        ui.heading("Settings");
        ui.add_space(std_space);
       
        
        ui.label("Node size:");
        let response = ui.add(egui::Slider::new( &mut self.node_dim.value, 20.0..=100.0).text("value"));
        self.node_dim.changed =  response.dragged();

        let response = ui.add(egui::Button::new("Reset"));
        self.reset = response.clicked();


        ui.add_space(std_space);
        ui.label("Select network type:");
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("").selected_text(self.network.value.to_string()).show_ui(ui, |ui| {
                for network_type in NetworkType::iter() {
                    ui.selectable_value(&mut self.network.value, network_type, network_type.to_string());
                }
            });
            
            let response = ui.button("Apply");
            self.network.changed = response.clicked();
        });

        ui.add_space(std_space);

        ui.label("State size:");

        ui.horizontal(|ui| {
            let mut text = self.state_size.value.to_string();
            ui.text_edit_singleline(&mut text);
            self.state_size.value = text.parse().unwrap();
            let response = ui.button("Apply");
            self.state_size.changed = response.clicked();

        });
        /*
        let response = ui.add(egui::Slider::new( &mut self.state_size.value, 2..=20).text(""));
        self.state_size.changed =  response.dragged();
        */

    }

    // Getters

    pub fn get_node_dim(&self) -> f32 {
        self.node_dim.value
    }

    pub fn has_node_dim_changed(&self) -> bool {
        self.node_dim.changed
    }

    pub fn get_selected_network(&self) -> NetworkType {
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
}