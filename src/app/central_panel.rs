pub struct CentralPanel {
    button_size: egui::Vec2,
}

impl CentralPanel {
    pub fn new() -> Self {
        Self {
            button_size: egui::vec2(20.0, 20.0),
        }
    }

    pub fn set_node_size(&mut self, size:f32){
        if size < 0.0 {
            return;
        }
        self.button_size = egui::vec2(size, size);
    
    }

    pub fn generate_ui<'a> (&self, ui: &'a mut egui::Ui, net_state: &Vec<f64>) {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.heading("Network state");
        let sqrt = (net_state.len()as f32).sqrt() as usize;
        if sqrt * sqrt != net_state.len() {
            ui.label("The network state is not a square matrix");
            return;
        }else{
            egui::Grid::new("central_panel_grid_0")
            .spacing(egui::vec2(3.0, 3.0))
            .min_col_width(0.0)
            .min_row_height(0.0)
            .show(ui, |ui| {
                for i in 0..net_state.len() {
                    let mut button = egui::Button::new(" ")
                    .min_size(self.button_size);

                    if net_state[i] == 1.0 {

                      button = button.fill(egui::Color32::from_rgb(0, 0, 0));
                    }else{

                       button = button.fill(egui::Color32::from_rgb(255, 255, 255));
                    }

                    ui.add(button);
                    if (i + 1) % sqrt == 0 {
                        ui.end_row();
                    }
                }
            });
        }
        
        
       // egui::warn_if_debug_build(ui);
  }
}