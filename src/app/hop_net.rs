// This file is here because otherwise the rust copiler doesn't corrctly compute the module tree
// there is probably a better way to do this, but at least for the moment this is good enough
// the problems probably originates from me oranizing the fils in a java-like fashion
pub mod classic_network;
pub mod storkey_learning;

use std::fmt::Display;
use std::fmt::Formatter;

use rand::prelude::SliceRandom;
use strum_macros::EnumIter;

// ---------------------------------Start of Net trait---------------------------------
pub trait Net<T> {
    fn get_state(&self) -> Vec<T>;

    fn learn(&mut self, state: &[T]);

    fn step(&mut self) -> (bool, Vec<T>);

    fn get_steps(&self) -> usize;

    fn set_state(&mut self, state: &[T]);

    fn reset_weights(&mut self);

    fn get_weights(&self) -> Vec<Vec<T>>;
}

// ---------------------------------Start of Network Type---------------------------------
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

// ---------------------------------Comuincation Enums---------------------------------
#[derive(PartialEq, Clone)]
pub enum NetworkCommand {
    None,
    Learn(Vec<f64>),
    Go,
    Stop,
    SetState(Vec<f64>),
    SetSpeed(u64),
    ResetWeights,
    //This command contais the type of net to setup, and its starting state,stored in a tuple
    ChangeNetType(NetworkType),
}

impl std::fmt::Debug for NetworkCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkCommand::None => write!(f, "None"),
            NetworkCommand::Learn(state) => write!(f, "Learn(\n{})", state_vec_to_string(&state)),
            NetworkCommand::Go => write!(f, "Go"),
            NetworkCommand::Stop => write!(f, "Stop"),
            NetworkCommand::SetState(state) => {
                write!(f, "SetState(\n{})", state_vec_to_string(&state))
            }
            NetworkCommand::SetSpeed(speed) => write!(f, "SetSpeed({})", speed),
            NetworkCommand::ResetWeights => write!(f, "ResetWeights"),
            NetworkCommand::ChangeNetType(net_type) => write!(f, "ChangeNetType({:?})", net_type),
        }
    }
}

#[derive(Debug)]
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

// ---------------------------------Utility Functions---------------------------------
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

fn reset_nodes_to_update(container: &mut Vec<usize>, lenght: usize) {
    // If the containere isn't already empty, we empty it
    while !container.is_empty() {
        container.clear();
    }

    // The container is populated with the indices of the nodes
    for i in 0..lenght {
        container.push(i);
    }

    container.shuffle(&mut rand::thread_rng());
}
