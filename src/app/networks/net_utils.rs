pub use net_utils::Net;
mod net_utils {
    pub trait Net<T> {
        fn get_state(&self) -> Vec<T>;

        fn learn(&mut self, state: &mut Vec<T>);

        fn step(&mut self) -> Vec<T>;

        fn set_state(&mut self, state: &mut Vec<T>);

        fn reset_weights(&mut self);
    }
}