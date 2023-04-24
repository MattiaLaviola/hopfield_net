// This file is here because otherwise the rust copiler doesn't corrctly compute the module tree
// there is probably a better way to do this, but at least for the moment this is good enough
// the problems probably originates from me oranizing the fils in a java-like fashion
pub mod classic_network;
use std::fmt::Display;
use std::fmt::Formatter;
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
        match self {
            NetworkResponse::None => false,
            _ => true,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            NetworkResponse::None => true,
            _ => false,
        }
    }

    pub fn unwrap(self) -> Vec<f64> {
        match self {
            NetworkResponse::NewState(state) => state,
            _ => panic!("Tried to unwrap a NetworkResponse::None"),
        }
    }
}
