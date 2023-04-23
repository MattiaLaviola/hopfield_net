// pub mod of all the modules to make the compiler happy
pub mod central_panel;
pub mod side_panel;
pub mod hop_net;
pub mod utilities;
pub mod thread_utils;

// Actually used stuff
use hop_net::Net as NetTrait;
use hop_net::classic_network::ClassicNetworkDiscrete;
use hop_net::NetworkCommand;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct HopfiledNetsApp {
    #[serde(skip)]
    central_panel: central_panel::CentralPanel,
    #[serde(skip)]
    side_panel: side_panel::SidePanel,

    #[serde(skip)]
    send_to_net: mpsc::Sender<NetworkCommand>,
    #[serde(skip)]
    recieve_from_net: mpsc::Receiver<Vec<f64>>,

    #[serde(skip)]
    // This attirbute is not really necessary, but it makes life a little simpler.
    net_stepping: bool,

    #[serde(skip)]
    saved_state: Vec<f64>,
}

impl Default for HopfiledNetsApp {
    fn default() -> Self {
        let (main_send, net_recieve) = mpsc::channel::<NetworkCommand>();
        let (net_send, main_recieve) = mpsc::channel::<Vec<f64>>();

        let state_size = 9;
        let start_state = vec![-1.0; state_size*state_size];
        let std_net_type = hop_net::NetworkType::SquareDiscrete;

        let side_panel = side_panel::SidePanel::new(std_net_type, state_size);
        let std_stepping_speed = side_panel.get_stepping_speed() as f32;

        let start_state_clone = start_state.clone();
        thread::spawn(move || {
            let mut sleep_time = Duration::from_millis((1000.0 / std_stepping_speed) as u64);
            let mut net = ClassicNetworkDiscrete::new(
                start_state_clone.len(),
                Some(&start_state_clone)
            );
            let mut is_stepping = false;
            println!("Net thread up");
            loop {
                let mess = thread_utils::get_message(&net_recieve, is_stepping);

                // If there are no messagges, mess will be NetworkCommand::None, so
                // we can get Option::None only if the channel has been close, and in that case we return.
                if !mess.is_some() {
                    println!("Net thread closed");
                    return;
                }
                println!("is_stepping: {:?} mess obtained: {:?}\n", is_stepping, mess);

                let mess = mess.unwrap();
                if mess != NetworkCommand::None {
                    // This methid reads the command coming from the main therad, and  updates the net, the stepping speed,
                    // and if the net is stepping accordingly.
                    thread_utils::handle_message(&mut net, mess, &mut is_stepping, &mut sleep_time);
                }

                if is_stepping {
                    // The net computes the next state, and than returns a copy to be sent to the main thread.
                    net_send.send(net.step());
                    thread::sleep(sleep_time);
                }
            }
        });

        Self {
            central_panel: central_panel::CentralPanel::new(std_net_type, &start_state),
            side_panel: side_panel,
            send_to_net: main_send,
            recieve_from_net: main_recieve,
            net_stepping: false,
            saved_state: start_state,
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
            let side_p = &mut self.side_panel;
            let centr_p = &mut self.central_panel;
            let stepping_speed = side_p.get_stepping_speed();

            // We check to see if the net has generated a new states only if we know that the net is genrating.
            if self.net_stepping {
                // If the network is generating new states faster then we are cousuming, we drop the extra states.
                let mut mess = self.recieve_from_net.try_iter().last();
                if mess.is_some() {
                    centr_p.set_net_state(mess.unwrap());
                }
            }

            if side_p.has_state_size_changed() {
                self.net_stepping = false;
                let mut new_state = vec![-1.0; side_p.get_state_size()];
                self.saved_state = new_state.clone();
                self.send_to_net.send(NetworkCommand::Stop);
                self.send_to_net.send(NetworkCommand::SetState(new_state.clone()));
                centr_p.set_net_state(new_state);
            }

            // We save what the user is seeing (it may be different from what the network actually is)
            if side_p.save_current_state() {
                self.saved_state = centr_p.get_net_state();
            }

            if side_p.load_saved_state() {
                self.net_stepping = false;
                self.send_to_net.send(NetworkCommand::Stop);
                self.send_to_net.send(NetworkCommand::SetState(self.saved_state.clone()));
                centr_p.set_net_state(self.saved_state.clone());
            }

            if side_p.has_selected_network_changed() {
                centr_p.set_net_type(side_p.get_net_type());
            }

            //If the user cahnged the nodes dimention through the slider, we update the gui.
            if side_p.has_node_dim_changed() {
                centr_p.set_node_size(side_p.get_node_dim());
            }

            // If the user editd the network state through the gui, we update the network.
            // The right way to do this is to save just the indices of the nodes that have changed, and then update only them
            // I'll rework this part for sure
            if centr_p.has_net_state_changed() {
                self.send_to_net.send(NetworkCommand::SetState(centr_p.get_net_state()));
            }

            if side_p.stop_stepping_pressed() {
                self.net_stepping = false;
                self.send_to_net.send(NetworkCommand::Stop);
            }

            if side_p.start_stepping_pressed() {
                self.net_stepping = true;
                self.send_to_net.send(NetworkCommand::Go);
            }

            if side_p.has_stepping_speed_changed() {
                self.send_to_net.send(NetworkCommand::SetSpeed(side_p.get_stepping_speed()));
            }

            // We learn what the user is seeing
            if side_p.learn_current_state() {
                self.send_to_net.send(NetworkCommand::Learn(centr_p.get_net_state()));
            }
        }

        //----------------------------------Rendering the UI----------------------------------
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