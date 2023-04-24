// This file is here because otherwise the rust copiler doesn't corrctly compute the module tree
// there is probably a better way to do this, but at least for the moment this is good enough
// the problems probably originates from me oranizing the fils in a java-like fashion
pub mod classic_network;
use crate::app::thread_utils;
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Duration;
use strum_macros::EnumIter;

pub trait Net<T> {
    fn get_state(&self) -> Vec<T>;

    fn learn(&mut self, state: &[T]);

    fn step(&mut self) -> (bool, Vec<T>);

    fn get_steps(&self) -> usize;

    fn set_state(&mut self, state: &[T]);

    fn reset_weights(&mut self);
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum NetworkType {
    StorkeySquareDiscrete,
    SquareDiscrete,
}

impl Display for NetworkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkType::StorkeySquareDiscrete => write!(f, "StorkeySquareDiscrete"),
            NetworkType::SquareDiscrete => write!(f, "HebbianSquareDiscrete"),
            _ => panic!("Unknown network type"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NetworkCommand {
    None,
    Learn(Vec<f64>),
    Go,
    Stop,
    SetState(Vec<f64>),
    SetSpeed(u64),
    ResetWeights,
    //This command contais the type of net to setup, and its starting state,stored in a tuple
    ChangeNetType((NetworkType, Vec<f64>)),
}

pub enum NetworkResponse {
    NewState(Vec<f64>),
    Stopped,
    None,
}

impl NetworkResponse {
    pub fn is_some(&self) -> bool {
        !matches!(self, NetworkResponse::None)
    }

    pub fn is_none(&self) -> bool {
        matches!(self, NetworkResponse::None)
    }

    pub fn unwrap(self) -> Vec<f64> {
        match self {
            NetworkResponse::NewState(state) => state,
            _ => panic!("Tried to unwrap a NetworkResponse::None"),
        }
    }
}

pub fn state_vec_to_string(state: &[f64]) -> String {
    let mut result = String::new();

    // Extract the closest integer square root
    let sqrt = (state.len() as f64).sqrt().round() as usize;

    //chesk if the lenght of the state is the square of a number
    if sqrt.pow(2) == state.len() {
        for i in 0..sqrt {
            for j in 0..sqrt {
                if state[i * sqrt + j] == 1.0 {
                    result.push('◼');
                } else {
                    result.push('◻');
                }
            }
            result.push('\n');
        }
    } else {
        for node in state {
            if *node == 1.0 {
                result.push('◼');
            } else {
                result.push('◻');
            }
        }
    }

    result
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
