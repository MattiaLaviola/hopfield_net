// This file is here because otherwise the rust copiler doesn't corrctly compute the module tree
// there is probably a better way to do this, but at least for the moment this is good enough
// the problems probably originates from me oranizing the fils in a java-like fashion
pub mod classic_network;
use strum_macros::EnumIter;

pub trait Net<T> {
    fn get_state(&self) -> Vec<T>;

    fn learn(&mut self, state: &Vec<T>);

    fn step(&mut self) -> Vec<T>;

    fn set_state(&mut self, state: &Vec<T>);

    fn reset_weights(&mut self);
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum NetworkType {
    StorkeySquareDiscrete,
    SquareDiscrete,
}

impl NetworkType {
    pub fn to_string(&self) -> String {
        match self {
            NetworkType::StorkeySquareDiscrete => "StorkeySquareDiscrete".to_string(),
            NetworkType::SquareDiscrete => "HebbianSquareDiscrete".to_string(),
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
    //This command contais the type of net to setup, and its starting state,stored in a tuple
    ChangeNetType((NetworkType, Vec<f64>)),
}

pub enum NetworkResponse {
    NewState(Vec<f64>),
    Ack,
}