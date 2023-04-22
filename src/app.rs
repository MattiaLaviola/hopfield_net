mod central_panel;
mod side_panel;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct HopfiledNetsApp {

    #[serde(skip)]
    central_panel : central_panel::CentralPanel,
    #[serde(skip)]
    side_panel : side_panel::SidePanel,
    dummy_state: Vec<f64>,
}

impl Default for HopfiledNetsApp {
   
    fn default() -> Self {
        let side_panel = side_panel::SidePanel::new();
        let dummy_state = vec![1.0; side_panel.get_state_size()];
        Self {
            central_panel : central_panel::CentralPanel::new(),
            side_panel : side_panel,
            dummy_state: dummy_state,
        }
       
    }
}

impl HopfiledNetsApp{

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for HopfiledNetsApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let Self { label, node_size_continer, central_panel } = self;

        //------------------------------Updating the UI components------------------------------
        if self.side_panel.has_node_dim_changed() {
            self.central_panel.set_node_size(self.side_panel.get_node_dim());
        }

        if self.side_panel.has_state_size_changed() {
            self.dummy_state = vec![1.0; self.side_panel.get_state_size()];
            self.dummy_state[0] = -1.0;
        }



        //----------------------------------Renderin the UI----------------------------------
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            self.side_panel.generate_ui(ui);
        });

        egui::CentralPanel::default().show(ctx,|ui| {
            //Test values
            let vec_size = self.side_panel.get_state_size();
            let mut vec = vec![1.0;vec_size];
            vec[0] = -1.0;
            //----------------
            self.central_panel.generate_ui(ui, &self.dummy_state);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }


      
    }
}
