use crate::app::hop_net;
use crate::app::hop_net::classic_network;
use crate::app::hop_net::storkey_learning;
use crate::app::hop_net::Net;
use crate::app::hop_net::NetworkCommand;
use crate::app::hop_net::NetworkType;
use crate::app::NetworkResponse;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub fn get_message(
    channel: &Receiver<NetworkCommand>,
    is_stepping: bool,
) -> Option<NetworkCommand> {
    if is_stepping {
        // If the network is stepping, we do not block the thread waiting for new messages
        match channel.try_recv() {
            Ok(mess) => Some(mess),
            Err(err) => match err {
                mpsc::TryRecvError::Empty => Some(NetworkCommand::None),
                mpsc::TryRecvError::Disconnected => None,
            },
        }
    } else {
        // If the network is not stepping, letting the thread run makes no sense, so we wait
        match channel.recv() {
            Ok(mess) => Some(mess),
            Err(_) => None,
        }
    }
}

pub fn handle_message(
    net: &mut Box<dyn Net<f64>>,
    command: NetworkCommand,
    is_stepping: &mut bool,
    old_step_num: &mut usize,
    stepping_speed: &mut Duration,
) -> bool {
    match command {
        NetworkCommand::None => {}

        NetworkCommand::Learn(vec) => {
            net.learn(&vec);
            println!("{}", hop_net::state_vec_to_string(&vec));
        }

        NetworkCommand::Go => {
            *is_stepping = true;
            *old_step_num = net.get_steps();
        }

        NetworkCommand::Stop => {
            *is_stepping = false;
        }

        NetworkCommand::SetState(vec) => {
            net.set_state(&vec);
            *old_step_num = net.get_steps();
        }

        NetworkCommand::SetSpeed(speed) => {
            *stepping_speed = Duration::from_millis(1000 / speed);
        }

        NetworkCommand::ResetWeights => {
            net.reset_weights();
        }

        NetworkCommand::ChangeNetType(new_type) => {
            let size = net.get_state().len();
            *net = match new_type {
                NetworkType::SquareDiscrete => {
                    Box::new(classic_network::ClassicNetworkDiscrete::new(size, None))
                }
                NetworkType::StorkeySquareDiscrete => {
                    Box::new(storkey_learning::StorkeyLearningNetwork::new(size, None))
                }
            };
            return true;
        }

        _ => println!("An unimplemented command was recieved"),
    }
    false
}

pub fn start_net_thread(
    net_type: NetworkType,
    start_state: Vec<f64>,
    step_speed: usize,
    net_send: Sender<NetworkResponse>,
    net_recieve: Receiver<NetworkCommand>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        println!("Net thread up");

        let std_err_fn = || {
            panic!("Net thread closed unexpectedly");
        };

        // -----------------------------Setup-----------------------------
        let mut net: Box<dyn Net<f64>> = match net_type {
            NetworkType::SquareDiscrete => Box::new(classic_network::ClassicNetworkDiscrete::new(
                start_state.len(),
                Some(&start_state),
            )),
            _ => {
                panic!("Not implemented");
            }
        };

        let mut sleep_time = Duration::from_millis((1000.0 / step_speed as f64) as u64);
        let mut is_stepping = false;
        let mut old_step_num = 0;
        let max_steps_without_change = net.get_state().len() + 1;

        // -----------------------------Main loop-----------------------------
        loop {
            let mess = get_message(&net_recieve, is_stepping);

            // get_message returns None only if the channel is closed, which would meann that the main thread stopped
            if mess.is_none() {
                println!("Net thread closed");
                return;
            }

            let mess = mess.unwrap();
            if mess != NetworkCommand::None {
                let net_state_changed = handle_message(
                    &mut net,
                    mess,
                    &mut is_stepping,
                    &mut old_step_num,
                    &mut sleep_time,
                );

                if net_state_changed {
                    let update = NetworkResponse::NewState(net.get_state().clone());
                    if net_send.send(update).is_err() {
                        std_err_fn();
                    }
                }
            }

            if is_stepping {
                // The net computes the next state, and than returns a copy to be sent to the main thread, it also computes
                // if the new state is equal to the old one.
                let (state_changed, new_state) = net.step();

                if state_changed {
                    old_step_num = net.get_steps();
                    if net_send.send(NetworkResponse::NewState(new_state)).is_err() {
                        std_err_fn();
                    }
                } else {
                    // We assume that is possible for the state to not change after a single step.
                    // But if after x steps it still has not changed, we assue that we have reached an equilibrium state.
                    let diff = net.get_steps() - old_step_num;
                    if diff >= max_steps_without_change {
                        println!("Stoppped stepping");
                        is_stepping = false;
                        if net_send.send(NetworkResponse::Stopped).is_err() {
                            std_err_fn();
                        }
                    } else if net_send.send(NetworkResponse::None).is_err() {
                        std_err_fn();
                    }
                }

                std::thread::sleep(sleep_time);
            }
        }
    })
}
