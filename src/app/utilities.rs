use strum_macros::EnumIter;

pub struct EditableValue<T> {
    pub value: T,
    pub changed: bool,
}

impl<T> EditableValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            changed: false,
        }
    }
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