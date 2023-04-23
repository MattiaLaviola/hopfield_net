pub mod central_panel;
pub mod side_panel;
pub mod utilities;
mod networks;
use networks::net_utils::Net as NetTrait;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct HopfiledNetsApp {
    #[serde(skip)]
    central_panel: central_panel::CentralPanel,
    #[serde(skip)]
    side_panel: side_panel::SidePanel,

    #[serde(skip)]
    current_net: Box<dyn NetTrait<f64>>,
}

impl Default for HopfiledNetsApp {
    fn default() -> Self {
        let state_size = 9;
        let mut start_state = vec![-1.0; state_size*state_size];
        let std_net_type = utilities::NetworkType::SquareDiscrete;
        let net = Box::new(
            networks::classic_network::ClassicNetworkDiscrete::new(state_size, Some(&start_state))
        );

        Self {
            central_panel: central_panel::CentralPanel::new(std_net_type, start_state),
            side_panel: side_panel::SidePanel::new(std_net_type, state_size),
            current_net: net,
        }
    }
}

impl HopfiledNetsApp {
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
        {
            // Creatig this variables to make the code less verbose
            let mut side_p = &mut self.side_panel;
            let mut centr_p = &mut self.central_panel;
            if side_p.has_node_dim_changed() {
                centr_p.set_node_size(side_p.get_node_dim());
            }

            if side_p.has_state_size_changed() {
                let mut new_state = vec![-1.0; side_p.get_state_size()];
                new_state[0] = 1.0;
                centr_p.set_net_state(new_state);
            }

            if side_p.save_current_state() {
                centr_p.save_current_state();
            }

            if side_p.load_saved_state() {
                centr_p.load_saved_state();
            }

            if side_p.has_selected_network_changed() {
                centr_p.set_net_type(side_p.get_net_type());
            }
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

        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel.generate_ui(ui);
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