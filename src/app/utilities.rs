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