use rand::Rng;
use rand::prelude::SliceRandom;

use crate::app::hop_net;
pub struct ClassicNetworkDiscrete {
    pub state: Vec<f64>,
    pub rng: rand::rngs::ThreadRng,
    weights: Vec<Vec<f64>>,
    number_of_learned_states: f64,
    steps: usize,
    nodes_yet_to_update: Vec<usize>,
}

// The network will mostly be interacted with trough this traits
impl hop_net::Net<f64> for ClassicNetworkDiscrete {
    fn get_state(&self) -> Vec<f64> {
        self.state.clone()
    }

    fn learn(&mut self, state: &[f64]) {
        self.number_of_learned_states += 1.0;
        self.hebbian_learning(state);
    }

    fn step(&mut self) -> Vec<f64> {
        //generat e random index from 0 to state.len()
        //let i = self.rng.gen_range(0..self.state.len());
        if self.nodes_yet_to_update.len() <= 0 {
            ClassicNetworkDiscrete::reset_nodes_to_update(
                &mut self.nodes_yet_to_update,
                self.state.len()
            );
        }
        let i = self.nodes_yet_to_update.pop().unwrap();
        self.update_node(i);
        self.steps += 1;
        self.state.clone()
    }

    fn set_state(&mut self, state: &[f64]) {
        if state.len() < 4 {
            panic!("State is too short");
        }

        if self.state.len() != state.len() {
            self.number_of_learned_states = 0.0;
            self.weights = vec![vec![0.4; state.len()]; state.len()];
            self.steps = 0;
        }
        self.state = state.to_vec();

        // The starting state has just been set, so we are 0 steps away from it
        self.steps = 0;

        // We make sure that all nodes are marked as "to update"
        ClassicNetworkDiscrete::reset_nodes_to_update(
            &mut self.nodes_yet_to_update,
            self.state.len()
        );
    }

    fn reset_weights(&mut self) {
        self.weights = vec![vec![0.4; self.state.len()]; self.state.len()];
        self.number_of_learned_states = 0.0;
    }
}

impl ClassicNetworkDiscrete {
    pub fn new(size: usize, start_state: Option<&Vec<f64>>) -> ClassicNetworkDiscrete {
        let state = if start_state.is_none() {
            Vec::with_capacity(size)
        } else {
            let mut start_s = start_state.unwrap();
            if start_s.len() != size {
                panic!("Size and start size lenght are differnt");
            }
            start_s.clone()
        };

        let mut nodes_to_update = Vec::with_capacity(size);
        ClassicNetworkDiscrete::reset_nodes_to_update(&mut nodes_to_update, size);

        ClassicNetworkDiscrete {
            state,
            rng: rand::thread_rng(),
            weights: vec![vec![0.4; size]; size],
            steps: 0,
            number_of_learned_states: 0.0,
            nodes_yet_to_update: nodes_to_update,
        }
    }

    pub fn init(&mut self, state: Option<&Vec<f64>>) {
        if state.is_some() {
            self.state = state.unwrap().clone();
        } else {
            for i in 0..self.state.len() {
                self.state[i] = if self.rng.gen_range(0..=1) == 1 { 1.0 } else { -1.0 };
            }
        }
        // self.setWeightsToRandom();
        self.steps = 0;
    }

    fn hebbian_learning(&mut self, state_to_learn: &[f64]) {
        if self.number_of_learned_states == 1.0 {
            for i in 0..self.weights.len() {
                for j in 0..self.weights[i].len() {
                    if i == j {
                        self.weights[i][j] = 0.0;
                    } else {
                        self.weights[i][j] = state_to_learn[i] * state_to_learn[j];
                    }
                }
            }
        } else {
            for i in 0..self.weights.len() {
                for j in 0..self.weights[i].len() {
                    if i == j {
                        self.weights[i][j] = 0.0;
                    } else {
                        // self.weights[i][j] +=(1.0 / self.number_of_learned_states) *(state_to_learn[i] * state_to_learn[j]) + ((self.number_of_learned_states - 1.0) /self.number_of_learned_states) self.weights[i][j];
                        self.weights[i][j] += state_to_learn[i] * state_to_learn[j];
                    }
                }
            }
        }
    }

    // This function summs the weight coming into a node, and then takes the sign of the sum
    // This is the standard upadate rule for the classic hopfield networks
    fn update_node(&mut self, i: usize) {
        let mut sum = 0.0;
        for j in 0..self.weights[i].len() {
            sum += self.weights[i][j] * self.state[j];
        }
        self.state[i] = if sum > 0.0 { 1.0 } else { -1.0 };
    }

    // May be cool to we wich state are memorized in the random matrix
    fn set_weights_to_random(&mut self) {
        for i in 0..self.weights.len() {
            for j in 0..self.weights[i].len() {
                self.weights[i][j] = self.rng.gen_range(-5.0..=5.0);
            }
        }
    }

    // It is higly possible that this funcitin will be moved to net_utils
    fn reset_nodes_to_update(container: &mut Vec<usize>, lenght: usize) {
        // If the containere isn't already empty, we empty it
        while !container.is_empty() {
            container.pop();
        }

        // The container is populated with the indices of the nodes
        for i in 0..lenght {
            container.push(i);
        }

        container.shuffle(&mut rand::thread_rng());
    }

    // Getters

    pub fn get_steps(&self) -> usize {
        self.steps
    }
}

impl std::fmt::Display for ClassicNetworkDiscrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //chek to see if the state lenght is greater then 0
        if self.state.len() == 0 {
            return write!(f, "state: the network has size 0");
        }

        let mut state = String::new();

        // Extract the closest integer square root
        let sqrt = (self.state.len() as f64).sqrt().round() as usize;

        //chesk if the lenght of the state is the square of a number
        if sqrt.pow(2) == self.state.len() {
            for i in 0..sqrt {
                for j in 0..sqrt {
                    if self.state[i * sqrt + j] == 1.0 {
                        state.push('◼');
                    } else {
                        state.push('◻');
                    }
                }
                state.push('\n');
            }
        } else {
            for i in 0..self.state.len() {
                if self.state[i] == 1.0 {
                    state.push('◼');
                } else {
                    state.push('◻');
                }
            }
        }

        write!(f, "state:\n{}", state)
    }
}