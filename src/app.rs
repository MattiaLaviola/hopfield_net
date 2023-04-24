// pub mod of all the modules to make the compiler happy
pub mod central_panel;
pub mod hop_net;
pub mod side_panel;
pub mod thread_utils;
pub mod utilities;

// Actually used stuff
use hop_net::classic_network::ClassicNetworkDiscrete;
use hop_net::Net as NetTrait;
use hop_net::NetworkCommand;
use hop_net::NetworkResponse;
use std::sync::mpsc;
use std::thread;
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
    recieve_from_net: mpsc::Receiver<NetworkResponse>,

    #[serde(skip)]
    // This attirbute is not really necessary, but it makes life a little simpler.
    net_stepping: bool,

    #[serde(skip)]
    saved_state: Vec<f64>,
}

impl Default for HopfiledNetsApp {
    fn default() -> Self {
        let (main_send, net_recieve) = mpsc::channel::<NetworkCommand>();
        let (net_send, main_recieve) = mpsc::channel::<NetworkResponse>();

        let state_size = 9;
        let start_state = vec![-1.0; state_size * state_size];
        let std_net_type = hop_net::NetworkType::SquareDiscrete;

        let side_panel = side_panel::SidePanel::new(std_net_type, state_size);
        let std_stepping_speed = side_panel.get_stepping_speed() as f32;

        let start_state_clone = start_state.clone();
        thread::spawn(move || {
            println!("Net thread up");

            let std_err_fn = || {
                panic!("Net thread closed unexpectedly");
            };

            let mut sleep_time = Duration::from_millis((1000.0 / std_stepping_speed) as u64);
            let mut is_stepping = false;
            let mut old_step_num = 0;
            let mut net =
                ClassicNetworkDiscrete::new(start_state_clone.len(), Some(&start_state_clone));
            let max_steps_without_change = 2 * net.get_state().len();

            // -----------------------------Main loop-----------------------------
            loop {
                let mess = thread_utils::get_message(&net_recieve, is_stepping);

                // get_message returns None only if the channel is closed, which would meann that the main thread stopped
                if mess.is_none() {
                    println!("Net thread closed");
                    return;
                }

                let mess = mess.unwrap();
                if mess != NetworkCommand::None {
                    thread_utils::handle_message(
                        &mut net,
                        mess,
                        &mut is_stepping,
                        &mut old_step_num,
                        &mut sleep_time,
                    );
                }

                if is_stepping {
                    // The net computes the next state, and than returns a copy to be sent to the main thread, it also computes
                    // if the new state is equal to the old one.
                    let (state_changed, new_state) = net.step();

                    if state_changed {
                        old_step_num = net.get_steps();
                        if let Err(_) = net_send.send(NetworkResponse::NewState(new_state)) {
                            std_err_fn();
                        }
                    } else {
                        // We assume that is possible for the state to not change after a single step.
                        // But if after x steps it still has not changed, we assue that we have reached an equilibrium state.
                        let diff = net.get_steps() - old_step_num;
                        println!("Diff: {}", diff);
                        if diff >= max_steps_without_change {
                            println!("Stoppped stepping");
                            is_stepping = false;
                            if let Err(_) = net_send.send(NetworkResponse::Stopped) {
                                std_err_fn();
                            }
                        } else {
                            if let Err(_) = net_send.send(NetworkResponse::None) {
                                std_err_fn();
                            }
                        }
                    }

                    thread::sleep(sleep_time);
                }
            }
        });

        Self {
            central_panel: central_panel::CentralPanel::new(std_net_type, &start_state),
            side_panel,
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

    fn process_net_mss(&self) -> NetworkResponse {
        let mut last_value = self.recieve_from_net.try_recv();
        // We check to see if the channel is still open and if there are new states to render.
        if let Err(e) = last_value {
            if e == mpsc::TryRecvError::Disconnected {
                panic!("Net thread closed unexpectedly");
            } else {
                return NetworkResponse::None;
            }
        }

        let mut new_value = self.recieve_from_net.try_recv();

        // If the network thread is procusing new states faster than we can render them
        // we stat skipping some, we don't want to skip indefinitely, because doing so
        // we risk having the main thread stuck stuck skipping stuff, and killing the responsiveness
        for _i in 0..1_000 {
            if let Err(_) = new_value {
                return last_value.unwrap();
            }
            last_value = self.recieve_from_net.try_recv();
            if let Err(_) = last_value {
                return new_value.unwrap();
            }
            new_value = self.recieve_from_net.try_recv();
        }

        if let Ok(v) = new_value {
            return v;
        } else {
            return last_value.unwrap();
        }
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

        // We check to see if the net has generated a new states only if we know that the net is genrating.
        if self.net_stepping {
            match self.process_net_mss() {
                NetworkResponse::NewState(s) => {
                    self.central_panel.set_net_state(s);
                }
                NetworkResponse::Stopped => {
                    println!("Main thread: net stopped");
                    self.side_panel.set_is_stepping(false);
                }
                _ => {}
            }
        }

        if self.side_panel.has_state_size_changed() {
            self.net_stepping = false;
            let new_state = vec![-1.0; self.side_panel.get_state_size()];
            self.saved_state = new_state.clone();
            if self.send_to_net.send(NetworkCommand::Stop).is_err() {
                println!("Error sending stop command to net");
            }
            if self
                .send_to_net
                .send(NetworkCommand::SetState(new_state.clone()))
                .is_err()
            {
                println!("Error sending set state command to net");
            }
            self.central_panel.set_net_state(new_state);
        }

        // We save what the user is seeing (it may be different from what the network actually is)
        if self.side_panel.save_current_state() {
            self.saved_state = self.central_panel.get_net_state();
        }

        if self.side_panel.load_saved_state() {
            self.net_stepping = false;
            if self.send_to_net.send(NetworkCommand::Stop).is_err() {
                panic!("The network is not running");
            }

            let command = NetworkCommand::SetState(self.saved_state.clone());
            if self.send_to_net.send(command).is_err() {
                panic!("The network is not running");
            }
            self.central_panel.set_net_state(self.saved_state.clone());
        }

        if self.side_panel.has_selected_network_changed() {
            self.central_panel
                .set_net_type(self.side_panel.get_net_type());
        }

        //If the user cahnged the nodes dimention through the slider, we update the gui.
        if self.side_panel.has_node_dim_changed() {
            self.central_panel
                .set_node_size(self.side_panel.get_node_dim());
        }

        // If the user editd the network state through the gui, we update the network.
        // The right way to do this is to save just the indices of the nodes that have changed, and then update only them
        // I'll rework this part for sure
        if self.central_panel.has_net_state_changed() {
            let command = NetworkCommand::SetState(self.central_panel.get_net_state());
            if self.send_to_net.send(command).is_err() {
                panic!("The network is not running");
            }
        }

        if self.side_panel.stop_stepping_pressed() {
            self.net_stepping = false;
            if self.send_to_net.send(NetworkCommand::Stop).is_err() {
                panic!("The network is not running");
            }
        }

        if self.side_panel.start_stepping_pressed() {
            self.net_stepping = true;
            if self.send_to_net.send(NetworkCommand::Go).is_err() {
                panic!("The network is not running");
            }
        }

        if self.side_panel.has_stepping_speed_changed() {
            let command = NetworkCommand::SetSpeed(self.side_panel.get_stepping_speed());
            if self.send_to_net.send(command).is_err() {
                panic!("The network is not running");
            }
        }

        // The current state is always the one being shown to the user, not the one of the net.

        if self.side_panel.learn_current_state() {
            let command = NetworkCommand::Learn(self.central_panel.get_net_state());
            if self.send_to_net.send(command).is_err() {
                panic!("The network is not running");
            }
        }

        if self.side_panel.forget_all() {
            if self.send_to_net.send(NetworkCommand::ResetWeights).is_err() {
                panic!("The network is not running");
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
            ui.label("Hopfield Nets");
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
